use alfath_torrent::torrent::init::init_service;

#[tokio::main]
async fn main() {
    init_service().await;
}
