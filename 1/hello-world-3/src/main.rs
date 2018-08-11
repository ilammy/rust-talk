extern crate actix_web;
extern crate rand;

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

use actix_web::{http, server, App, HttpRequest, HttpResponse};

fn index(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"<h1>Here's your kitten</h1><img src="/random-kitten"/>"#)
}

fn get_kitten(_: HttpRequest) -> HttpResponse {
    match get_kitten_image() {
        Ok(image_bytes) => HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(image_bytes),
        Err(error) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Failed to get a kitten: {}", error)),
    }
}

fn get_kitten_image() -> io::Result<Vec<u8>> {
    if let Some(path) = select_kitten_image() {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no kittens in current directory",
        ))
    }
}

fn select_kitten_image() -> Option<PathBuf> {
    let paths = fs::read_dir(".")
        .expect("failed to read pwd")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(is_jpeg)
        .collect::<Vec<_>>();

    if paths.len() > 0 {
        Some(paths[rand::random::<usize>() % paths.len()].clone())
    } else {
        None
    }
}

fn is_jpeg(path: &PathBuf) -> bool {
    path.to_str()
        .map(|path| path.ends_with(".jpg") || path.ends_with(".jpeg"))
        .unwrap_or(false)
}

fn p404(_: &HttpRequest) -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(r#"<h1>Error 404: Kittens Not Found</h1>"#)
}

fn main() {
    let server = server::new(|| {
        App::new()
            .route("/", http::Method::GET, index)
            .route("/random-kitten", http::Method::GET, get_kitten)
            .default_resource(|r| r.f(p404))
    });

    server
        .bind("0.0.0.0:8080")
        .expect("Can't listen on 8080 port")
        .run();
}
