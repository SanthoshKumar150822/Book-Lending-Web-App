use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::fs;
use std::path::Path;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Safe file name
    let db_path = "book_lending.db";

    // Manually create the file if it doesn't exist
    if !Path::new(db_path).exists() {
        println!("📄 Creating new database file...");
        fs::File::create(db_path).expect("❌ Failed to create database file");
    }

    let db_url = format!("sqlite://{}", db_path);
    println!("🔗 Connecting to database at: {}", db_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Run schema init
    sqlx::query(include_str!("../sql/init.sql"))
        .execute(&pool)
        .await?;

    Ok(pool)
}
