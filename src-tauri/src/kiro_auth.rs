use tauri::AppHandle;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

use crate::{log_to_file, log_token_summary, parse_token_response, save_token_data, TokenData};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct KiroLoginConfig {
    pub auth_url: String,
    pub username: String,
    pub password: String,
}

#[tauri::command]
pub async fn login_kiro(_app: AppHandle, config: KiroLoginConfig) -> Result<TokenData, String> {
    log_to_file("========== KIRO LOGIN STARTED ==========");

    if config.auth_url.is_empty() || config.username.is_empty() || config.password.is_empty() {
        log_to_file("ERROR: Kiro auth_url, username or password is empty");
        return Err("Kiro login URL, username va password la bat buoc".to_string());
    }

    let client = reqwest::Client::new();
    let credentials = BASE64_STANDARD.encode(format!("{}:{}", config.username, config.password));

    log_to_file(&format!("Calling Kiro login endpoint: {}", config.auth_url));

    let res = client
        .get(&config.auth_url)
        .header("Authorization", format!("Basic {}", credentials))
        .send()
        .await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Network error during Kiro login: {}", e));
            format!("Network error during Kiro login: {}", e)
        })?;

    if !res.status().is_success() {
        let status = res.status();
        let err_text = res.text().await.unwrap_or_default();
        let short_error: String = err_text.chars().take(400).collect();
        log_to_file(&format!("ERROR: Kiro login failed ({})", status));
        return Err(format!("Kiro login failed ({}): {}", status, short_error));
    }

    let response_text = res.text().await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to read Kiro login response: {}", e));
            format!("Failed to read Kiro login response: {}", e)
        })?;

    let token_data = parse_token_response(&response_text)
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to parse Kiro login response: {}", e));
            e
        })?;

    log_token_summary(&token_data);
    save_token_data(&token_data)?;
    log_to_file("========== KIRO LOGIN COMPLETED SUCCESSFULLY ==========\n");

    Ok(token_data)
}
