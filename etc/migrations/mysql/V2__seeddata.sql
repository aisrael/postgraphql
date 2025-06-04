
-- Sample authors
INSERT INTO authors (name) VALUES
                               ('J.K. Rowling'),
                               ('George Orwell'),
                               ('Harper Lee'),
                               ('Agatha Christie'),
                               ('Ernest Hemingway'),
                               ('Jane Austen'),
                               ('F. Scott Fitzgerald'),
                               ('Mark Twain');

-- Sample books
INSERT INTO books (title, publish_year, publish_month) VALUES
                                                           ('Harry Potter and the Sorcerer''s Stone', 1997, 6),
                                                           ('1984', 1949, 6),
                                                           ('To Kill a Mockingbird', 1960, 7),
                                                           ('Murder on the Orient Express', 1934, 1),
                                                           ('The Old Man and the Sea', 1952, 9),
                                                           ('Pride and Prejudice', 1813, 1),
                                                           ('The Great Gatsby', 1925, 4),
                                                           ('The Adventures of Tom Sawyer', 1876, 12);

-- Associate books with their authors
INSERT INTO authors_books (book_id, author_id) VALUES
                                                   (1, 1),  -- Harry Potter and the Sorcerer's Stone -> J.K. Rowling
                                                   (2, 2),  -- 1984 -> George Orwell
                                                   (3, 3),  -- To Kill a Mockingbird -> Harper Lee
                                                   (4, 4),  -- Murder on the Orient Express -> Agatha Christie
                                                   (5, 5),  -- The Old Man and the Sea -> Ernest Hemingway
                                                   (6, 6),  -- Pride and Prejudice -> Jane Austen
                                                   (7, 7),  -- The Great Gatsby -> F. Scott Fitzgerald
                                                   (8, 8);  -- The Adventures of Tom Sawyer -> Mark Twain
