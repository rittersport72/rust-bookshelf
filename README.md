# rust-bookshelf
Rust library for a book collection using protobuf for serialization and deserialization.

### Usage

```rust
// Create book collection with book
let mut collection = create_book_collection("programming");
let rust_book = create_book("rust", "Steve Klabnik, Carol Nichols");
collection.books.push(rust_book);

// Serialize book collection
let bytes = serialize_book_collection(&collection);
// Deserialize book collection
let object = deserialize_book_collection(&bytes);
```

## Dependency
Crate prost https://crates.io/crates/prost  
Crate prost-build https://crates.io/crates/prost-build

## References
protobuf for rust https://protobuf.dev/reference/rust/rust-generated  
protobuf rust example https://github.com/danburkert/snazzy/tree/master