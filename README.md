# Book-Lending-Web-App

## Tech Stack
- *Language:* Rust  
- *Database:* SQLite  
- *Frontend:* HTML + JavaScript  
- *Repository:* [Book Lending Web App](https://github.com/SanthoshKumar150822/Book-Lending-Web-App)



## Overview

This is a minimalist full-stack *Book Lending System* developed as part of an internship screening task. The system enables:

- *User Registration* with role-based access (admin, lender)
- *Book Addition* and availability tracking
- Simple *frontend* using HTML and JavaScript
- *Pure Rust backend* handling HTTP requests without frameworks
- Persistent storage via *SQLite*, with auto-initialized schema



## Architecture

### Backend – Rust
- Built using std::net::TcpListener (manual HTTP parsing)
- Async handling with *Tokio*
- Password hashing via *bcrypt*
- JSON (de)serialization using *serde*
- Database interaction with *sqlx*

### Frontend – HTML + JS
- Basic forms for:
  - User Registration
  - Book Addition
- Uses fetch() to POST JSON payloads
- Plain HTML for simplicity, browser-compatible


## Dependencies

Listed in Cargo.toml:
- serde, serde_json – Serialization
- tokio – Asynchronous runtime
- bcrypt – Password hashing
- sqlx – Async database interaction


## Database Schema

*Defined in* sql/init.sql. Auto-loaded on first run.

### Tables:
- users: id, username, password_hash, role (admin or lender)
- books: title, author, isbn, publication_year, genre, copies
- lendings: Tracks borrow/return history


## REST API Documentation

### POST /register
Register a new user.

*Request:*
```json
{
  "username": "john_doe",
  "password": "secure123",
  "role": "admin"
}



### Execution steps 
Cargo build
Cargo run
