use actix_web::{get, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    info!("Server starting...");
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 6767))?
        .run()
        .await
}

#[get("/hello")]
async fn hello() -> &'static str {
    "hello"
}
