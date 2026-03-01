use std::io::Cursor;

use prost::Message;

// Include the `books` module, which is generated from books.proto
pub mod books {
    include!(concat!(env!("OUT_DIR"), "/bookshelf.books.rs"));
}

pub fn create_book(title: &str, author: &str) -> books::Book {
    let mut book = books::Book::default();
    book.title = title.to_string();
    book.author = author.to_string();
    book.publication_year = 2023;
    book.set_cover(books::book::Cover::Paperback);
    book
}

pub fn create_book_collection(topic: &str) -> books::BookCollection {
    let mut collection = books::BookCollection::default();
    collection.topic = topic.to_string();
    collection
}

pub fn serialize_book_collection(collection: &books::BookCollection) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(collection.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    collection.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_book_collection(buf: &[u8]) -> Result<books::BookCollection, prost::DecodeError> {
    books::BookCollection::decode(&mut Cursor::new(buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        let mut collection = create_book_collection("programming");

        let java_book = create_book("java", "any");
        let rust_book = create_book("rust", "any");
        let mut c_book = create_book("c", "any");
        c_book.set_cover(books::book::Cover::Hardcover);
        c_book.publication_year = 1995;

        collection.books.push(java_book);
        collection.books.push(rust_book);
        collection.books.push(c_book);

        let bytes = serialize_book_collection(&collection);

        let object = deserialize_book_collection(&bytes);

        assert_eq!(collection, object.unwrap());
    }
}
