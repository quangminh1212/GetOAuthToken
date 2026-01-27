use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct EmailnatorClient {
    client: Client,
    xsrf_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailData {
    pub email: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InboxMessage {
    #[serde(rename = "messageID")]
    pub message_id: String,
    pub from: String,
    pub subject: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InboxData {
    #[serde(rename = "messageData")]
    pub message_data: Vec<InboxMessage>,
}

impl EmailnatorClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .build()
                .unwrap(),
            xsrf_token: None,
        }
    }

    async fn get_xsrf_token(&mut self) -> Result<(), Box<dyn Error>> {
        let response = self.client
            .get("https://www.emailnator.com/")
            .send()
            .await?;

        // Extract XSRF token from cookies
        if let Some(cookie_header) = response.headers().get("set-cookie") {
            let cookie_str = cookie_header.to_str()?;
            if let Some(xsrf) = cookie_str.split("XSRF-TOKEN=").nth(1) {
                if let Some(token) = xsrf.split(';').next() {
                    self.xsrf_token = Some(urlencoding::decode(token)?.to_string());
                }
            }
        }

        Ok(())
    }

    pub async fn generate_email(&mut self) -> Result<EmailData, Box<dyn Error>> {
        // Get XSRF token first
        self.get_xsrf_token().await?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        
        if let Some(ref token) = self.xsrf_token {
            headers.insert("X-XSRF-TOKEN", token.parse()?);
        }

        let response = self.client
            .post("https://www.emailnator.com/generate-email")
            .headers(headers)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        let email_data: EmailData = response.json().await?;
        Ok(email_data)
    }

    pub async fn get_inbox(&self, email: &str) -> Result<InboxData, Box<dyn Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        
        if let Some(ref token) = self.xsrf_token {
            headers.insert("X-XSRF-TOKEN", token.parse()?);
        }

        let response = self.client
            .post("https://www.emailnator.com/message-list")
            .headers(headers)
            .json(&serde_json::json!({ "email": email }))
            .send()
            .await?;

        let inbox_data: InboxData = response.json().await?;
        Ok(inbox_data)
    }

    pub async fn get_message(&self, email: &str, message_id: &str) -> Result<String, Box<dyn Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        
        if let Some(ref token) = self.xsrf_token {
            headers.insert("X-XSRF-TOKEN", token.parse()?);
        }

        let response = self.client
            .post("https://www.emailnator.com/message-list")
            .headers(headers)
            .json(&serde_json::json!({
                "email": email,
                "messageID": message_id
            }))
            .send()
            .await?;

        let content = response.text().await?;
        Ok(content)
    }
}
