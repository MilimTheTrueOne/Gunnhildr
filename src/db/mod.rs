#![allow(unused)]
use log::{info, warn};
use sqlx::{migrate::MigrateDatabase, PgPool, Postgres, Sqlite, SqlitePool};

pub mod models;
mod postgres;
mod sqlite;

/// Utility for interacting with the database
#[derive(Clone)]
pub enum DbInterface {
    /// Used for Sqlite database
    Sqlite(sqlx::Pool<Sqlite>),
    /// Used for Postgres database
    Postgres(sqlx::Pool<Postgres>),
}

/// Error type for handling DB related errors
pub enum DbError {
    /// No such entry found
    NotFound,
    /// Database related error
    SqlxError(sqlx::Error),
}

impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => DbError::NotFound,
            _ => DbError::SqlxError(value),
        }
    }
}
type DbResult<T> = Result<T, DbError>;

impl DbInterface {
    /// Database backed by SQLite
    pub async fn sqlite() -> Self {
        // Check if db exists, if not create it.
        if !sqlx::Sqlite::database_exists("sqlite:gunnhildr.db")
            .await
            .expect("failed to connect to db")
        {
            warn!("No SQLite database found, if this is the first time you are starting Gunnhildr, you can safely ignore this.");
            sqlx::Sqlite::create_database("sqlite:gunnhildr.db")
                .await
                .expect("failed to create SQLite Database");
            info!("Created new SQLite Database");
        }

        let pool = SqlitePool::connect("sqlite:gunnhildr.db").await.unwrap();

        // run migrations
        sqlx::migrate!("migrations/sqlite")
            .run(&pool)
            .await
            .expect("Failed to apply migration!");
        info!("Applied migrations.");

        Self::Sqlite(pool)
    }

    /// Database backed by Postgres
    pub async fn postgres(url: &str) -> Self {
        // check if database exists and create one if not
        if !sqlx::Postgres::database_exists(url)
            .await
            .expect("failed to connect to db")
        {
            warn!("No Postgres database found, if this is the first time you are starting Gunnhildr, you can safely ignore this.");
            sqlx::Postgres::create_database(url)
                .await
                .expect("failed to create Postgres Database!");
            info!("Created new Postgres Database");
        }

        let pool = PgPool::connect("url").await.unwrap();

        // run migrations
        sqlx::migrate!("migrations/postgres")
            .run(&pool)
            .await
            .expect("Failed to apply migration!");

        Self::Postgres(pool)
    }

    /// Tries to fetch a book from the database
    pub async fn get_book(&self, id: u32) -> DbResult<models::Book> {
        match self {
            DbInterface::Sqlite(pool) => sqlite::sqlite_book(pool, id).await,
            DbInterface::Postgres(pool) => todo!(),
        }
    }

    /// Tries to create a book and returns the book's id if successful
    pub async fn create_book(
        &self,
        title: &String,
        description: &String,
        author_id: u32,
    ) -> DbResult<u32> {
        match self {
            DbInterface::Sqlite(pool) => {
                sqlite::sqlite_book_create(pool, title, description, author_id).await
            }
            DbInterface::Postgres(pool) => todo!(),
        }
    }

    /// Tries to fetch a chapter from the database
    pub async fn get_chapter(&self, id: u32) -> DbResult<models::Chapter> {
        match self {
            DbInterface::Sqlite(pool) => sqlite::sqlite_chapter(pool, id).await,
            DbInterface::Postgres(pool) => todo!(),
        }
    }

    /// Tries to create a chapter and returns the chapter's id if successfu
    pub async fn create_chapter(
        &self,
        title: &String,
        text: &String,
        book_id: u32,
        author_id: u32,
    ) -> DbResult<u32> {
        match self {
            DbInterface::Sqlite(pool) => {
                sqlite::sqlite_chapter_create(pool, title, text, book_id, author_id).await
            }
            DbInterface::Postgres(pool) => todo!(),
        }
    }

    /// Tries to fetch a user from the database
    pub async fn get_user(&self, id: u32) -> DbResult<models::User> {
        match self {
            DbInterface::Sqlite(pool) => sqlite::sqlite_user(pool, id).await,
            DbInterface::Postgres(pool) => todo!(),
        }
    }

    /// Tries to create a user and returns the user's id if successful
    pub async fn create_user(&self, name: &String) -> DbResult<u32> {
        match self {
            DbInterface::Sqlite(pool) => sqlite::sqlite_user_create(pool, name).await,
            DbInterface::Postgres(pool) => todo!(),
        }
    }
}
