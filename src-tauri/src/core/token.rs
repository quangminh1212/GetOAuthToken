use serde::Serialize;
use serde_json::Value;

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

pub fn parse_token_response(provider: &str, response_text: &str) -> Result<TokenData, String> {
    let value: Value = serde_json::from_str(response_text)
        .map_err(|error| format!("{} response khong phai JSON hop le: {}", provider, error))?;

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

#[cfg(test)]
mod tests {
    use super::parse_token_response;

    #[test]
    fn parses_flat_refresh_token_response() {
        let token = parse_token_response(
            "Kiro",
            r#"{"refresh_token":"refresh","access_token":"access","expires_in":3600}"#,
        )
        .expect("token should parse");

        assert_eq!(token.refresh_token.as_deref(), Some("refresh"));
        assert_eq!(token.access_token, "access");
        assert_eq!(token.expires_in, Some(3600));
    }

    #[test]
    fn parses_nested_refresh_token_response() {
        let token = parse_token_response(
            "Kiro",
            r#"{"data":{"tokens":{"refreshToken":"refresh","accessToken":"access"}}}"#,
        )
        .expect("token should parse");

        assert_eq!(token.refresh_token.as_deref(), Some("refresh"));
        assert_eq!(token.access_token, "access");
    }
}
