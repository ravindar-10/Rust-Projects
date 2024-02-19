use crate::models::Book;
use actix::Message;
use diesel::QueryResult;
use serde::{self, Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Book>>")]
pub struct FetchBook;
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Book>>")]
pub struct FetchBookAuthor {
    pub user_id: i32,
}
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Book>>")]
pub struct FetchBookId {
    pub user_id: i32,
}
#[derive(Debug, Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Vec<Book>>")]
pub struct BooksByAuthor {
    pub author: String,
}
#[derive(Debug, Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Vec<Book>>")]
pub struct BooksByPublication {
    pub publication: String,
}
#[derive(Debug, Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Vec<Book>>")]
pub struct BooksByYear {
    pub year: i32,
}

#[derive(Debug, Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Book>")]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub publication: Option<String>,
    pub year: Option<i32>,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Book>")]

pub struct UpdateBook {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub publication: Option<String>,
    pub year: Option<i32>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Book>>")]
pub struct DeleteBook {
    pub id: i32,
}
