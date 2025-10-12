use alfath_torrent::torrent_handler::transmission::{handle_dir, init_transmission};
use std::fs::create_dir_all;

#[tokio::main]
async fn main() {
    create_dir_all("./log").expect("Failed Create folder log");

    let _ = init_transmission().await;
    // Creates a log file
    let _ = simple_logging::log_to_file(
        String::from("./log/alfath_torrent.log"),
        log::LevelFilter::Info,
    );
    let _ = handle_dir("torrents");
}
