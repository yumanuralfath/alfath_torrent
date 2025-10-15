use once_cell::sync::Lazy;
use std::path::PathBuf;
use tokio::sync::RwLock;

pub static LOG_DIR: Lazy<RwLock<PathBuf>> = Lazy::new(|| RwLock::new(PathBuf::from("./log")));
pub static TORRENTS_DIR: Lazy<RwLock<PathBuf>> =
    Lazy::new(|| RwLock::new(PathBuf::from("./torrents")));

pub async fn set_log_dir<P: Into<PathBuf>>(path: P) {
    let mut dir = LOG_DIR.write().await;
    *dir = path.into();
}

pub async fn get_log_dir() -> PathBuf {
    LOG_DIR.read().await.clone()
}

pub async fn set_torrents_dir<P: Into<PathBuf>>(path: P) {
    let mut dir = TORRENTS_DIR.write().await;
    *dir = path.into();
}

pub async fn get_torrents_dir() -> PathBuf {
    TORRENTS_DIR.read().await.clone()
}
