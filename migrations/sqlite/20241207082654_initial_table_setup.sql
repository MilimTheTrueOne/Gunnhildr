CREATE TABLE IF NOT EXISTS users(
    user_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS books(
    book_id INTEGER PRIMARY KEY,
    book_title TEXT NOT NULL,
    book_description TEXT, 
    book_creation_date INT NOT NULL,
    author_id INTEGER NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(user_id) ON DELETE CASCADE

);

CREATE TABLE IF NOT EXISTS chapters(
    chapter_id INTEGER PRIMARY KEY,
    chapter_title TEXT NOT NULL,
    chapter_text TEXT NOT NULL,
    chapter_creation_date INT NOT NULL,
    book_id INTEGER NOT NULL,
    author_id INTEGER NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(book_id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(user_id) ON DELETE CASCADE
);