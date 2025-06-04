-- Authors table
CREATE TABLE authors
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- Books table
CREATE TABLE books
(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    publish_year SMALLINT NOT NULL,
    publish_month SMALLINT NOT NULL,
    CHECK (publish_year > 0),
    CHECK (publish_month BETWEEN 1 AND 12)
);

-- Many-to-many relationship table
CREATE TABLE authors_books
(
    author_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    PRIMARY KEY (author_id, book_id),
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE RESTRICT,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);
