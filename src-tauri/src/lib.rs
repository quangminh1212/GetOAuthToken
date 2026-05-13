use tauri::AppHandle;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use std::fs::OpenOptions;
use std::io::Write;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BasicLoginConfig {
    pub auth_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub timestamp: Option<String>,
}

// Logging utility
fn log_to_file(message: &str) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let log_message = format!("[{}] {}\n", timestamp, message);
    
    // Print to console
    println!("{}", log_message.trim());
    
    // Write to log file
    if let Ok(cwd) = std::env::current_dir() {
        let log_path = cwd.join("log").join("log.txt");
        
        // Create log directory if not exists
        if let Some(parent) = log_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path) {
            let _ = file.write_all(log_message.as_bytes());
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

fn select_token_payload<'a>(root: &'a Value) -> &'a Value {
    let has_token_fields = |value: &Value| {
        value.get("access_token").is_some()
            || value.get("accessToken").is_some()
            || value.get("refresh_token").is_some()
            || value.get("refreshToken").is_some()
            || value.get("token").is_some()
    };

    if has_token_fields(root) {
        return root;
    }

    for key in ["data", "tokens", "result", "payload"] {
        if let Some(candidate) = root.get(key) {
            if has_token_fields(candidate) {
                return candidate;
            }
        }
    }

    root
}

fn parse_token_response(response_text: &str) -> Result<TokenData, String> {
    let value: Value = serde_json::from_str(response_text)
        .map_err(|e| format!("Failed to parse token response JSON: {}", e))?;

    let payload = select_token_payload(&value);

    let access_token = read_string(payload, &["access_token", "accessToken", "token"])
        .unwrap_or_default();

    let refresh_token = read_string(payload, &["refresh_token", "refreshToken"]);

    if access_token.is_empty() && refresh_token.is_none() {
        return Err("Response does not contain access_token/accessToken or refresh_token/refreshToken".to_string());
    }

    Ok(TokenData {
        access_token,
        refresh_token,
        expires_in: read_u64(payload, &["expires_in", "expiresIn"]),
        scope: read_string(payload, &["scope"]),
        token_type: read_string(payload, &["token_type", "tokenType"]),
        id_token: read_string(payload, &["id_token", "idToken"]),
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
    })
}

fn log_token_summary(token_data: &TokenData) {
    log_to_file("Token data parsed successfully");
    log_to_file(&format!(
        "  - access_token: present (length: {})",
        token_data.access_token.len()
    ));

    if let Some(ref rt) = token_data.refresh_token {
        log_to_file(&format!("  - refresh_token: present (length: {})", rt.len()));
    } else {
        log_to_file("  - refresh_token: not present in response");
    }

    if let Some(expires) = token_data.expires_in {
        log_to_file(&format!("  - expires_in: {} seconds (~{} minutes)", expires, expires / 60));
    }
    if let Some(ref scope) = token_data.scope {
        log_to_file(&format!("  - scope: {}", scope));
    }
    if let Some(ref token_type) = token_data.token_type {
        log_to_file(&format!("  - token_type: {}", token_type));
    }
    if let Some(ref id_token) = token_data.id_token {
        log_to_file(&format!("  - id_token: present (length: {})", id_token.len()));
    }
}

fn save_token_data(token_data: &TokenData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(token_data)
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to serialize token data: {}", e));
            format!("Failed to serialize token data: {}", e)
        })?;

    let cwd = std::env::current_dir()
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to get current directory: {}", e));
            format!("Failed to get current directory: {}", e)
        })?;
    let file_path = cwd.join("tokens.json");

    std::fs::write(&file_path, json)
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to write tokens.json: {}", e));
            format!("Failed to write tokens.json: {}", e)
        })?;

    println!("Tokens saved successfully to: {:?}", file_path);
    log_to_file(&format!("Tokens saved to: {:?}", file_path));
    Ok(())
}

#[tauri::command]
async fn login_google(_app: AppHandle, config: OAuthConfig) -> Result<TokenData, String> {
    log_to_file("========== OAUTH LOGIN STARTED ==========");
    log_to_file(&format!("Client ID: {}...{}", &config.client_id[..20.min(config.client_id.len())], if config.client_id.len() > 20 { "***" } else { "" }));
    log_to_file(&format!("Redirect URI: {}", config.redirect_uri));
    log_to_file(&format!("Scope: {}", config.scope));
    
    // Validate config
    if config.client_id.is_empty() || config.client_secret.is_empty() {
        log_to_file("ERROR: Client ID or Client Secret is empty");
        return Err("Client ID and Client Secret are required".to_string());
    }
    log_to_file("✓ Config validation passed");

    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = Arc::new(Mutex::new(Some(tx)));

    // Parse port from redirect_uri
    let url = url::Url::parse(&config.redirect_uri)
        .map_err(|e| {
            log_to_file(&format!("ERROR: Invalid redirect URI: {}", e));
            format!("Invalid redirect URI: {}", e)
        })?;
    let port = url.port().unwrap_or(3000);
    log_to_file(&format!("✓ Parsed port from redirect URI: {}", port));

    // Create a channel for server shutdown
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));
    
    log_to_file(&format!("✓ Starting callback server on port {}", port));

    // Create a temporary server for the callback
    let callback = warp::get()
        .and(warp::path("oauth"))
        .and(warp::path("callback"))
        .and(warp::query::<HashMap<String, String>>())
        .map(move |p: HashMap<String, String>| {
            let code = p.get("code").cloned();
            let error = p.get("error").cloned();
            let tx = tx.clone();
            let shutdown_tx = shutdown_tx.clone();
            
            if let Some(err) = error {
                log_to_file(&format!("ERROR: OAuth callback received error: {}", err));
                tokio::spawn(async move {
                    let mut s_lock = shutdown_tx.lock().await;
                    if let Some(s) = s_lock.take() {
                        let _ = s.send(());
                    }
                });
                return warp::reply::html(format!(
                    "<h1>Login Failed</h1><p>Error: {}</p><script>setTimeout(() => window.close(), 3000)</script>",
                    err
                ));
            }
            
            if let Some(c) = code {
                 log_to_file(&format!("✓ Received authorization code: {}...", &c[..20.min(c.len())]));
                 let tx_clone = tx.clone();
                 // Send code back to main thread
                 tokio::spawn(async move {
                     let mut lock = tx_clone.lock().await;
                     if let Some(sender) = lock.take() {
                         let _ = sender.send(c);
                     }
                     // Signal shutdown
                     let mut s_lock = shutdown_tx.lock().await;
                     if let Some(s) = s_lock.take() {
                         let _ = s.send(());
                     }
                 });
                 warp::reply::html("<h1>✓ Login Successful!</h1><p>You can close this window now.</p><script>setTimeout(() => window.close(), 1500)</script>".to_string())
            } else {
                 log_to_file("ERROR: No authorization code received in callback");
                 warp::reply::html("<h1>Error</h1><p>No authorization code received.</p><script>setTimeout(() => window.close(), 3000)</script>".to_string())
            }
        });

    let (_, server) = warp::serve(callback)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], port), async move {
             shutdown_rx.await.ok();
        });

    // Spawn server
    tauri::async_runtime::spawn(server);

    // Construct Auth URL
    let client = reqwest::Client::new();
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
        config.auth_url, 
        urlencoding::encode(&config.client_id),
        urlencoding::encode(&config.redirect_uri),
        urlencoding::encode(&config.scope)
    );

    log_to_file("✓ Constructed authorization URL");
    log_to_file(&format!("Auth URL: {}", auth_url));
    log_to_file("NOTE: Using access_type=offline and prompt=consent to ensure refresh_token is returned");
    
    println!("Opening browser for auth: {}", auth_url);

    // Open Browser
    if let Err(e) = open::that(&auth_url) {
        log_to_file(&format!("ERROR: Failed to open browser: {}", e));
        return Err(format!("Failed to open browser: {}. Please open manually: {}", e, auth_url));
    }
    log_to_file("✓ Browser opened successfully");

    // Wait for code or timeout
    println!("Waiting for callback on port {}...", port);
    log_to_file("Waiting for OAuth callback (timeout: 120 seconds)...");
    let code = match tokio::time::timeout(std::time::Duration::from_secs(120), rx).await {
        Ok(Ok(c)) => {
            log_to_file("✓ Authorization code received from callback");
            c
        },
        Ok(Err(_)) => {
            log_to_file("ERROR: Login flow cancelled or connection closed");
            return Err("Login flow cancelled or connection closed".into());
        },
        Err(_) => {
            log_to_file("ERROR: Timeout - No response received within 2 minutes");
            return Err("Timeout: No response received within 2 minutes. Please try again.".into());
        },
    };

    println!("Received auth code. Exchanging for token...");
    log_to_file("Exchanging authorization code for access token...");

    // Exchange Code for Token
    let params = [
        ("client_id", &config.client_id),
        ("client_secret", &config.client_secret),
        ("code", &code),
        ("grant_type", &"authorization_code".to_string()),
        ("redirect_uri", &config.redirect_uri),
    ];
    
    log_to_file(&format!("POST {}", config.token_url));
    log_to_file("Request params: client_id, client_secret, code, grant_type=authorization_code, redirect_uri");

    let res = client.post(&config.token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Network error during token exchange: {}", e));
            format!("Network error during token exchange: {}", e)
        })?;

    if !res.status().is_success() {
        let status = res.status();
        let err_text = res.text().await.unwrap_or_default();
        log_to_file(&format!("ERROR: Token exchange failed ({}): {}", status, err_text));
        return Err(format!("Token exchange failed ({}): {}", status, err_text));
    }

    log_to_file(&format!("Token exchange successful (status: {})", res.status()));

    let response_text = res.text().await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to read response text: {}", e));
            format!("Failed to read response text: {}", e)
        })?;

    let token_data = parse_token_response(&response_text)
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to parse token response: {}", e));
            e
        })?;

    log_token_summary(&token_data);
    save_token_data(&token_data)?;
    log_to_file("========== OAUTH LOGIN COMPLETED SUCCESSFULLY ==========\n");

    Ok(token_data)
}


#[tauri::command]
async fn login_with_password(_app: AppHandle, config: BasicLoginConfig) -> Result<TokenData, String> {
    log_to_file("========== BASIC LOGIN STARTED ==========");

    if config.auth_url.is_empty() || config.username.is_empty() || config.password.is_empty() {
        log_to_file("ERROR: auth_url, username or password is empty");
        return Err("Login endpoint, username and password are required".to_string());
    }

    let client = reqwest::Client::new();
    let credentials = BASE64_STANDARD.encode(format!("{}:{}", config.username, config.password));

    log_to_file(&format!("Calling login endpoint: {}", config.auth_url));

    let res = client
        .get(&config.auth_url)
        .header("Authorization", format!("Basic {}", credentials))
        .send()
        .await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Network error during password login: {}", e));
            format!("Network error during password login: {}", e)
        })?;

    if !res.status().is_success() {
        let status = res.status();
        let err_text = res.text().await.unwrap_or_default();
        let short_error: String = err_text.chars().take(400).collect();
        log_to_file(&format!("ERROR: Password login failed ({})", status));
        return Err(format!("Password login failed ({}): {}", status, short_error));
    }

    let response_text = res.text().await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to read password login response: {}", e));
            format!("Failed to read password login response: {}", e)
        })?;

    let token_data = parse_token_response(&response_text)
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to parse password login response: {}", e));
            e
        })?;

    log_token_summary(&token_data);
    save_token_data(&token_data)?;
    log_to_file("========== BASIC LOGIN COMPLETED SUCCESSFULLY ==========\n");

    Ok(token_data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![login_google, login_with_password])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


