use prompt_core::config::{Chat, PromptConfig};
use tauri::{Error, Manager, State};

struct AppData {
    welcome_message: &'static str,
    prompt_config: Box<PromptConfig>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_chats(state: State<AppData>) -> Result<Vec<Chat>, Error> {
    let chats = state.prompt_config.list_chats()?;

    Ok(chats)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let config = PromptConfig::from_prompt_home(app.path().home_dir()?)?;

            app.manage(AppData {
                welcome_message: "Welcome to Tauri!",
                prompt_config: Box::new(config),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_chats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
