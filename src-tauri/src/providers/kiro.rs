use crate::core::log::log_to_file;
use crate::core::storage::save_token_data;
use crate::core::token::{parse_token_response, TokenData};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

const PROVIDER_NAME: &str = "Kiro";

#[derive(Debug, Deserialize)]
pub struct KiroLoginConfig {
    pub auth_url: String,
    pub username: String,
    pub password: String,
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default = "default_credential_mode")]
    pub credential_mode: String,
}

fn default_method() -> String {
    "GET".to_string()
}

fn default_credential_mode() -> String {
    "basic".to_string()
}

fn validate_config(config: &KiroLoginConfig) -> Result<(), String> {
    if config.auth_url.trim().is_empty() {
        return Err("Kiro login URL la bat buoc.".to_string());
    }

    let parsed = url::Url::parse(config.auth_url.trim())
        .map_err(|error| format!("Kiro login URL khong hop le: {}", error))?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err("Kiro login URL phai dung http hoac https.".to_string());
    }

    if config.username.trim().is_empty() || config.password.is_empty() {
        return Err("Tai khoan va mat khau la bat buoc.".to_string());
    }

    let method = config.method.trim().to_ascii_uppercase();
    if method != "GET" && method != "POST" {
        return Err("Request method chi ho tro GET hoac POST.".to_string());
    }

    let mode = config.credential_mode.trim().to_ascii_lowercase();
    if mode != "basic" && mode != "json" {
        return Err("Credential mode chi ho tro basic hoac json.".to_string());
    }

    Ok(())
}

pub async fn login_with_kiro(config: KiroLoginConfig) -> Result<TokenData, String> {
    log_to_file("========== KIRO LOGIN STARTED ==========");
    validate_config(&config)?;

    let method = config.method.trim().to_ascii_uppercase();
    let mode = config.credential_mode.trim().to_ascii_lowercase();
    let client = reqwest::Client::builder()
        .user_agent("XLab-RefreshToken/1.0")
        .build()
        .map_err(|error| format!("Khong tao duoc HTTP client: {}", error))?;

    log_to_file(&format!(
        "Calling Kiro login endpoint: {} ({}, {})",
        config.auth_url, method, mode
    ));

    let mut request = if method == "POST" {
        client.post(config.auth_url.trim())
    } else {
        client.get(config.auth_url.trim())
    }
    .header(ACCEPT, "application/json");

    if mode == "json" {
        request = request
            .header(CONTENT_TYPE, "application/json")
            .json(&json!({
                "username": config.username.trim(),
                "password": config.password,
            }));
    } else {
        let credentials = BASE64_STANDARD.encode(format!("{}:{}", config.username.trim(), config.password));
        request = request.header(AUTHORIZATION, format!("Basic {}", credentials));
    }

    let response = request
        .send()
        .await
        .map_err(|error| format!("Loi network khi login Kiro: {}", error))?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|error| format!("Khong doc duoc response Kiro: {}", error))?;

    if !status.is_success() {
        let short_error: String = response_text.chars().take(500).collect();
        log_to_file(&format!("ERROR: Kiro login failed ({})", status));
        return Err(format!("Kiro login failed ({}): {}", status, short_error));
    }

    let mut token_data = parse_token_response(PROVIDER_NAME, &response_text)?;
    let saved_path = save_token_data(&token_data)?;
    token_data.saved_path = Some(saved_path.display().to_string());

    log_to_file(&format!(
        "Token parsed: access_token={}, refresh_token={}",
        if token_data.access_token.is_empty() { "missing" } else { "present" },
        if token_data.refresh_token.is_some() { "present" } else { "missing" }
    ));
    log_to_file("========== KIRO LOGIN COMPLETED ==========");

    Ok(token_data)
}
