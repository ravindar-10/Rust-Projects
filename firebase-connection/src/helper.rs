use firebase_rs::Firebase;
use std::collections::HashMap;
use structs::*;

use crate::structs;
pub async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let response = firebase.set::<User>(&user).await;
    return string_to_reponse(&response.unwrap().data);
}

pub async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
    return users.unwrap();
}
pub async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}
pub async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}
pub async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}
fn string_to_reponse(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// Convert a string to a User
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
