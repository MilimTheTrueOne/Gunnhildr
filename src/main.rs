use actix_web::{get, App, HttpServer};
use log::info;

mod config;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let config = config::parse_config();

    info!("Server starting...");
    HttpServer::new(|| App::new().service(hello))
        .bind((config.binding_ip, config.port))?
        .run()
        .await
}

#[get("/hello")]
async fn hello() -> &'static str {
    "hello"
}
