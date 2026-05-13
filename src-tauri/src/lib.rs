mod core;
mod providers;

pub use core::token::TokenData;
use providers::kiro::{login_with_kiro, KiroLoginConfig};

#[tauri::command]
async fn login_kiro(config: KiroLoginConfig) -> Result<TokenData, String> {
    login_with_kiro(config).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login_kiro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
