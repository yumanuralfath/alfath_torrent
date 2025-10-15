use log::LevelFilter;
use log::{error, info};
use simplelog::*;
use std::fs::OpenOptions;
use std::process::Command;

use crate::alfath_torrent_lib::folder::{
    get_log_dir, get_torrents_dir, set_log_dir, set_torrents_dir,
};
use crate::alfath_torrent_lib::utils::{check_log_size, check_transmission, ensure_dir};

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

fn init_logger(log_path: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open log file");

    let mut builder = ConfigBuilder::new();
    builder
        .set_time_format_custom(format_description!(
            "[day]/[month]/[year] [hour]:[minute]:[second]"
        ))
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
