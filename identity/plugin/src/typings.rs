use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignBox {
    pub public_key: String,
    pub signature: String,
}
