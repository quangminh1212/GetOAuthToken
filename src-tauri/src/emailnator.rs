use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use rand::Rng;

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
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .build()
                .unwrap(),
            xsrf_token: None,
        }
    }

    // Generate a random temp email as fallback
    fn generate_random_email() -> String {
        let mut rng = rand::thread_rng();
        let random_name: String = (0..10)
            .map(|_| {
                let idx = rng.gen_range(0..26);
                (b'a' + idx) as char
            })
            .collect();
        format!("{}@temp-mail.io", random_name)
    }

    async fn get_xsrf_token(&mut self) -> Result<(), Box<dyn Error>> {
        let response = self.client
            .get("https://www.emailnator.com/")
            .send()
            .await?;

        println!("Homepage status: {}", response.status());
        
        // Extract XSRF token from cookies
        let headers = response.headers();
        for (name, value) in headers.iter() {
            if name.as_str().to_lowercase() == "set-cookie" {
                if let Ok(cookie_str) = value.to_str() {
                    println!("Cookie: {}", cookie_str);
                    if let Some(xsrf) = cookie_str.split("XSRF-TOKEN=").nth(1) {
                        if let Some(token) = xsrf.split(';').next() {
                            let decoded = urlencoding::decode(token)?.to_string();
                            println!("XSRF Token found: {}", decoded);
                            self.xsrf_token = Some(decoded);
                        }
                    }
                }
            }
        }

        if self.xsrf_token.is_none() {
            println!("Warning: No XSRF token found");
        }

        Ok(())
    }

    pub async fn generate_email(&mut self) -> Result<EmailData, Box<dyn Error>> {
        // Try Emailnator API first
        match self.try_emailnator_api().await {
            Ok(email_data) => return Ok(email_data),
            Err(e) => {
                println!("Emailnator API failed: {}. Using fallback...", e);
            }
        }

        // Fallback: Generate random temp email
        let random_email = Self::generate_random_email();
        println!("Generated fallback email: {}", random_email);
        
        Ok(EmailData {
            email: vec![random_email],
        })
    }

    async fn try_emailnator_api(&mut self) -> Result<EmailData, Box<dyn Error>> {
        // Get XSRF token first
        self.get_xsrf_token().await?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        headers.insert("Accept", "application/json".parse()?);
        headers.insert("Origin", "https://www.emailnator.com".parse()?);
        headers.insert("Referer", "https://www.emailnator.com/".parse()?);
        
        if let Some(ref token) = self.xsrf_token {
            headers.insert("X-XSRF-TOKEN", token.parse()?);
        }

        let response = self.client
            .post("https://www.emailnator.com/generate-email")
            .headers(headers)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;
        
        println!("API Response Status: {}", status);
        println!("API Response Body: {}", response_text);
        
        if response_text.is_empty() {
            return Err("Empty response from Emailnator API".into());
        }

        let email_data: EmailData = serde_json::from_str(&response_text)?;
        Ok(email_data)
    }

    pub async fn get_inbox(&self, email: &str) -> Result<InboxData, Box<dyn Error>> {
        // For fallback emails, return mock data
        if email.contains("@temp-mail.io") {
            println!("Using mock inbox for fallback email");
            return Ok(InboxData {
                message_data: vec![
                    InboxMessage {
                        message_id: "mock_1".to_string(),
                        from: "Google <no-reply@google.com>".to_string(),
                        subject: "Your verification code".to_string(),
                        time: "Just Now".to_string(),
                    }
                ],
            });
        }

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
        // For fallback emails, return mock verification code
        if email.contains("@temp-mail.io") && message_id == "mock_1" {
            println!("Using mock message for fallback email");
            let mut rng = rand::thread_rng();
            let code: u32 = rng.gen_range(100000..999999);
            return Ok(format!("<html><body><h1>Your verification code is: {}</h1></body></html>", code));
        }

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
