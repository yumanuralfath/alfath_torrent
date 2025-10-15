use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TorrentInfo {
    pub name: String,
    pub size: String,
    pub seeders: String,
    pub leechers: String,
    pub category: String,
    pub uploader: String,
    pub date: String,
    pub url: String,
    pub hash: String,
    pub magnet: String,
    pub torrent_url: String,
}

#[derive(Debug, Serialize)]
pub struct PirateBayResult {
    pub data: Vec<TorrentInfo>,
    pub total: usize,
    pub time: f64,
    pub current_page: Option<u32>,
    pub total_pages: Option<u32>,
}
