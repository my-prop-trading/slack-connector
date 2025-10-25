use slack_connector::client::SlackApiClient;

#[tokio::main]
async fn main() {
    let url = std::env::var("SLACK_URL").unwrap();
    let client = SlackApiClient::new();
    let req = serde_json::json!({
        "notification_id": "test-notification-id",
        "trigger_name": "test-trigger-name"
    });
    let res = client.send_webhook(&url, &req).await;

    println!("send_webhook: {:?}", res);
    println!("Run basic example: FINISHED");
}
