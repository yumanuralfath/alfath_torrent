use log::error;
use log::info;
use std::fs::create_dir_all;
use std::io::Result;
use std::path::Path;

use std::fs::{metadata, remove_file};
use transmission_rpc::TransClient;
use transmission_rpc::types;

pub fn humanize_size(bytes: f64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    if bytes <= 0.0 {
        return "0 B".to_string();
    }

    let i = (bytes.ln() / 1024f64.ln()).floor() as usize;
    let i = i.min(units.len() - 1);

    let value = bytes / 1024f64.powi(i as i32);
    format!("{:.2} {}", value, units[i])
}

pub async fn check_transmission() -> types::Result<()> {
    let url = "http://localhost:9091/transmission/rpc".parse().unwrap();
    let mut client = TransClient::new(url);

    client.session_stats().await?;

    Ok(())
}

pub fn check_log_size(path: &Path) -> Result<()> {
    let metadata = metadata(path)?;
    let size = metadata.len() / 1000;

    info!("log Size: {size} kilobyte");

    if size >= 100000 {
        match remove_file(path) {
            Ok(_) => info!("File {} successfully remove", path.display()),
            Err(e) => error!("Faield to remove file {}: {e}", path.display()),
        }
    }
    Ok(())
}

pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let dir = path.as_ref();

    if dir.exists() {
        if dir.is_dir() {
            println!(
                "Directory '{}' already exists. Continuing...",
                dir.display()
            );
        } else {
            eprintln!("Path '{}' exists but is not a directory!", dir.display());
        }
    } else {
        create_dir_all(dir)?;
        println!("Directory '{}' created successfully.", dir.display());
    }

    Ok(())
}
