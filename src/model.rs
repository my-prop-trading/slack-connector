use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WebhookResponse {
    pub ok: bool,
    pub error: Option<String>,
}
