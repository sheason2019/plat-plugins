use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use autosurgeon::{hydrate, reconcile, Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    public_key: String,
    username: String,
    avatar_url: String,
    hosts: Vec<IdentityHost>,
    // milliseconds since epoch
    updated_at: u64,
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq, Serialize, Deserialize)]
pub struct IdentityHost {
    identity_data_address: String,
    identity_page_address: String,
}

impl Identity {
    pub fn create_empty(public_key: String) -> Identity {
        Identity {
            public_key,
            username: "".to_string(),
            avatar_url: "".to_string(),
            hosts: Vec::new(),
            updated_at: 0,
        }
    }

    pub fn find_one(public_key: String) -> anyhow::Result<Option<Identity>> {
        let user_dir = must_get_user_dir();
        let identity_file = user_dir.join(public_key + ".automerge");
        if !identity_file.exists() {
            return Ok(None);
        }

        let identity_file_bytes = fs::read(identity_file)?;
        let doc = automerge::AutoCommit::load(&identity_file_bytes)?;

        let identity: Identity = hydrate(&doc)?;
        Ok(Some(identity))
    }

    pub fn mutate_updated_at(&mut self) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("get time by milliseconds since epoch failed")
            .as_millis() as u64;
        self.updated_at = time;
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let identity_file = must_get_user_dir().join(self.public_key.clone() + ".automerge");
        let mut doc = automerge::AutoCommit::new();
        reconcile(&mut doc, &self.clone())?;

        let identity_bytes = doc.save();
        fs::write(identity_file, &identity_bytes)?;

        Ok(())
    }
}

fn must_get_user_dir() -> PathBuf {
    let p = Path::new("/storage");
    let identity_dir = p.join("identity");
    if !identity_dir.exists() {
        fs::create_dir_all(&identity_dir).expect("create identity dir failed");
    }

    identity_dir
}
