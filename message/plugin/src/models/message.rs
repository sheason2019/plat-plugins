use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    variant: String,
    public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    variant: String,
    content: String,
    time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFeedback {
    variant: String,
    content: String,
    time: u64,
}
