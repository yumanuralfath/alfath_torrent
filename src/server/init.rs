use log::info;
use std::fs::File;
use tiny_http::{Header, Response, Server};

pub fn init_server() {
    let server = Server::http("0.0.0.0:2007").unwrap();
    info!("Server Running on port 2007");
    println!("Server Succes Run on Port 2007");

    for request in server.incoming_requests() {
        let url_path = request.url();
        let file_path = if url_path == "/" {
            "public/index.html"
        } else {
            &format!("public{url_path}")
        };

        match File::open(file_path) {
            Ok(file) => {
                let content_type = if file_path.ends_with(".css") {
                    "text/css"
                } else if file_path.ends_with(".js") {
                    "application/javascript"
                } else if file_path.ends_with(".html") {
                    "text/html"
                } else {
                    "application/octet-stream"
                };

                let response = Response::from_file(file)
                    .with_header(Header::from_bytes(&b"Content-Type"[..], content_type).unwrap());
                request.respond(response).unwrap();
            }
            Err(_) => {
                let response = Response::from_string("404 Not Found").with_status_code(404);
                request.respond(response).unwrap();
            }
        }
    }
}
