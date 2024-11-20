use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskInput {
    /// the DAO address
    pub dao: String,
    /// the address being evaluated
    pub address: String,
    /// the incrementing message index, starting at 0
    pub message_id: u16,
    /// the next message in the conversation
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskOutput {
    Success(TaskOutputSuccess),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskOutputSuccess {
    /// the DAO address
    pub dao: String,
    /// the address being evaluated
    pub address: String,
    /// the message ID being responded to
    pub message_id: u16,
    /// the response to the message
    pub response: String,
    /// the decision made by the AI bouncer, which will be present once
    /// finalized
    pub decision: Option<bool>,
}
