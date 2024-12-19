use actix_web::{get, App, HttpServer};
use anyhow::Ok;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 5678))?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn greet() -> actix_web::Result<String> {
    todo!()
}
