use ipc_if::greet::{GreetArgs, GreetResponse};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(args: GreetArgs) -> GreetResponse {
    let message = format!("Hello, {}! You've been greeted from Rust!", args.name);

    GreetResponse { message }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
