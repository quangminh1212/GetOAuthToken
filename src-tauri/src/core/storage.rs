use crate::core::token::TokenData;
use std::path::PathBuf;

pub fn save_token_data(token_data: &TokenData) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir()
        .map_err(|error| format!("Khong doc duoc current directory: {}", error))?;
    let file_path = cwd.join("tokens.json");
    let json = serde_json::to_string_pretty(token_data)
        .map_err(|error| format!("Khong serialize duoc token data: {}", error))?;

    std::fs::write(&file_path, json)
        .map_err(|error| format!("Khong ghi duoc tokens.json: {}", error))?;
    Ok(file_path)
}
