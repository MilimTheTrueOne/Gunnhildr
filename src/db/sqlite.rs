//! Module containing database code for SQLite
use super::models::{Book, Chapter, User};
use super::{DbError, DbResult};
use serde::de;
use sqlx::{Pool, Row, Sqlite};

pub async fn sqlite_book(pool: &Pool<Sqlite>, id: u32) -> DbResult<Book> {
    let row = sqlx::query("SELECT * FROM users WHERE user_id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let book = Book {
        id,
        title: row.get("book_title"),
        description: row.get("book_description"),
        creation_date: row.get("book_creation_date"),
        author_id: row.get("author_id"),
    };

    Ok(book)
}

pub async fn sqlite_book_create(
    pool: &Pool<Sqlite>,
    title: String,
    description: String,
    author_id: u32,
) -> DbResult<u32> {
    let id = sqlx::query("INSERT INTO books (title, description, author_id, book_creation_date) VALUES ( ?1, ?2, ?3, ?4 )")
        .bind(title)
        .bind(description)
        .bind(author_id)
        .bind(chrono::Local::now().timestamp())
        .execute(pool)
        .await?
        .last_insert_rowid() as u32;
    Ok(id)
}

pub async fn sqlite_chapter(pool: &Pool<Sqlite>, id: u32) -> DbResult<Chapter> {
    let row = sqlx::query("SELECT * FROM chapters WHERE chapter_id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let chapter = Chapter {
        id,
        title: row.get("chapter_title"),
        text: row.get("chapter_text"),
        creation_date: row.get("chapter_creation_date"),
        book_id: row.get("book_id"),
        author_id: row.get("author_id"),
    };
    Ok(chapter)
}

pub async fn sqlite_chapter_create(
    pool: &Pool<Sqlite>,
    title: String,
    text: String,
    book_id: u32,
    author_id: u32,
) -> DbResult<u32> {
    let id = sqlx::query("INSERT INTO chapters (chapter_title, chapter_text, book_id, author_id, chapter_creation_date) VALUES ( ?1, ?2, ?3, ?4, ?5 )")
        .bind(title)
        .bind(text)
        .bind(book_id)
        .bind(author_id)
        .bind(chrono::Local::now().timestamp())
        .execute(pool)
        .await?
        .last_insert_rowid() as u32;

    Ok(id)
}

pub async fn sqlite_user(pool: &Pool<Sqlite>, id: u32) -> DbResult<User> {
    let row = sqlx::query("SELECT * FROM users WHERE user_id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let user = User {
        id,
        name: row.get("name"),
    };

    Ok(user)
}

pub async fn sqlite_user_create(pool: &Pool<Sqlite>, name: String) -> DbResult<u32> {
    let id = sqlx::query("INSERT INTO users (name) VALUES ( ? )")
        .bind(name)
        .execute(pool)
        .await?
        .last_insert_rowid() as u32;
    Ok(id)
}
