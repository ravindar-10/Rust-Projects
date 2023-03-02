#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use postgres::Config;
use rocket::State;
use serde::{Deserialize, Serialize};
// #[get("/config")]
// fn get_config(config: &State<Config>) -> String {
//     format!("Hello,{}! You are {} years old.", config.name, config.age)
// }
#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}
#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
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
#[get("/")]
fn get_all_blog_posts() -> Json<Vec<BlogPost>> {
    Json(vec![
        BlogPost {
            id: 0,
            title: "Ramayan".to_string(),
            body: "There once lived a boy".to_string(),
            published: true,
        },
        BlogPost {
            id: 1,
            title: "Fantastic Beast".to_string(),
            body: "there once lived a beast".to_string(),
            published: true,
        },
    ])
}
#[post("/", data = "<blog_post>")]
fn create_blog_post(blog_post: Json<BlogPost>) -> Json<BlogPost> {
    blog_post
}
#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket.mount("/", routes![index]).mount(
        "/blog-posts",
        routes![
            get_random_blog_post,
            get_blog_post,
            get_all_blog_posts,
            create_blog_post
        ],
    )
}
