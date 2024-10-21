use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "random-example.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RandomExampleArgs {
    pub request_id: String,
}

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "random-example.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RandomExampleResponse {
    pub message: String,
    pub response_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "random-example.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RandomExampleError {
    pub error_message: String,
}

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "random-example.ts", rename_all = "snake_case")]
pub enum RandomExampleChannelName {
    RandomExample,
}
