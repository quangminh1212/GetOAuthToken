use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct EmailnatorClient {
    client: Client,
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

// 1secmail API structures
#[derive(Debug, Deserialize)]
struct SecMailMessage {
    id: i64,
    from: String,
    subject: String,
    date: String,
}

impl EmailnatorClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .build()
                .unwrap(),
        }
    }

    // Generate random email using 1secmail API
    fn generate_random_username() -> String {
        let mut rng = rand::thread_rng();
        let random_name: String = (0..10)
            .map(|_| {
                let idx = rng.gen_range(0..26);
                (b'a' + idx) as char
            })
            .collect();
        random_name
    }

    pub async fn generate_email(&mut self) -> Result<EmailData, Box<dyn Error>> {
        println!("Generating email using 1secmail API...");
        
        // Generate random username
        let username = Self::generate_random_username();
        
        // Get available domains from 1secmail
        let domains_response = self.client
            .get("https://www.1secmail.com/api/v1/?action=getDomainList")
            .send()
            .await?;
        
        let domains: Vec<String> = domains_response.json().await?;
        
        if domains.is_empty() {
            return Err("No domains available from 1secmail".into());
        }
        
        // Use first domain (usually 1secmail.com, 1secmail.org, etc.)
        let email = format!("{}@{}", username, domains[0]);
        
        println!("✓ Generated email: {}", email);
        
        Ok(EmailData {
            email: vec![email],
        })
    }

    pub async fn get_inbox(&self, email: &str) -> Result<InboxData, Box<dyn Error>> {
        println!("Fetching inbox for: {}", email);
        
        // Parse email to get username and domain
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email format".into());
        }
        
        let username = parts[0];
        let domain = parts[1];
        
        // Fetch messages from 1secmail API
        let url = format!(
            "https://www.1secmail.com/api/v1/?action=getMessages&login={}&domain={}",
            username, domain
        );
        
        let response = self.client.get(&url).send().await?;
        let messages: Vec<SecMailMessage> = response.json().await?;
        
        println!("✓ Found {} messages", messages.len());
        
        // Convert to our format
        let message_data: Vec<InboxMessage> = messages
            .into_iter()
            .map(|msg| InboxMessage {
                message_id: msg.id.to_string(),
                from: msg.from,
                subject: msg.subject,
                time: msg.date,
            })
            .collect();
        
        Ok(InboxData { message_data })
    }

    pub async fn get_message(&self, email: &str, message_id: &str) -> Result<String, Box<dyn Error>> {
        println!("Fetching message {} for: {}", message_id, email);
        
        // Parse email
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email format".into());
        }
        
        let username = parts[0];
        let domain = parts[1];
        
        // Fetch message content from 1secmail API
        let url = format!(
            "https://www.1secmail.com/api/v1/?action=readMessage&login={}&domain={}&id={}",
            username, domain, message_id
        );
        
        let response = self.client.get(&url).send().await?;
        let message: serde_json::Value = response.json().await?;
        
        // Extract text or HTML body
        let content = if let Some(html) = message.get("htmlBody").and_then(|v| v.as_str()) {
            html.to_string()
        } else if let Some(text) = message.get("textBody").and_then(|v| v.as_str()) {
            format!("<html><body><pre>{}</pre></body></html>", text)
        } else {
            message.to_string()
        };
        
        println!("✓ Message content retrieved");
        
        Ok(content)
    }
}
