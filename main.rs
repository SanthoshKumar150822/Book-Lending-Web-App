mod auth;
mod books;
mod db;

use std::net::TcpListener;
use std::io::{Read, Write};
use db::init_db;
use auth::{register_user, RegisterRequest};
use books::{add_book, BookRequest};
use serde_json;

#[tokio::main]
async fn main() {
    println!("📚 Book Lending Server Starting...");

    let db = init_db().await.expect("❌ Failed to init DB");

    let listener = TcpListener::bind("127.0.0.1:3000").expect("❌ Failed to bind port");
    println!("🚀 Listening on http://127.0.0.1:3000");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let db = db.clone();
                tokio::spawn(async move {
                    let mut buffer = [0; 4096];
                    if let Ok(bytes_read) = stream.read(&mut buffer) {
                        let req = String::from_utf8_lossy(&buffer[..bytes_read]);
                        let (method, path) = parse_method_and_path(&req);

                        println!("➡️ {} {}", method, path);

                        // ✅ CORS Preflight Handling
                        if method == "OPTIONS" {
                            let response = "HTTP/1.1 204 No Content\r\n\
                                Access-Control-Allow-Origin: *\r\n\
                                Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
                                Access-Control-Allow-Headers: Content-Type\r\n\
                                Content-Length: 0\r\n\r\n";
                            stream.write_all(response.as_bytes()).unwrap();
                            return;
                        }

                        // ✅ Serve frontend HTML
                        if method == "GET" && path == "/" {
                            let html = std::fs::read_to_string("frontend/index.html")
                                .unwrap_or_else(|_| "<h1>Frontend not found</h1>".to_string());
                            write_response_html(&mut stream, 200, &html);
                        }

                        // ✅ Serve script.js
                        else if method == "GET" && path == "/script.js" {
                            let js = std::fs::read_to_string("frontend/script.js")
                                .unwrap_or_else(|_| "console.error('Missing script.js');".to_string());
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\n\r\n{}",
                                js.len(), js
                            );
                            stream.write_all(response.as_bytes()).unwrap();
                        }

                        // ✅ Register user
                        else if method == "POST" && path == "/register" {
                            if let Some(body) = extract_body(&req) {
                                println!("📥 Register JSON: {}", body);
                                if let Ok(data) = serde_json::from_str::<RegisterRequest>(&body) {
                                    match register_user(data, &db).await {
                                        Ok(_) => write_response(&mut stream, 200, "User registered"),
                                        Err(_) => write_response(&mut stream, 500, "Failed to register"),
                                    }
                                } else {
                                    write_response(&mut stream, 400, "Invalid JSON");
                                }
                            }
                        }

                        // ✅ Add book
                        else if method == "POST" && path == "/books" {
                            if let Some(body) = extract_body(&req) {
                                println!("📘 Book JSON: {}", body);
                                if let Ok(data) = serde_json::from_str::<BookRequest>(&body) {
                                    match add_book(data, &db).await {
                                        Ok(_) => write_response(&mut stream, 200, "Book added"),
                                        Err(_) => write_response(&mut stream, 500, "Failed to add book"),
                                    }
                                } else {
                                    write_response(&mut stream, 400, "Invalid book JSON");
                                }
                            }
                        }

                        // ❌ 404 fallback
                        else {
                            println!("❌ Route Not Found: {} {}", method, path);
                            write_response(&mut stream, 404, "Not Found");
                        }
                    }
                });
            }
            Err(e) => eprintln!("⚠️ Connection error: {}", e),
        }
    }
}

fn parse_method_and_path(req: &str) -> (&str, &str) {
    let mut lines = req.lines();
    if let Some(first_line) = lines.next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            return (parts[0], parts[1]);
        }
    }
    ("", "")
}

fn extract_body(req: &str) -> Option<String> {
    let parts: Vec<&str> = req.split("\r\n\r\n").collect();
    if parts.len() > 1 {
        Some(parts[1].trim().to_string())
    } else {
        None
    }
}

fn write_response(stream: &mut std::net::TcpStream, status: u16, message: &str) {
    let response = format!(
        "HTTP/1.1 {} OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        status, message.len(), message
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn write_response_html(stream: &mut std::net::TcpStream, status: u16, body: &str) {
    let response = format!(
        "HTTP/1.1 {} OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        status, body.len(), body
    );
    stream.write_all(response.as_bytes()).unwrap();
}
