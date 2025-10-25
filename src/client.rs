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

        let result = resp.receive_body().await;

        let Ok(body_bytes) = result else {
            return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
        };

        let body_str = String::from_utf8(body_bytes).unwrap();
        if body_str.contains("OK") {
            Ok(())
        } else {
            Err(format!("HTTP {}: {}", url, body_str))
        }
    }
}
