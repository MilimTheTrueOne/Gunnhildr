#![allow(unused)]
use log::{info, warn};
use sqlx::{migrate::MigrateDatabase, PgPool, Postgres, Sqlite, SqlitePool};

/// Utility for interacting with the database
#[derive(Clone)]
pub enum DataBase {
    /// Used for Sqlite database
    Sqlite(sqlx::Pool<Sqlite>),
    /// Used for Postgres database
    Postgres(sqlx::Pool<Postgres>),
}

impl DataBase {
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
}
