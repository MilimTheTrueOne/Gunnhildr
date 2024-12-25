use actix_web::{get, web, App, HttpServer};
use migration::MigratorTrait;
use sea_orm::Database;

mod entity;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let conn = Database::connect("sqlite:file:gunnhildr.db").await.unwrap();
    migration::Migrator::up(&conn, None).await.unwrap();

    HttpServer::new(move || App::new().service(greet))
        .bind(("127.0.0.1", 5678))?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn greet() -> actix_web::Result<String> {
    todo!()
}
