use helper::{delete_user, get_user, get_users, set_user, update_user};
use structs::*;
mod helper;
mod structs;
use firebase_rs::Firebase;
use helper_lib::fetch_var_from_env;
#[tokio::main]
async fn main() {
    let db_url = fetch_var_from_env("DATABASEURL");
    let firebase = match Firebase::new(&db_url) {
        Ok(val) => val,
        Err(e) => panic!("firebase database can not be connected {}", e),
    };
    //Create the user
    let user = User {
        name: "Jhon Doe".to_string(),
        age: 25,
        email: "jhon.doe@mail.com".to_string(),
    }; 
    let response = set_user(&firebase, &user).await;
    println!("set_user response {:?}", response);
    println!("User created");
    //get user by id
    let user_id = response.name;
    let response = get_user(&firebase, &user_id).await;
    println!("user for the user_id: {} is : {:?}", user_id, response);
    // Get all users
    let users = get_users(&firebase).await;
    println!("All Users {:?}", users);
    let mut user = get_user(&firebase, &user_id).await;
    println!("response: {:?}", response);
    // Update the user
    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("updated user :{:?}", updated_user);
    // Delete the user
    let _response: () = delete_user(&firebase, &user_id).await;
    println!("User deleted");
}
