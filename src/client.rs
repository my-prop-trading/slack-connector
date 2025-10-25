use flurl::body::FlUrlBody;
use serde::Serialize;

pub struct SlackApiClient {}

impl SlackApiClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_webhook<TReq>(&self, url: &str, req: &TReq) -> Result<(), String>
    where
        TReq: Serialize,
    {
        let flurl = flurl::FlUrl::new(url);
        let body = FlUrlBody::as_json(req);
        let result = flurl.post(body).await;

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {} {:?}",
                url,
                result.unwrap_err()
            )
            .into());
        };

        let status_code = resp.get_status_code();
        let result = resp.receive_body().await;

        let Ok(body_bytes) = result else {
            return Err(format!("Failed receive_body: {:?}", result.unwrap_err()).into());
        };

        let body_str = String::from_utf8(body_bytes).unwrap();
        if status_code < 300 {
            Ok(())
        } else {
            Err(format!("Failed HTTP {}: {}", url, body_str))
        }
    }
}
