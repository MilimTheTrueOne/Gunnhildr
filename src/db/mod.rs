use log::{info, warn};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[derive(Clone)]
pub enum DataBase {
    Sqlite(sqlx::Pool<Sqlite>),
    //Postgres(sqlx::Pool<Postgres>),
}

impl DataBase {
    pub async fn sqlite() -> Self {
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
        sqlx::migrate!("migrations/sqlite")
            .run(&pool)
            .await
            .expect("Failed to apply migration!");

        Self::Sqlite(pool)
    }
}
