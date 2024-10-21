use ipc_if::{
    greet::{GreetArgs, GreetResponse},
    random_example::{RandomExampleArgs, RandomExampleError, RandomExampleResponse},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(args: GreetArgs) -> GreetResponse {
    let message = format!("Hello, {}! You've been greeted from Rust!", args.name);

    GreetResponse { message }
}

#[tauri::command]
fn random_example(args: RandomExampleArgs) -> Result<RandomExampleResponse, RandomExampleError> {
    println!("request_id: {}", args.request_id);

    if rand::random() {
        let res = RandomExampleResponse {
            message: format!("[{}]success.", args.request_id),
            response_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        };
        Ok(res)
    } else {
        let err = RandomExampleError {
            error_message: format!("[${}]failed.", args.request_id),
        };
        Err(err)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, random_example,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
