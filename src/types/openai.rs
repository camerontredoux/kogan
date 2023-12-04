use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct OAIChoices {
    pub message: OAIMessage,
    pub index: u8,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct OAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct OAIResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Vec<OAIChoices>,
}

#[derive(Serialize, Debug)]
pub struct OAIRequest {
    pub model: String,
    pub messages: Vec<OAIReqMessage>,
}

#[derive(Serialize, Debug)]
pub struct OAIReqMessage {
    pub role: String,
    pub content: String,
}
