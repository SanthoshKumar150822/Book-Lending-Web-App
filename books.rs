use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct BookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub publication_year: i32,
    pub genre: String,
    pub copies: i32,
}

pub async fn add_book(book: BookRequest, db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO books (title, author, isbn, publication_year, genre, copies, status)
                 VALUES (?, ?, ?, ?, ?, ?, 'available')")
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .bind(book.publication_year)
        .bind(&book.genre)
        .bind(book.copies)
        .execute(db)
        .await?;
    Ok(())
}
