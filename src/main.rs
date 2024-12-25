use actix_web::{get, web, App, HttpServer};
use log::info;
use migration::MigratorTrait;
use sea_orm::Database;

mod entity;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let conn = Database::connect("sqlite:file:gunnhildr.db").await.unwrap();
    migration::Migrator::up(&conn, None).await.unwrap();

    HttpServer::new(move || App::new().service(greet))
        .bind(("127.0.0.1", 5678))?
        .run()
        .await?;

    info!("Server shut down! Goodbye!");
    Ok(())
}

#[get("/")]
async fn greet() -> actix_web::Result<String> {
    todo!()
}
