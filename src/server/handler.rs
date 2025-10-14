use minijinja::{Environment, context};
use std::io::Cursor;
use tiny_http::{Header, Request, Response};
use urlencoding::decode;

#[derive(Debug)]
pub struct PageData {
    pub message: String,
}

pub fn handle_get(env: &Environment<'_>) -> Response<Cursor<Vec<u8>>> {
    let template = env.get_template("index").unwrap();
    let rendered = template.render(context! { message => ""}).unwrap();

    Response::from_string(rendered)
        .with_header(Header::from_bytes("Content-Type", "text/html").unwrap())
}

pub fn handle_post(req: &mut Request, env: &Environment<'_>) -> Response<Cursor<Vec<u8>>> {
    let mut body = String::new();

    req.as_reader().read_to_string(&mut body).unwrap();

    let input = body.strip_prefix("text=").unwrap_or("").replace("+", " ");

    let decoded = decode(&input).unwrap_or_default().into_owned();

    let data = PageData { message: decoded };

    let template = env.get_template("index").unwrap();
    let rendered = template.render(context! {message => data.message}).unwrap();

    Response::from_string(rendered)
        .with_header(Header::from_bytes("Content-Type", "text/html").unwrap())
}
