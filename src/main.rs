#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;
use rocket::serde::json::Json;
use rocket::response::content::RawHtml;
// use postgres::Config;
use rocket::State;
use rocket::fairing::AdHoc;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
use crate::diesel::RunQueryDsl;
use rocket::form::Form;
use rocket_dyn_templates::Template;

#[macro_use] 
extern crate diesel;
use diesel::{Insertable, Queryable, table};

#[get("/")]
fn index() -> RawHtml<&'static str>{
    RawHtml(include_str!("index.html"))
}
#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(BlogPost {
        id: 1,
        title: "My first Post".to_string(),
        body: "This is my firstpost".to_string(),
        published: true,
    })
}
#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(BlogPost {
        id,
        title: "Some Title".to_string(),
        body: "Some Body".to_string(),
        published: true,
    })
}
// #[get("/")]
// fn get_all_blog_posts() -> Json<Vec<BlogPost>> {
//     Json(vec![
//         BlogPost {
//             id: 0,
//             title: "Ramayan".to_string(),
//             body: "There once lived a boy".to_string(),
//             published: true,
//         },
//         BlogPost {
//             id: 1,
//             title: "Fantastic Beast".to_string(),
//             body: "there once lived a beast".to_string(),
//             published: true,
//         },
//     ])
// }

//macro used for table creation
table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
table! {
    batchdef (batchid){
        batchid->Int4,
        batchname->Varchar,
    }
}
//define the database name
#[database("my_db")]
pub struct Db(diesel::PgConnection);
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "blog_posts"]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}
//define the database name
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "batchdef"]
struct BatchDef{
    batchid:i32,
    batchname:String,
}
#[derive(Deserialize)]
pub struct Config{
    name:String,
    age:u8,
}
#[post("/", data = "<blog_post>")]
async fn create_blog_post(
  connection: Db,
  blog_post: Json<BlogPost>,
) -> Json<BlogPost> {
  
    connection
        .run(move |c| {
            diesel::insert_into(blog_posts::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
  
}
#[post("/add_batch", data = "<batchdef>")]
async fn add_batch(
  connection: Db,
  batchdef: Json<BatchDef>,
) -> Json<BatchDef> {
  
    connection
        .run(move |c| {
            diesel::insert_into(batchdef::table)
                .values(&batchdef.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
  
}
#[get("/")]
async fn get_all_blog_posts(connection: Db) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}
#[get("/batches")]
async fn get_all_batches(connection: Db) -> Json<Vec<BatchDef>> {
    connection
        .run(|c| batchdef::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}
#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!("Hello,{}! You are {} years old.", config.name, config.age)
}
#[derive(Debug, FromForm, Serialize)]
struct Task<'r>{
   email: &'r str,
   name: String,
   password:String,
   gender:String,
}
#[post("/", data = "<task>")]
fn new(task: Form<Task<'_>>) -> Template {
    if task.email.is_empty() {
        Template::render("index", &*task)
    } else {
        Template::render("home", &*task)
    }
}
#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
    .attach(Db::fairing())
    .attach(AdHoc::config::<Config>())
    .mount("/", routes![index]).mount(
        "/blog-posts",
        routes![
            get_random_blog_post,
            get_blog_post,
            get_all_blog_posts,
            create_blog_post,
            add_batch,
            get_all_batches,
        ],
    )
}
