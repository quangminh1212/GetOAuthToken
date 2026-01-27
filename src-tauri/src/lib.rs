use tauri::AppHandle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use std::fs::OpenOptions;
use std::io::Write;

mod emailnator;
use emailnator::{EmailnatorClient, EmailData, InboxData};

// Global state for email client
lazy_static::lazy_static! {
    static ref EMAIL_CLIENT: Arc<Mutex<EmailnatorClient>> = Arc::new(Mutex::new(EmailnatorClient::new()));
}

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

    log_to_file(&format!("✓ Token exchange successful (status: {})", res.status()));
    
    // Log raw response for debugging
    let response_text = res.text().await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to read response text: {}", e));
            format!("Failed to read response text: {}", e)
        })?;
    
    log_to_file(&format!("Raw token response: {}", response_text));
    
    // Parse response
    let mut token_data: TokenData = serde_json::from_str(&response_text)
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to parse token response: {}", e));
            format!("Failed to parse token response: {}", e)
        })?;
    token_data.timestamp = Some(chrono::Utc::now().to_rfc3339());
    
    // Log token info (without exposing full tokens)
    log_to_file("✓ Token data parsed successfully");
    log_to_file(&format!("  - access_token: {}... (length: {})", 
        &token_data.access_token[..20.min(token_data.access_token.len())], 
        token_data.access_token.len()));
    
    if let Some(ref rt) = token_data.refresh_token {
        log_to_file(&format!("  - refresh_token: ✓ PRESENT (length: {}, starts with: {})", 
            rt.len(),
            &rt[..10.min(rt.len())]));
    } else {
        log_to_file("  - refresh_token: ✗ NOT PRESENT - This may happen if user already granted consent before");
        log_to_file("    To get refresh_token: Revoke app access at https://myaccount.google.com/permissions and try again");
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
        log_to_file(&format!("  - id_token: {}... (length: {})", &id_token[..20.min(id_token.len())], id_token.len()));
    }

    // Save to file
    let json = serde_json::to_string_pretty(&token_data)
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
    
    println!("✓ Tokens saved successfully to: {:?}", file_path);
    log_to_file(&format!("✓ Tokens saved to: {:?}", file_path));
    log_to_file("========== OAUTH LOGIN COMPLETED SUCCESSFULLY ==========\n");

    Ok(token_data)
}

// Emailnator commands
#[tauri::command]
async fn generate_temp_email() -> Result<EmailData, String> {
    log_to_file("========== GENERATING TEMP EMAIL ==========");
    
    let mut client = EMAIL_CLIENT.lock().await;
    let email_data = client.generate_email()
        .await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to generate email: {}", e));
            format!("Failed to generate email: {}", e)
        })?;
    
    if let Some(email) = email_data.email.first() {
        log_to_file(&format!("✓ Generated temp email: {}", email));
    }
    
    log_to_file("========== TEMP EMAIL GENERATED ==========\n");
    Ok(email_data)
}

#[tauri::command]
async fn get_email_inbox(email: String) -> Result<InboxData, String> {
    log_to_file(&format!("========== FETCHING INBOX FOR: {} ==========", email));
    
    let client = EMAIL_CLIENT.lock().await;
    let inbox_data = client.get_inbox(&email)
        .await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to fetch inbox: {}", e));
            format!("Failed to fetch inbox: {}", e)
        })?;
    
    log_to_file(&format!("✓ Found {} messages", inbox_data.message_data.len()));
    log_to_file("========== INBOX FETCHED ==========\n");
    
    Ok(inbox_data)
}

#[tauri::command]
async fn get_email_message(email: String, message_id: String) -> Result<String, String> {
    log_to_file(&format!("========== FETCHING MESSAGE: {} ==========", message_id));
    
    let client = EMAIL_CLIENT.lock().await;
    let content = client.get_message(&email, &message_id)
        .await
        .map_err(|e| {
            log_to_file(&format!("ERROR: Failed to fetch message: {}", e));
            format!("Failed to fetch message: {}", e)
        })?;
    
    log_to_file("✓ Message content retrieved");
    log_to_file("========== MESSAGE FETCHED ==========\n");
    
    Ok(content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![
        login_google,
        generate_temp_email,
        get_email_inbox,
        get_email_message
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
