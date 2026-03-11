use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub geo_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct UserProfile {
    pub email: String,
    pub username: String,
    pub geo_location: Option<String>,
}
