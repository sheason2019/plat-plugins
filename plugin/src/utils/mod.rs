use std::{
    fs,
    path::{self, PathBuf},
};

pub fn must_get_user_dir() -> PathBuf {
    let p = path::Path::new("/static");
    let user_dir = p.join("users");
    if !user_dir.exists() {
        fs::create_dir_all(&user_dir).expect("create user dir failed");
    }

    user_dir
}
