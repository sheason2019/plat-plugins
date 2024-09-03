use std::{
    fs,
    path::{Path, PathBuf},
};

use automerge::AutoCommit;
use autosurgeon::{hydrate, reconcile, Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub public_key: String,
    pub username: String,
    pub avatar_url: String,
    pub hosts: Vec<IdentityHost>,
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

    pub fn save(&self) -> anyhow::Result<()> {
        let identity_file = must_get_user_dir().join(self.public_key.clone() + ".automerge");
        let doc = find_doc(self.public_key.clone())?;
        let mut doc = if doc.is_some() {
            doc.unwrap()
        } else {
            automerge::AutoCommit::new()
        };

        reconcile(&mut doc, &self.clone())?;

        let identity_bytes = doc.save();
        fs::write(identity_file, &identity_bytes)?;

        Ok(())
    }
}

fn find_doc(public_key: String) -> anyhow::Result<Option<AutoCommit>> {
    let user_dir = must_get_user_dir();
    let identity_file = user_dir.join(public_key + ".automerge");
    if !identity_file.exists() {
        return Ok(None);
    }

    let identity_file_bytes = fs::read(identity_file)?;
    let doc = automerge::AutoCommit::load(&identity_file_bytes)?;
    Ok(Some(doc))
}

fn must_get_user_dir() -> PathBuf {
    let p = Path::new("/storage");
    let identity_dir = p.join("identity");
    if !identity_dir.exists() {
        fs::create_dir_all(&identity_dir).expect("create identity dir failed");
    }

    identity_dir
}
