use actix_web::{get, web, Scope};

use crate::db::DbInterface;

/// scope to handle all reading related pages
pub fn reading_scope() -> Scope {
    web::scope("/r").service(book_view).service(chapter_view)
}

/// route to view info for a specific book
#[get("/b/{book}")]
async fn book_view(book: web::Path<u32>, db: web::Data<DbInterface>) -> Option<String> {
    Some(format!(
        "This is the info for {}",
        db.get_book(*book).await.ok()?.title
    ))
}

/// view for reading a chapter
#[get("/c/{chapter})")]
async fn chapter_view(id: web::Path<u32>, db: web::Data<DbInterface>) -> Option<String> {
    let chapter = db.get_chapter(*id).await.ok()?;
    Some(format!("Text: {}", chapter.text))
}
