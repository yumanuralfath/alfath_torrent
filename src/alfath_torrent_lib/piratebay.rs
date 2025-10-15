use crate::alfath_torrent_lib::{
    structure::{PirateBayResult, TorrentInfo},
    utils::humanize_size,
};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct ApiBayItem {
    id: Option<String>,
    name: Option<String>,
    info_hash: Option<String>,
    leechers: Option<String>,
    seeders: Option<String>,
    size: Option<String>,
    username: Option<String>,
    added: Option<String>,
    category: Option<String>,
}

pub struct PirateBay {
    api_url: String,
    limit: Option<usize>,
}

impl PirateBay {
    pub fn new() -> Self {
        Self {
            api_url: "https://apibay.org/q.php".to_string(),
            limit: None,
        }
    }

    pub fn search(&mut self, query: &str, page: u32, limit: Option<usize>) -> PirateBayResult {
        self.limit = limit;
        let start = Instant::now();
        let client = Client::new();

        let url = format!("{}?q={}", self.api_url, query);
        let response = match client.get(&url).send() {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Failed to fetch API: {e}");
                return PirateBayResult {
                    data: vec![],
                    total: 0,
                    time: 0.0,
                    current_page: Some(page),
                    total_pages: Some(1),
                };
            }
        };

        let json: Vec<ApiBayItem> = match response.json() {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Failed to parse JSON: {e}");
                return PirateBayResult {
                    data: vec![],
                    total: 0,
                    time: 0.0,
                    current_page: Some(page),
                    total_pages: Some(1),
                };
            }
        };

        let mut results = Vec::new();
        let start_index = ((page - 1) * limit.unwrap_or(50) as u32) as usize;

        for item in json.iter().skip(start_index) {
            if let Some(limit) = self.limit {
                if results.len() >= limit {
                    break;
                }
            }

            let name = item.name.clone().unwrap_or_default();
            let hash = item.info_hash.clone().unwrap_or_default();
            if name.is_empty() || hash.is_empty() {
                continue;
            }

            let size_str = match item.size.clone().unwrap_or_default().parse::<f64>() {
                Ok(bytes) if bytes > 0.0 => humanize_size(bytes),
                _ => "Unknown".to_string(),
            };

            let magnet = format!(
                "magnet:?xt=urn:btih:{}&dn={}&tr=udp://tracker.openbittorrent.com:80",
                hash,
                urlencoding::encode(&name)
            );

            let torrent_url = format!("https://itorrents.org/torrent/{hash}.torrent");

            results.push(TorrentInfo {
                name: name.clone(),
                size: size_str,
                seeders: item.seeders.clone().unwrap_or_default(),
                leechers: item.leechers.clone().unwrap_or_default(),
                category: item.category.clone().unwrap_or_default(),
                uploader: item.username.clone().unwrap_or("anonymous".to_string()),
                date: item.added.clone().unwrap_or_default(),
                url: format!(
                    "https://thepiratebay.org/description.php?id={}",
                    item.id.clone().unwrap_or_default()
                ),
                hash,
                magnet,
                torrent_url,
            });
        }

        let elapsed = start.elapsed().as_secs_f64();

        PirateBayResult {
            total: results.len(),
            time: elapsed,
            data: results,
            current_page: Some(page),
            total_pages: Some(1),
        }
    }
}

impl Default for PirateBay {
    fn default() -> Self {
        Self::new()
    }
}
