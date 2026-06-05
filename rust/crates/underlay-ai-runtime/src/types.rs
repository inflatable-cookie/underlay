use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredOutputSpec {
    pub schema_version: String,
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub system_prompt: String,
    pub user_payload: Value,
    pub structured_output: StructuredOutputSpec,
    pub temperature: f32,
    pub max_output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: Option<String>,
    pub structured_output: Value,
    pub usage: Option<TokenUsage>,
    pub latency_ms: u64,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedModelRoute {
    pub alias: String,
    pub provider_name: String,
    pub model_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelCapability {
    StructuredJson,
    ToolCalling,
    LongContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedModelRouteCandidate {
    pub route: ResolvedModelRoute,
    pub priority: u16,
    pub capabilities: HashSet<ModelCapability>,
}
