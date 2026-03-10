use crate::domain::errors::DomainError;
use crate::domain::value_objects::flow_id::FlowId;

pub struct Flow {
    pub id: FlowId,
    pub csrf_token: String,
}

impl Flow {
    pub fn from_json(value: &serde_json::Value) -> Result<Self, DomainError> {
        let id = value["id"]
            .as_str()
            .ok_or_else(|| DomainError::NotFound("flow id".into()))
            .map(FlowId::new)?;

        let csrf_token = value["ui"]["nodes"]
            .as_array()
            .and_then(|nodes| {
                nodes
                    .iter()
                    .find(|n| n["attributes"]["name"].as_str() == Some("csrf_token"))
            })
            .and_then(|n| n["attributes"]["value"].as_str())
            .ok_or_else(|| DomainError::NotFound("csrf_token".into()))?
            .to_string();

        Ok(Self { id, csrf_token })
    }
}
