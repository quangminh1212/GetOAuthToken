use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct EmailnatorClient {
    client: Client,
    api_key: String,
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

// RapidAPI Gmailnator response structures
#[derive(Debug, Deserialize)]
struct GmailnatorEmailResponse {
    email: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GmailnatorMessage {
    #[serde(rename = "messageID")]
    message_id: String,
    from: String,
    subject: String,
    time: String,
}

#[derive(Debug, Deserialize)]
struct GmailnatorInboxResponse {
    #[serde(rename = "messageData")]
    message_data: Vec<GmailnatorMessage>,
}

impl EmailnatorClient {
    pub fn new() -> Self {
        // Get API key from environment or use default
        let api_key = std::env::var("RAPIDAPI_KEY")
            .unwrap_or_else(|_| "YOUR_RAPIDAPI_KEY".to_string());
        
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .build()
                .unwrap(),
            api_key,
        }
    }

    pub async fn generate_email(&mut self) -> Result<EmailData, Box<dyn Error>> {
        println!("Generating email using Gmailnator RapidAPI...");
        
        let response = self.client
            .get("https://gmailnator.p.rapidapi.com/generate-email")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "gmailnator.p.rapidapi.com")
            .send()
            .await?;

        let status = response.status();
        println!("API Response Status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(format!("API Error ({}): {}", status, error_text).into());
        }

        let email_response: GmailnatorEmailResponse = response.json().await?;
        
        if let Some(email) = email_response.email.first() {
            println!("✓ Generated email: {}", email);
        }
        
        Ok(EmailData {
            email: email_response.email,
        })
    }

    pub async fn get_inbox(&self, email: &str) -> Result<InboxData, Box<dyn Error>> {
        println!("Fetching inbox for: {}", email);
        
        let response = self.client
            .post("https://gmailnator.p.rapidapi.com/inbox")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "gmailnator.p.rapidapi.com")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "email": email }))
            .send()
            .await?;

        let status = response.status();
        println!("Inbox API Response Status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(format!("Inbox API Error ({}): {}", status, error_text).into());
        }

        let inbox_response: GmailnatorInboxResponse = response.json().await?;
        
        println!("✓ Found {} messages", inbox_response.message_data.len());
        
        // Convert to our format
        let message_data: Vec<InboxMessage> = inbox_response.message_data
            .into_iter()
            .map(|msg| InboxMessage {
                message_id: msg.message_id,
                from: msg.from,
                subject: msg.subject,
                time: msg.time,
            })
            .collect();
        
        Ok(InboxData { message_data })
    }

    pub async fn get_message(&self, email: &str, message_id: &str) -> Result<String, Box<dyn Error>> {
        println!("Fetching message {} for: {}", message_id, email);
        
        let response = self.client
            .post("https://gmailnator.p.rapidapi.com/message")
            .header("X-RapidAPI-Key", &self.api_key)
            .header("X-RapidAPI-Host", "gmailnator.p.rapidapi.com")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "email": email,
                "messageID": message_id
            }))
            .send()
            .await?;

        let status = response.status();
        println!("Message API Response Status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(format!("Message API Error ({}): {}", status, error_text).into());
        }

        let content = response.text().await?;
        
        println!("✓ Message content retrieved");
        
        Ok(content)
    }
}
