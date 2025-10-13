use alfath_torrent::{server::init::init_server, torrent::init::init_service};

#[tokio::main]
async fn main() {
    init_service().await;
    init_server();
}
