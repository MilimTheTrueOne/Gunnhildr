use actix_web::{get, web, App, HttpServer};
use log::info;

mod api;
mod config;
mod db;
mod reading;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // init env logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config = config::parse_config();
    let db = db::DbInterface::sqlite().await;

    info!("Server starting...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config))
            .app_data(web::Data::new(db.clone()))
            .service(reading::reading_scope())
            .service(hello)
    })
    .bind((config.binding_ip, config.port))?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> &'static str {
    "hello"
}
