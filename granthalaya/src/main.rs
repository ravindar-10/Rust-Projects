use actix::{Addr, SyncArbiter};
use actix_web::{web::Data, App, HttpServer};
use db_utils::{get_pool, AppState, DbActor};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;
mod actors;
mod db_utils;
mod insertable;
mod messages;
mod models;
mod schema;
mod services;
use services::{
    add_book, delete_book, get_books, get_books_by_author, get_books_by_publication,
    get_books_by_year, index, update_book,
};

fn establish_connection() -> Addr<DbActor> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&database_url);
    let db_addr: Addr<DbActor> = SyncArbiter::start(5, move || DbActor(pool.clone()));

    db_addr
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(AppState {
                db: establish_connection().clone(),
            }))
            .service(index)
            .service(get_books_by_author)
            .service(get_books_by_publication)
            .service(get_books_by_year)
            .service(get_books)
            .service(add_book)
            .service(update_book)
            .service(delete_book)
    })
    .bind("127.0.0.2:8080")?
    .run()
    .await
}
