use log::error;
use log::info;
use std::fs::read_dir;
use std::io::Result;
use std::{fs, io};
use transmission_rpc::TransClient;
use transmission_rpc::types;

pub fn handle_dir(dir: &str) -> io::Result<()> {
    match fs::create_dir(dir) {
        Ok(_) => {
            info!("Folder '{dir}' berhasil dibuat.");
            Ok(())
        }
        Err(e) => {
            error!("Gagal membuat folder '{dir}': {e}");
            Err(e)
        }
    }
}

pub fn ls() -> Result<()> {
    for entry in read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}

pub async fn init_transmission() -> types::Result<()> {
    let url = "http://localhost:9091/transmission/rpc".parse().unwrap();
    let mut client = TransClient::new(url);

    match client.session_stats().await {
        Ok(_) => println!("Transmission detect"),
        Err(_) => eprintln!("not detect"),
    }
    Ok(())
}
