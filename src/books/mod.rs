use actix_web::{get, web, Scope};

/// scope to handle all reading related pages
pub fn reading_scope() -> Scope {
    web::scope("/b").service(book_view).service(chapter_view)
}

/// route to view info for a specific book
#[get("/{book}")]
async fn book_view(book: web::Path<String>) -> String {
    format!("This is the info for {book}")
}

/// view for reading a chapter
#[get("/{book}/{chapter})")]
async fn chapter_view(path: web::Path<(String, String)>) -> String {
    format!("This is {} of {}", path.0, path.1)
}
