#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthMethod {
    Password,
    Code,
    Link,
}

impl AuthMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Password => "password",
            Self::Code => "code",
            Self::Link => "link",
        }
    }

    pub fn is_recovery(&self) -> bool {
        matches!(self, Self::Link | Self::Code)
    }
}

impl serde::Serialize for AuthMethod {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
