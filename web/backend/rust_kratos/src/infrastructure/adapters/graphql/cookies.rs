use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct ResponseCookies {
    pub cookies: Arc<Mutex<Vec<String>>>,
}

#[allow(unused)]
impl ResponseCookies {
    pub fn new() -> Self {
        Self {
            cookies: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_cookie(&self, cookie: String) {
        self.cookies.lock().await.push(cookie);
    }

    pub async fn get_cookies(&self) -> Vec<String> {
        self.cookies.lock().await.clone()
    }

    pub async fn clear(&self) {
        self.cookies.lock().await.clear();
    }
}
