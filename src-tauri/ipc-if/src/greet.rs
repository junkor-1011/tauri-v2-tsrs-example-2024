use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "greet.ts")]
pub struct Args {
    pub name: String,
}

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "greet.ts")]
pub struct Response {
    pub message: String,
}

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "greet.ts", rename_all = "lowercase")]
pub enum Channel {
    Greet,
}
