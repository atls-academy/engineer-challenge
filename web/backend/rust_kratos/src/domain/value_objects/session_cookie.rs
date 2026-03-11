#[derive(Debug, Clone)]
pub struct SessionCookie(String);

impl SessionCookie {
    pub fn find_in(cookies: Vec<String>) -> Option<Self> {
        cookies
            .into_iter()
            .find(|c| c.contains("session") || c.starts_with("ory_"))
            .map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SessionCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<SessionCookie> for String {
    fn from(c: SessionCookie) -> Self {
        c.0
    }
}
