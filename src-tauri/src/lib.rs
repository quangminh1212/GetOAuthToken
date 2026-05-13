use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

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

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub timestamp: String,
    pub saved_path: Option<String>,
}

fn default_method() -> String {
    "GET".to_string()
}

fn default_credential_mode() -> String {
    "basic".to_string()
}

fn log_to_file(message: &str) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let line = format!("[{}] {}\n", timestamp, message);
    println!("{}", line.trim_end());

    if let Ok(cwd) = std::env::current_dir() {
        let path = cwd.join("log").join("log.txt");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = file.write_all(line.as_bytes());
        }
    }
}

fn read_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        value.get(key).and_then(|field| match field {
            Value::String(text) if !text.is_empty() => Some(text.clone()),
            Value::Number(number) => Some(number.to_string()),
            Value::Bool(flag) => Some(flag.to_string()),
            _ => None,
        })
    })
}

fn read_u64(value: &Value, keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| {
        value.get(key).and_then(|field| match field {
            Value::Number(number) => number.as_u64(),
            Value::String(text) => text.parse::<u64>().ok(),
            _ => None,
        })
    })
}

fn has_token_field(value: &Value) -> bool {
    value.get("access_token").is_some()
        || value.get("accessToken").is_some()
        || value.get("refresh_token").is_some()
        || value.get("refreshToken").is_some()
        || value.get("token").is_some()
}

fn find_token_payload(value: &Value) -> Option<&Value> {
    if has_token_field(value) {
        return Some(value);
    }

    match value {
        Value::Object(map) => {
            for key in ["data", "tokens", "result", "payload", "response", "auth"] {
                if let Some(candidate) = map.get(key).and_then(find_token_payload) {
                    return Some(candidate);
                }
            }

            map.values().find_map(find_token_payload)
        }
        Value::Array(items) => items.iter().find_map(find_token_payload),
        _ => None,
    }
}

fn parse_token_response(response_text: &str) -> Result<TokenData, String> {
    let value: Value = serde_json::from_str(response_text)
        .map_err(|error| format!("Kiro response khong phai JSON hop le: {}", error))?;

    let payload = find_token_payload(&value)
        .ok_or_else(|| "Response khong co access_token hoac refresh_token.".to_string())?;

    let access_token = read_string(payload, &["access_token", "accessToken", "token"])
        .unwrap_or_default();
    let refresh_token = read_string(payload, &["refresh_token", "refreshToken"]);

    if access_token.is_empty() && refresh_token.is_none() {
        return Err("Response khong co access_token hoac refresh_token.".to_string());
    }

    Ok(TokenData {
        access_token,
        refresh_token,
        expires_in: read_u64(payload, &["expires_in", "expiresIn"]),
        scope: read_string(payload, &["scope"]),
        token_type: read_string(payload, &["token_type", "tokenType"]),
        id_token: read_string(payload, &["id_token", "idToken"]),
        timestamp: chrono::Utc::now().to_rfc3339(),
        saved_path: None,
    })
}

fn save_token_data(token_data: &TokenData) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir()
        .map_err(|error| format!("Khong doc duoc current directory: {}", error))?;
    let file_path = cwd.join("tokens.json");
    let json = serde_json::to_string_pretty(token_data)
        .map_err(|error| format!("Khong serialize duoc token data: {}", error))?;

    std::fs::write(&file_path, json)
        .map_err(|error| format!("Khong ghi duoc tokens.json: {}", error))?;
    Ok(file_path)
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

#[tauri::command]
async fn login_kiro(config: KiroLoginConfig) -> Result<TokenData, String> {
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

    let mut token_data = parse_token_response(&response_text)?;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login_kiro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::parse_token_response;

    #[test]
    fn parses_flat_refresh_token_response() {
        let token = parse_token_response(r#"{"refresh_token":"refresh","access_token":"access","expires_in":3600}"#)
            .expect("token should parse");

        assert_eq!(token.refresh_token.as_deref(), Some("refresh"));
        assert_eq!(token.access_token, "access");
        assert_eq!(token.expires_in, Some(3600));
    }

    #[test]
    fn parses_nested_refresh_token_response() {
        let token = parse_token_response(r#"{"data":{"tokens":{"refreshToken":"refresh","accessToken":"access"}}}"#)
            .expect("token should parse");

        assert_eq!(token.refresh_token.as_deref(), Some("refresh"));
        assert_eq!(token.access_token, "access");
    }
}
