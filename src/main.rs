use alfath_torrent::alfath_torrent_lib::init::init_service;

#[tokio::main]
async fn main() {
    init_service().await;
    //    init_server();
}
