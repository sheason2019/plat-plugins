use std::{
    fs,
    path::{Path, PathBuf},
};

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

impl Identity {
    pub fn find_by_public_key(public_key: String) -> Option<Identity> {
        let identity_path = get_identity_root_path().join(public_key);
        if !identity_path.exists() {
            None
        } else {
            let file_bytes = fs::read(identity_path).unwrap();
            Some(serde_json::from_slice(&file_bytes).unwrap())
        }
    }

    pub fn save(&self) {
        let p = get_identity_root_path().join(&self.public_key);
        fs::write(p, serde_json::to_string(&self).unwrap()).unwrap();
    }
}

pub fn get_identity_root_path() -> PathBuf {
    let p = Path::new("/storage");
    let p = p.join("identity");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap();
    }
    p
}
