use log::LevelFilter;
use log::{error, info};
use simplelog::*;
use std::fs::{self, OpenOptions, metadata, remove_file};
use std::io::{self, Result};
use std::path::Path;
use std::process::Command;
use transmission_rpc::{TransClient, types};

use crate::torrent::folder::{get_log_dir, get_torrents_dir, set_log_dir, set_torrents_dir};

pub async fn init_service() {
    set_log_dir("./log").await;
    set_torrents_dir("./torrents").await;

    let log_dir = get_log_dir().await;
    let torrents_dir = get_torrents_dir().await;

    ensure_dir(&log_dir).expect("Failed to create log directory");
    ensure_dir(torrents_dir).expect("Failed to create torrents directory");

    let log_file = log_dir.join("alfath_torrent.log");

    init_logger(log_file.to_str().unwrap());

    check_log_size(&log_file).expect("Failed to remove log file");
    info!("=== Alfath Torrent Service Started ===");

    if let Err(e) = check_transmission().await {
        error!("Failed to connect to Transmission: {e}");
        error!("--- Service Not Ready ---");

        error!("Try To turn transmission-daemon, please restart");
        match Command::new("transmission-daemon").output() {
            Ok(_) => info!("+++ Transmission Ready +++"),
            Err(_) => error!("--- Please setup transmission ---"),
        }
    } else {
        info!("Transmission Detected.");
        info!("+++ Service Ready +++")
    }
}

fn ensure_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
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
        fs::create_dir_all(dir)?;
        println!("Directory '{}' created successfully.", dir.display());
    }

    Ok(())
}

fn init_logger(log_path: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open log file");

    let mut builder = ConfigBuilder::new();
    builder
        .set_time_format_custom(format_description!("[hour]:[minute]:[second]"))
        .set_level_padding(LevelPadding::Right);

    let config = match builder.set_time_offset_to_local() {
        Ok(b) => b.build(),
        Err(_) => builder.build(),
    };

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, config, file),
    ])
    .unwrap();

    log::info!("Logger initialized, writing to {log_path}");
}

fn check_log_size(path: &Path) -> Result<()> {
    let metadata = metadata(path)?;
    let size = metadata.len();

    info!("log Size: {size} byte");

    if size >= 100 {
        match remove_file(path) {
            Ok(_) => info!("File {} successfully remove", path.display()),
            Err(e) => error!("Faield to remove file {}: {e}", path.display()),
        }
    }
    Ok(())
}

async fn check_transmission() -> types::Result<()> {
    let url = "http://localhost:9091/transmission/rpc".parse().unwrap();
    let mut client = TransClient::new(url);

    client.session_stats().await?;

    Ok(())
}
