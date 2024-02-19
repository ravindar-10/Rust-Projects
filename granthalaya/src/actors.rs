use crate::db_utils::DbActor;
use crate::models::Book;
// use crate::schema::books::{self, dsl};
use crate::insertable::NewBook;
use crate::messages::{
    BooksByAuthor, BooksByPublication, BooksByYear, CreateBook, DeleteBook, FetchBook, UpdateBook,
};

use crate::schema::books::{dsl::*, id as book_id};
use actix::Handler;
use diesel::result::Error::NotFound;
use diesel::{self, prelude::*};
impl Handler<FetchBook> for DbActor {
    type Result = QueryResult<Vec<Book>>;
    fn handle(&mut self, _msg: FetchBook, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Book:unable to establish the connection");
        books.get_results::<Book>(&mut conn)
    }
}
impl Handler<BooksByAuthor> for DbActor {
    type Result = QueryResult<Vec<Book>>;
    fn handle(&mut self, msg: BooksByAuthor, _ctx: &mut Self::Context) -> Self::Result {
        let auther_name = msg.author;
        let mut conn = self
            .0
            .get()
            .expect("Fetch Book:unable to establish the connection");
        books
            .filter(author.eq(auther_name))
            .get_results::<Book>(&mut conn)
    }
}
impl Handler<BooksByPublication> for DbActor {
    type Result = QueryResult<Vec<Book>>;
    fn handle(&mut self, msg: BooksByPublication, _ctx: &mut Self::Context) -> Self::Result {
        let pub_name = msg.publication;
        let mut conn = self
            .0
            .get()
            .expect("Fetch Book:unable to establish the connection");
        books
            .filter(publication.eq(pub_name))
            .get_results::<Book>(&mut conn)
    }
}
impl Handler<BooksByYear> for DbActor {
    type Result = QueryResult<Vec<Book>>;
    fn handle(&mut self, msg: BooksByYear, _ctx: &mut Self::Context) -> Self::Result {
        let query_year: i32 = msg.year;
        let mut conn = self
            .0
            .get()
            .expect("Fetch Book:unable to establish the connection");
        books
            .filter(year.eq(query_year))
            .get_results::<Book>(&mut conn)
    }
}
impl Handler<CreateBook> for DbActor {
    type Result = QueryResult<Book>;
    fn handle(&mut self, msg: CreateBook, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create Book:unable to establish the connection");
        println!("msg:{:?}", msg);
        let new_book = NewBook {
            title: msg.title,
            author: msg.author,
            publication: msg.publication,
            year: msg.year,
        };
        diesel::insert_into(books)
            .values(new_book)
            .returning((book_id, title, author, publication, year))
            .get_result::<Book>(&mut conn)
    }
}
impl Handler<UpdateBook> for DbActor {
    type Result = QueryResult<Book>;
    fn handle(&mut self, msg: UpdateBook, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create Book:unable to establish the connection");
        diesel::update(books.filter(id.eq(msg.id)))
            .set((
                title.eq(msg.title),
                author.eq(msg.author),
                publication.eq(msg.publication),
                year.eq(msg.year),
            ))
            .get_result::<Book>(&mut conn)
    }
}

impl Handler<DeleteBook> for DbActor {
    type Result = QueryResult<Vec<Book>>;
    fn handle(&mut self, msg: DeleteBook, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create Book:unable to establish the connection");
        let deleted_books: Result<Vec<Book>, diesel::result::Error> =
            books.filter(id.eq(msg.id)).get_results::<Book>(&mut conn);
        let num_deleted_books: Result<usize, diesel::result::Error> =
            diesel::delete(books.filter(id.eq(msg.id))).execute(&mut conn);
        match num_deleted_books {
            Ok(del_books) => {
                if del_books == 0 {
                    Err(NotFound)
                } else {
                    deleted_books
                }
            }
            _ => Err(NotFound),
        }
    }
}
