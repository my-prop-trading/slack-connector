use flurl::body::FlUrlBody;
use serde::Serialize;

use crate::model::WebhookResponse;

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
        let result: Result<WebhookResponse, _> = serde_json::from_str(&body_str);

        let Ok(resp) = result else {
            let msg = format!(
                "Failed to deserialize: {:?}. Url: {:?} Body: {}",
                result, url, body_str
            );
            return Err(msg.into());
        };

        if !resp.ok {
            return Err(format!("Failed HTTP response: {} {}", url, body_str));
        }

        if status_code < 300 {
            Ok(())
        } else {
            Err(format!(
                "Failed HTTP status code {} {}: {}",
                status_code, url, body_str
            ))
        }
    }
}
