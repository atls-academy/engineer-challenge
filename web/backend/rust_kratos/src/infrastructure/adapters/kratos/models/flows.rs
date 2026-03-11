use crate::domain::value_objects::flow_id::FlowId;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FlowResult {
    pub flow_id: FlowId,
    pub csrf_token: String,
    pub cookies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PostFlowResult {
    pub data: Value,
    pub cookies: Vec<String>,
}
