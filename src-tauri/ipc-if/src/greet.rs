use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "greet.ts")]
pub struct GreetArgs {
    pub name: String,
}

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "greet.ts")]
pub struct GreetResponse {
    pub message: String,
}

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "greet.ts", rename_all = "snake_case")]
pub enum GreetChannelName {
    Greet,
}
