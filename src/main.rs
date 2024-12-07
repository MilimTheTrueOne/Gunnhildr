use actix_web::{get, App, HttpServer};
use log::info;

mod books;
mod config;
mod db;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let config = config::parse_config();
    let db = db::DataBase::sqlite().await;

    info!("Server starting...");
    HttpServer::new(move || {
        App::new()
            .app_data(config)
            .app_data(db.clone())
            .service(books::book_scope())
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
