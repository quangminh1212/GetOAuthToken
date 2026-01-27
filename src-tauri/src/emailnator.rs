use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct EmailnatorClient {
    client: Client,
    api_key: Option<String>,
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

// 1secmail fallback structures
#[derive(Debug, Deserialize)]
struct SecMailMessage {
    id: i64,
    from: String,
    subject: String,
    date: String,
}

impl EmailnatorClient {
    pub fn new() -> Self {
        // Get API key from environment (optional)
        let api_key = std::env::var("RAPIDAPI_KEY").ok();
        
        if api_key.is_some() {
            println!("✓ RapidAPI key found, will use Gmailnator API");
        } else {
            println!("ℹ No RapidAPI key found, will use free 1secmail API");
        }
        
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .build()
                .unwrap(),
            api_key,
        }
    }

    fn generate_random_username() -> String {
        let mut rng = rand::thread_rng();
        (0..10)
            .map(|_| {
                let idx = rng.gen_range(0..26);
                (b'a' + idx) as char
            })
            .collect()
    }

    pub async fn generate_email(&mut self) -> Result<EmailData, Box<dyn Error>> {
        // Try RapidAPI if key is available
        if let Some(ref api_key) = self.api_key {
            match self.try_rapidapi_generate(api_key).await {
                Ok(email_data) => return Ok(email_data),
                Err(e) => {
                    println!("⚠ RapidAPI failed: {}. Falling back to 1secmail...", e);
                }
            }
        }

        // Fallback to 1secmail (free, no API key needed)
        self.generate_with_1secmail().await
    }

    async fn try_rapidapi_generate(&self, api_key: &str) -> Result<EmailData, Box<dyn Error>> {
        println!("Generating email using Gmailnator RapidAPI...");
        
        let response = self.client
            .get("https://gmailnator.p.rapidapi.com/generate-email")
            .header("X-RapidAPI-Key", api_key)
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
            println!("✓ Generated email via RapidAPI: {}", email);
        }
        
        Ok(EmailData {
            email: email_response.email,
        })
    }

    async fn generate_with_1secmail(&self) -> Result<EmailData, Box<dyn Error>> {
        println!("Generating email using 1secmail (free API)...");
        
        let username = Self::generate_random_username();
        
        // Get available domains
        let domains_response = self.client
            .get("https://www.1secmail.com/api/v1/?action=getDomainList")
            .send()
            .await?;
        
        let domains: Vec<String> = domains_response.json().await?;
        
        if domains.is_empty() {
            return Err("No domains available from 1secmail".into());
        }
        
        let email = format!("{}@{}", username, domains[0]);
        
        println!("✓ Generated email via 1secmail: {}", email);
        
        Ok(EmailData {
            email: vec![email],
        })
    }

    pub async fn get_inbox(&self, email: &str) -> Result<InboxData, Box<dyn Error>> {
        println!("Fetching inbox for: {}", email);
        
        // Check if it's a Gmail address (from RapidAPI)
        if email.contains("@gmail.com") || email.contains("@googlemail.com") {
            if let Some(ref api_key) = self.api_key {
                match self.try_rapidapi_inbox(email, api_key).await {
                    Ok(inbox_data) => return Ok(inbox_data),
                    Err(e) => {
                        println!("⚠ RapidAPI inbox failed: {}", e);
                    }
                }
            }
        }

        // Use 1secmail for other domains
        self.get_inbox_1secmail(email).await
    }

    async fn try_rapidapi_inbox(&self, email: &str, api_key: &str) -> Result<InboxData, Box<dyn Error>> {
        let response = self.client
            .post("https://gmailnator.p.rapidapi.com/inbox")
            .header("X-RapidAPI-Key", api_key)
            .header("X-RapidAPI-Host", "gmailnator.p.rapidapi.com")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "email": email }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(format!("Inbox API Error ({}): {}", status, error_text).into());
        }

        let inbox_response: GmailnatorInboxResponse = response.json().await?;
        
        println!("✓ Found {} messages via RapidAPI", inbox_response.message_data.len());
        
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

    async fn get_inbox_1secmail(&self, email: &str) -> Result<InboxData, Box<dyn Error>> {
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email format".into());
        }
        
        let username = parts[0];
        let domain = parts[1];
        
        let url = format!(
            "https://www.1secmail.com/api/v1/?action=getMessages&login={}&domain={}",
            username, domain
        );
        
        let response = self.client.get(&url).send().await?;
        let messages: Vec<SecMailMessage> = response.json().await?;
        
        println!("✓ Found {} messages via 1secmail", messages.len());
        
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
        
        // Check if it's a Gmail address (from RapidAPI)
        if email.contains("@gmail.com") || email.contains("@googlemail.com") {
            if let Some(ref api_key) = self.api_key {
                match self.try_rapidapi_message(email, message_id, api_key).await {
                    Ok(content) => return Ok(content),
                    Err(e) => {
                        println!("⚠ RapidAPI message failed: {}", e);
                    }
                }
            }
        }

        // Use 1secmail for other domains
        self.get_message_1secmail(email, message_id).await
    }

    async fn try_rapidapi_message(&self, email: &str, message_id: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .post("https://gmailnator.p.rapidapi.com/message")
            .header("X-RapidAPI-Key", api_key)
            .header("X-RapidAPI-Host", "gmailnator.p.rapidapi.com")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "email": email,
                "messageID": message_id
            }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(format!("Message API Error ({}): {}", status, error_text).into());
        }

        let content = response.text().await?;
        println!("✓ Message retrieved via RapidAPI");
        
        Ok(content)
    }

    async fn get_message_1secmail(&self, email: &str, message_id: &str) -> Result<String, Box<dyn Error>> {
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email format".into());
        }
        
        let username = parts[0];
        let domain = parts[1];
        
        let url = format!(
            "https://www.1secmail.com/api/v1/?action=readMessage&login={}&domain={}&id={}",
            username, domain, message_id
        );
        
        let response = self.client.get(&url).send().await?;
        let message: serde_json::Value = response.json().await?;
        
        let content = if let Some(html) = message.get("htmlBody").and_then(|v| v.as_str()) {
            html.to_string()
        } else if let Some(text) = message.get("textBody").and_then(|v| v.as_str()) {
            format!("<html><body><pre>{}</pre></body></html>", text)
        } else {
            message.to_string()
        };
        
        println!("✓ Message retrieved via 1secmail");
        
        Ok(content)
    }
}
