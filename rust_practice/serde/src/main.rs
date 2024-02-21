use std::any::Any;

use serde_json;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct User{
    name:String,
    id:u32,
    id_deleted:bool
}
#[derive(Debug,Serialize,Deserialize)]
struct NewUserRequest{
    name:String,
    id:u32,
}

fn make_user(request:NewUserRequest)->User{
    User { 
        name: request.name,
        id: request.id, 
        id_deleted: false 
    }
}
fn handle_request(json_request:&str){
    match serde_json::from_str(json_request) {
       Ok(good_request)=>{
        let new_user=make_user(good_request);
        println!("Made a new user! {new_user:#?}");

       },
       Err(e) =>{
        println!("Got an error from {json_request}: {e}");

       }       
    }
}

fn main() {
    let good_json_request=r#"{
        "name":"BillyTheUser",
        "id":6876
    }
    "#;
    let bad_json_request=r#"{
        "name":"BobbyTheUser",
        "id":6877
    }
    "#;
    // Deserialize the data
    handle_request(good_json_request);
    handle_request(bad_json_request);
    // serialize the data
   
    let serialized: String = match serde_json::to_string(&User {
        name: String::from("RAM"),
        id: 1001,
        id_deleted: true,
    }) {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            return;
        }
    };

    println!("Serialized: {}", serialized);
}
