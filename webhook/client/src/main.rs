use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    email: String,
}
#[derive(Serialize)]
struct WebhookNotification {
    event: String,
    data: User,
}
#[derive(Clone)]
struct AppState {
    registered_users: Arc<Mutex<Vec<User>>>,
    webhook_url: String,
    client: Arc<Client>,
}

async fn send_webhook_notification(
    client: Arc<Client>,
    url: &str,
    notification: &WebhookNotification,
) {
    match client.post(url).json(notification).send().await {
        Ok(response) => {
            println!("Webhook sent successfully: {:?}", response.status());
        }
        Err(error) => {
            eprintln!("Failed to send webhook: {:?}", error);
        }
    }
}

async fn register_user(user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.registered_users.lock().unwrap();
    users.push(user.clone());

    let notification = WebhookNotification {
        event: "user_registered".to_string(),
        data: user.into_inner(),
    };

    let webhook_url = data.webhook_url.clone();
    let client_clone = data.client.clone();
    actix_web::rt::spawn(async move {
        send_webhook_notification(client_clone, &webhook_url, &notification).await;
    });

    HttpResponse::Ok().json("User registered successfully! Notification sent via webhook.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_state = AppState {
        registered_users: Arc::new(Mutex::new(Vec::new())),
        webhook_url: "http://localhost:9001/webhook".to_string(),
        client: Arc::new(Client::new()),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_state.clone()))
            .route("/register", web::post().to(register_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
