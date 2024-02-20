use crate::{
    db_utils::{AppState, DbActor},
    messages::{
        BooksByAuthor, BooksByPublication, BooksByYear, CreateBook, DeleteBook, FetchBook,
        UpdateBook,
    },
    insertable::NewBook,
};
use actix::Addr;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path, Query},
    HttpResponse, Responder,
};

#[get("/")]
async fn index() -> String {
    "This is a Home Page".to_string()
}

#[get("/library/api/v1/books")]
async fn get_books(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(FetchBook).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No books found"),
        Err(_) => HttpResponse::InternalServerError().json("Unable to retrieve books"),
    }
}

#[get("/library/api/v1/books/author")]
async fn get_books_by_author(
    state: Data<AppState>,
    author_name: Query<BooksByAuthor>,
) -> impl Responder {
    let book_by_author = author_name.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(book_by_author).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No books found by author name"),
        Err(_) => HttpResponse::InternalServerError().json("Unable to retrieve books"),
    }
}

#[get("/library/api/v1/books/publication")]
async fn get_books_by_publication(
    state: Data<AppState>,
    publication_name: Query<BooksByPublication>,
) -> impl Responder {
    let book_by_pub = publication_name.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(book_by_pub).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No books found by publication name"),
        Err(_) => HttpResponse::InternalServerError().json("Unable to retrieve books"),
    }
}

#[get("/library/api/v1/books/year")]
async fn get_books_by_year(state: Data<AppState>, year: Query<BooksByYear>) -> impl Responder {
    let book_by_year = year.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(book_by_year).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No books found by year"),
        Err(_) => HttpResponse::InternalServerError().json("Unable to retrieve books"),
    }
}

#[post("/library/api/v1/books")]
async fn add_book(state: Data<AppState>, new_book: Json<NewBook>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db
        .send(CreateBook {
            title: new_book.title.clone(),
            author: new_book.author.clone(),
            publication: new_book.publication.clone(),
            year: new_book.year,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Unable to add book"),
    }
}

#[put("/library/api/v1/books/{id}")]
async fn update_book(
    state: Data<AppState>,
    id: Path<i32>,
    new_book: Json<NewBook>,
) -> impl Responder {
    let book_id = id.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db
        .send(UpdateBook {
            id: book_id,
            title: new_book.title.clone(),
            author: new_book.author.clone(),
            publication: new_book.publication.clone(),
            year: new_book.year,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to update book"),
    }
}

#[delete("/library/api/v1/books/{id}")]
async fn delete_book(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let book_id = id.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(DeleteBook { id: book_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to delete book"),
    }
}
