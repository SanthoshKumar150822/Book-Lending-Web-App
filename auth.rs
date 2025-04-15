use serde::Deserialize;
use sqlx::SqlitePool;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub role: String,
}

pub async fn register_user(data: RegisterRequest, db: &SqlitePool) -> Result<(), sqlx::Error> {
    let hashed = hash(&data.password, DEFAULT_COST).unwrap();

    sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, ?)")
        .bind(&data.username)
        .bind(&hashed)
        .bind(&data.role)
        .execute(db)
        .await?;

    Ok(())
}
