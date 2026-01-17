use tauri::{AppHandle, Manager, Emitter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use std::net::SocketAddr;

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

// Global state to manage the server shutdown if needed, or just let it timeout
struct AppState {
    // server_handle: Option<tokio::task::JoinHandle<()>>, 
}

#[tauri::command]
async fn login_google(app: AppHandle, config: OAuthConfig) -> Result<TokenData, String> {
    // Validate config
    if config.client_id.is_empty() || config.client_secret.is_empty() {
        return Err("Client ID and Client Secret are required".to_string());
    }

    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = Arc::new(Mutex::new(Some(tx)));

    // Parse port from redirect_uri
    let url = url::Url::parse(&config.redirect_uri)
        .map_err(|e| format!("Invalid redirect URI: {}", e))?;
    let port = url.port().unwrap_or(3000);

    // Create a channel for server shutdown
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));

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
                 warp::reply::html("<h1>✓ Login Successful!</h1><p>You can close this window now.</p><script>setTimeout(() => window.close(), 1500)</script>")
            } else {
                 warp::reply::html("<h1>Error</h1><p>No authorization code received.</p><script>setTimeout(() => window.close(), 3000)</script>")
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

    println!("Opening browser for auth: {}", auth_url);

    // Open Browser
    if let Err(e) = open::that(&auth_url) {
        return Err(format!("Failed to open browser: {}. Please open manually: {}", e, auth_url));
    }

    // Wait for code or timeout
    println!("Waiting for callback on port {}...", port);
    let code = match tokio::time::timeout(std::time::Duration::from_secs(120), rx).await {
        Ok(Ok(c)) => c,
        Ok(Err(_)) => return Err("Login flow cancelled or connection closed".into()),
        Err(_) => return Err("Timeout: No response received within 2 minutes. Please try again.".into()),
    };

    println!("Received auth code. Exchanging for token...");

    // Exchange Code for Token
    let params = [
        ("client_id", &config.client_id),
        ("client_secret", &config.client_secret),
        ("code", &code),
        ("grant_type", &"authorization_code".to_string()),
        ("redirect_uri", &config.redirect_uri),
    ];

    let res = client.post(&config.token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Network error during token exchange: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Token exchange failed ({}): {}", status, err_text));
    }

    let mut token_data: TokenData = res.json().await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;
    token_data.timestamp = Some(chrono::Utc::now().to_rfc3339());

    // Save to file
    let json = serde_json::to_string_pretty(&token_data)
        .map_err(|e| format!("Failed to serialize token data: {}", e))?;
    
    let cwd = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    let file_path = cwd.join("tokens.json");
    
    std::fs::write(&file_path, json)
        .map_err(|e| format!("Failed to write tokens.json: {}", e))?;
    
    println!("✓ Tokens saved successfully to: {:?}", file_path);

    Ok(token_data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![login_google])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
