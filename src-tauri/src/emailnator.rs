use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct EmailnatorClient {
    client: Client,
    current_sid_token: Option<String>,
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

// Guerrilla Mail API structures
#[derive(Debug, Deserialize)]
struct GuerrillaResponse {
    email_addr: String,
    sid_token: String,
}

impl EmailnatorClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .build()
                .unwrap(),
            current_sid_token: None,
        }
    }

    pub async fn generate_email(&mut self) -> Result<EmailData, Box<dyn Error>> {
        println!("Generating email using Guerrilla Mail API...");
        
        // Get new email address
        let response = self.client
            .get("https://api.guerrillamail.com/ajax.php")
            .query(&[
                ("f", "get_email_address"),
                ("ip", "127.0.0.1"),
                ("agent", "Mozilla/5.0")
            ])
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API Error ({}): {}", status, error_text).into());
        }
        
        let result: GuerrillaResponse = response.json().await?;
        
        println!("✓ Generated email: {}", result.email_addr);
        println!("✓ Session token: {}...", &result.sid_token[..20.min(result.sid_token.len())]);
        
        // Store session token for later use
        self.current_sid_token = Some(result.sid_token);
        
        Ok(EmailData {
            email: vec![result.email_addr],
        })
    }

    pub async fn get_inbox(&self, _email: &str) -> Result<InboxData, Box<dyn Error>> {
        println!("Fetching inbox...");
        
        let sid_token = self.current_sid_token.as_ref()
            .ok_or("No session token available. Generate email first.")?;
        
        let response = self.client
            .get("https://api.guerrillamail.com/ajax.php")
            .query(&[
                ("f", "get_email_list"),
                ("offset", "0"),
                ("sid_token", sid_token)
            ])
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Failed to fetch inbox ({}): {}", status, error_text).into());
        }
        
        let data: serde_json::Value = response.json().await?;
        let emails = data["list"].as_array()
            .ok_or("Invalid response format")?;
        
        println!("✓ Found {} messages", emails.len());
        
        // Convert to our format
        let inbox_messages: Vec<InboxMessage> = emails.iter()
            .filter_map(|email| {
                Some(InboxMessage {
                    message_id: email["mail_id"].as_str()?.to_string(),
                    from: email["mail_from"].as_str()?.to_string(),
                    subject: email["mail_subject"].as_str()?.to_string(),
                    time: email["mail_timestamp"].as_str()?.to_string(),
                })
            })
            .collect();
        
        Ok(InboxData {
            message_data: inbox_messages,
        })
    }

    pub async fn get_message(&self, _email: &str, message_id: &str) -> Result<String, Box<dyn Error>> {
        println!("Fetching message: {}", message_id);
        
        let sid_token = self.current_sid_token.as_ref()
            .ok_or("No session token available")?;
        
        let response = self.client
            .get("https://api.guerrillamail.com/ajax.php")
            .query(&[
                ("f", "fetch_email"),
                ("email_id", message_id),
                ("sid_token", sid_token)
            ])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err("Failed to fetch message".into());
        }
        
        let data: serde_json::Value = response.json().await?;
        
        // Get HTML or text content
        let content = data["mail_body"].as_str()
            .unwrap_or("<html><body><p>No content</p></body></html>")
            .to_string();
        
        println!("✓ Message content retrieved");
        
        Ok(content)
    }
}
