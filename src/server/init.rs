use log::info;
use minijinja::Environment;
use tiny_http::{Header, Response, Server};

use crate::server::handler::{handle_get, handle_post};

pub fn init_server() {
    let mut env = Environment::new();

    env.add_template("index", include_str!("../frontend/index.html"))
        .unwrap();

    let server = Server::http("0.0.0.0:2007").unwrap();
    println!("✅ Server running at http://0.0.0.0:2007");
    info!("✅ Server running at http://0.0.0.0:2007");

    for mut req in server.incoming_requests() {
        let url = req.url().to_string();
        let method = req.method().clone();

        let response = match (url.as_str(), method) {
            ("/", tiny_http::Method::Post) => handle_post(&mut req, &env),
            ("/", _) => handle_get(&env),
            _ => not_found_response(),
        };

        let _ = req.respond(response);
    }
}

fn not_found_response() -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string("<h1>404 Not Found</h1>")
        .with_header(Header::from_bytes("Content-Type", "text/html").unwrap())
}
