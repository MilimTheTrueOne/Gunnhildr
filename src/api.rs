use actix_web::{
    get,
    web::{self, Json, Redirect},
    Scope,
};
use serde::Deserialize;

use crate::db::DbInterface;

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(create_book)
        .service(create_chapter)
        .service(create_user)
}

#[derive(Deserialize)]
struct BookForm {
    title: String,
    description: String,
}

#[get("/create/book")]
async fn create_book(Json(form): Json<BookForm>, db: web::Data<DbInterface>) -> Redirect {
    let id = db
        .create_book(&form.title, &form.description, todo!())
        .await
        .unwrap();

    Redirect::to(format!("r/b/{}", id)).permanent()
}

#[get("/create/chapter")]
async fn create_chapter() -> String {
    todo!()
}

#[derive(Deserialize)]
struct UserForm {
    name: String,
}

#[get("/create/user")]
async fn create_user(web::Form(form): web::Form<UserForm>, db: web::Data<DbInterface>) -> String {
    todo!()
}
