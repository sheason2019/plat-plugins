use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    variant: String,
    payload: String,
    time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationIndex {
    agent_time: u64,
    actor: String,
    ref_sha256: String,
    object_sha256: String,
}
