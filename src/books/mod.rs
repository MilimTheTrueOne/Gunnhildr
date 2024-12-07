use actix_web::{get, web, Scope};

pub fn book_scope() -> Scope {
    web::scope("/b").service(book_view).service(chapter_view)
}

#[get("/{book}")]
async fn book_view(book: web::Path<String>) -> String {
    format!("This is the info for {book}")
}

#[get("/{book}/{chapter})")]
async fn chapter_view(path: web::Path<(String, String)>) -> String {
    format!("This is {} of {}", path.0, path.1)
}
