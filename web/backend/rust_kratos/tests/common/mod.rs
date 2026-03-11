#![allow(dead_code)]

use reqwest::Client;
use rust_kratos::infrastructure::adapters::kratos::client::KratosClient;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

pub struct TestContext {
    pub client: Arc<KratosClient>,
}

impl TestContext {
    pub fn new() -> Self {
        let public_url = std::env::var("KRATOS_PUBLIC_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4433".to_string());
        let admin_url = std::env::var("KRATOS_ADMIN_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4434".to_string());
        Self {
            client: Arc::new(KratosClient {
                client: Client::builder()
                    .cookie_store(false)
                    .redirect(reqwest::redirect::Policy::none())
                    .danger_accept_invalid_certs(true)
                    .build()
                    .expect("Failed to build HTTP client"),
                public_url,
                admin_url,
                max_retries: 3,
                retry_delay: Duration::from_millis(1000),
            }),
        }
    }

    pub fn random_email() -> String {
        format!("test_{}@example.com", uuid::Uuid::new_v4())
    }
}

pub struct MailhogClient {
    client: Client,
    base_url: String,
}

#[derive(Deserialize)]
pub struct MailhogResponse {
    pub items: Vec<MailhogMessage>,
}

#[derive(Deserialize)]
pub struct MailhogMessage {
    #[serde(rename = "Content")]
    pub content: MailhogContent,
}

#[derive(Deserialize)]
pub struct MailhogContent {
    #[serde(rename = "Body")]
    pub body: String,
}

impl MailhogClient {
    pub fn new() -> Self {
        let base_url = std::env::var("MAILHOG_API_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8025/api/v2".to_string());
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn delete_all(&self) {
        let _ = self
            .client
            .delete(format!("{}/messages", self.base_url))
            .send()
            .await;
    }

    pub async fn fetch_recovery_link(&self, email: &str) -> Option<String> {
        for _ in 0..10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let resp = self
                .client
                .get(format!("{}/search?kind=to&query={}", self.base_url, email))
                .send()
                .await
                .ok()?;
            let data: MailhogResponse = resp.json().await.ok()?;
            if let Some(msg) = data.items.first() {
                if let Some(link) = Self::extract_link(&msg.content.body, "recovery") {
                    return Some(link);
                }
            }
        }
        None
    }

    fn extract_link(body: &str, contains: &str) -> Option<String> {
        let start = body.find("http")?;
        let link: String = body[start..]
            .chars()
            .take_while(|c| !c.is_whitespace() && *c != '"' && *c != '<')
            .collect();
        if link.contains(contains) {
            Some(link)
        } else {
            None
        }
    }
}
