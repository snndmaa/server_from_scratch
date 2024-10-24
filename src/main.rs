#![allow(dead_code)]    // For dev only disable console warnings for deadcode // The ! makes sure it applies to the entire file
use server::Server;
use http::Request;
use http::Method;

mod server;
mod http;

fn main() {
    let server: Server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

/* 
    GET /user?id=10 HTTP/1.1\r\n
    HEADERS \r\n
    BODY
*/