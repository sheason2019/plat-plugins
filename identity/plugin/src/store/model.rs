use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub public_key: String,
    pub username: String,
    pub avatar_url: String,
    pub hosts: Vec<IdentityHost>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityHost {
    identity_data_address: String,
    identity_page_address: String,
}
