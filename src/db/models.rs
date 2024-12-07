pub struct Chapter {
    pub id: u32,
    pub title: String,
    pub text: String,
    pub creation_data: String,
    pub book_id: u32,
    pub author_id: u32,
}

pub struct Book {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub creation_date: String,
    pub author_id: u32,
}

pub struct User {
    pub id: u32,
    pub name: String,
}
