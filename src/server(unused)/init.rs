use log::info;
use minijinja::{Environment, context};
use std::collections::HashMap;
use tiny_http::{Header, Response, Server};
use urlencoding::decode;

use crate::torrent::piratebay::{self, PirateBay, PirateBayResult};

pub fn init_server() {
    let mut env = Environment::new();

    env.add_template("index", include_str!("../frontend/index.html"))
        .unwrap();

    let server = Server::http("0.0.0.0:2007").unwrap();
    println!("✅ Server running at http://0.0.0.0:2007");
    info!("✅ Server running at http://0.0.0.0:2007");

    for request in server.incoming_requests() {
        let url = request.url().to_string();

        // =========================
        // Route: /search?q=...
        // =========================
        if url.starts_with("/search") {
            let query_params = parse_query_params(&url);
            let query = query_params.get("q").cloned().unwrap_or_default();
            let decoded = decode(&query).unwrap_or_default();

            let page = query_params
                .get("page")
                .and_then(|p| p.parse::<u32>().ok())
                .unwrap_or(1);

            let limit = query_params
                .get("limit")
                .and_then(|l| l.parse::<usize>().ok())
                .unwrap_or(50);

            let mut scraper = PirateBay::new();
            let results: PirateBayResult = scraper.search(&decoded, page, Some(limit));

            let template = env.get_template("index").unwrap();
            let rendered = template
                .render(context! {
                    q => decoded,
                    results => results.data,
                    total => results.total,
                    time => results.time,
                    current_page => results.current_page,
                    total_pages => results.total_pages,
                })
                .unwrap();

            let response = Response::from_string(rendered)
                .with_header(Header::from_bytes("Content-Type", "text/html").unwrap());
            request.respond(response).unwrap();
        }
        // =========================
        // Route: /
        // =========================
        else {
            let template = env.get_template("index").unwrap();
            let rendered = template
                .render(context! {
                    q => "",
                    results => Vec::<piratebay::TorrentInfo>::new(),
                    total => 0,
                    time => 0.0,
                    current_page => None::<u32>,
                    total_pages => None::<u32>,
                })
                .unwrap();

            let response = Response::from_string(rendered)
                .with_header(Header::from_bytes("Content-Type", "text/html").unwrap());
            request.respond(response).unwrap();
        }
    }
}

/// Utility sederhana untuk mem-parse query string ke dalam HashMap
fn parse_query_params(url: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    if let Some(query_part) = url.split('?').nth(1) {
        for kv in query_part.split('&') {
            if let Some((key, value)) = kv.split_once('=') {
                params.insert(key.to_string(), value.to_string());
            }
        }
    }
    params
}
