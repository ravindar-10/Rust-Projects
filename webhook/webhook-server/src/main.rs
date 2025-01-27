use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WebhookNotification {
    event: String,
    data: User,
}

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    email: String,
}

async fn handle_webhook(notification: web::Json<WebhookNotification>) -> impl Responder {
    println!("Received webhook notification: {:?}", notification);
    HttpResponse::Ok().json("Webhook received.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting webhook server at http://127.0.0.1:9001/webhook");
    HttpServer::new(|| App::new().route("/webhook", web::post().to(handle_webhook)))
        .bind("127.0.0.1:9001")?
        .run()
        .await
}
