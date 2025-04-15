use serde::{Deserialize, Serialize};
use argon2::{self, Config};
use rand::Rng;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UsernameAvailability {
    pub available: bool,
}

impl NewUser {
    pub async fn create(self, pool: &sqlx::SqlitePool) -> Result<User, sqlx::Error> {
        let password_hash = hash_password(&self.password)?;
        
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, role)
            VALUES (?, ?, ?, ?)
            RETURNING id, username, email, password_hash, role
            "#,
            self.username,
            self.email,
            password_hash,
            self.role
        )
        .fetch_one(pool)
        .await?;
        
        Ok(user)
    }
}

fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config)
}

pub async fn check_username_available(
    username: &str,
    pool: &sqlx::SqlitePool,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = ?) as "exists!: bool""#,
        username
    )
    .fetch_one(pool)
    .await?;
    
    Ok(!result.exists)
}