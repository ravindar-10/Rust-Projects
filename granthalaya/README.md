# Granthalaya - Library Management System
Granthalaya is a web application built using Actix-Web framework in Rust. It provides a set of APIs for managing books in a library. Granthalaya uses Diesel as ORM (Object-Relational Mapping) with PostgreSQL as the database.
# Features
1. Add, retrieve, update, and delete books from the library.
2. Filter books by author, publication, and publication year.
3. Proper error handling for various use cases.
4. Documentation for codebase and APIs.
# Dependencies
1. actix - The Actix framework provides a powerful, pragmatic, and extremely fast web framework for Rust.
2. actix-web - Actix-web is a simple, pragmatic, and extremely fast web framework for Rust.
3. diesel - Diesel is a safe, extensible ORM and query builder for Rust.
4. dotenv - Dotenv is a crate to load environment variables from a .env file.
5. serde - Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
6. serde_json - Serde JSON is a JSON serialization and deserialization library for Rust.
# Installation
1. Clone the repository: git clone <[repository-url](https://github.com/ravindar-10/Rust-Projects/tree/master/Rusty-Journal)>
2. Navigate to the project directory: cd granthalaya
3. Set up the environment variables by creating a .env file and adding the following configuration:
DATABASE_URL=postgres://username:password@localhost/db_name
# Install dependencies:
1. Add Diesel as a dependency with PostgreSQL and r2d2 features:  
cargo add diesel --features "diesel/postgres,diesel/r2d2"
2. Install the Diesel CLI with PostgreSQL support:
cargo install diesel_cli --no-default-features --features postgres
3. Install the Diesel CLI extension:
cargo install diesel_cli_ext
4. Set up Diesel for your project:
diesel setup
5. Generate the schema from your database:
diesel print-schema > src/schema.rs
6. Generate models from your database schema:
diesel_ext > src/models.rs
7. Run the database migrations: diesel migration run
8. Start the application: cargo run
The application will be accessible at http://127.0.0.2:8080.
# API Endpoints
1. Add a Book
a. Method: POST
b. Endpoint: /library/api/v1/books
c. Description: Add a new book to the library.
2. Get List of All Books
a. Method: GET
b. Endpoint: /library/api/v1/books
c. Description: Retrieve a list of all books in the library.
3. Update a Book
a. Method: PUT
b. Endpoint: /library/api/v1/books/:id
c. Description: Update details of a specific book in the library.
4. Delete a Book
a. Method: DELETE
b. Endpoint: /library/api/v1/books/:id
c. Description: Delete a specific book from the library.
5. Get Books by Author
a. Method: GET
b. Endpoint: /library/api/v1/books/author?author=author_name
c. Description: Retrieve a list of books by a specific author.
6. Get Books by Publication
a. Method: GET
b. Endpoint: /library/api/v1/books/publication?publication=publication_name
c. Description: Retrieve a list of books by a specific publication.
7. Get Books by Year
a. Method: GET
b. Endpoint: /library/api/v1/books/year?year=publication_year
c. Description: Retrieve a list of books published in a specific year.
# Contributing
Contributions are welcome! Feel free to submit pull requests to contribute to this project.
