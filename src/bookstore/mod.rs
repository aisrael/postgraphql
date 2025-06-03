/// The bookstore module contains SQLx code for the sample tables, authors & books

pub mod authors;
pub mod books;
pub mod bookstore;

pub use authors::Author;
pub use books::Book;
pub use bookstore::BookStore;