use core::panic;

use dotenv::*;
use futures_util::TryStreamExt;
use helper_lib::fetch_var_from_env;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use tokio::*;
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    dotenv().ok();
    let uri = fetch_var_from_env("CONNECTION_STRING");

    // Create a new client and connect to the server
    let client = match Client::with_uri_str(uri).await {
        Ok(val) => val,
        Err(e) => panic!("connection can not be established Error: {}", e),
    };

    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let collection: Collection<Document> = database.collection("movies");

    // Find a movie based on the title value
    let my_movie = match collection
        .find_one(doc! { "title": "The Perils of Pauline" }, None)
        .await
    {
        Ok(val) => val,
        Err(e) => panic!("query can not be executed Error: {}", e),
    };
    // Print the document
    println!("Found a movie: {:#?}", my_movie);
    // Find Multiple docs
    let filter = doc! { "year": 1925 };
    let movies_for_1925 = match collection.find(filter, None).await {
        Ok(val) => val,
        Err(e) => panic!("the query can not be executed Error: {}", e),
    };
    println!("Found a movie for 1925:\n{:#?}", movies_for_1925);
    // Add a docs
    let doc = doc! {
        "title": "Mistress America", "type": "movie"
    };

    let result = match collection.insert_one(doc, None).await {
        Ok(val) => val,
        Err(e) => panic!("the query can not be executed Error: {}", e),
    };
    println!("inserted result :{:#?}", result);
    //insert multiple docs
    let docs = vec![
        doc! { "title": "Friends With Money", "runtime": 88 },
        doc! { "title": "Please Give", "runtime": 90 },
        doc! { "title": "You Hurt My Feelings", "runtime": 93 },
    ];

    let result = match collection.insert_many(docs, None).await {
        Ok(val) => val,
        Err(e) => panic!("the query can not be executed Error: {}", e),
    };
    println!("inserted result :{:#?}", result);
    //update a docs
    let filter = doc! { "title": "Burn After Reading"};
    let update = doc! {
            "$set": doc!{ "num_mflix_comments": 1 }
    };

    let result = match collection.update_one(filter, update, None).await {
        Ok(val) => val,
        Err(e) => panic!("the query can not be executed Error: {}", e),
    };
    println!("inserted result :{:#?}", result);
    //Access Data from a Cursor as an Array
    let cursor = collection
        .find(doc! { "title": "Secrets & Lies" }, None)
        .await?;

    let results: Vec<Document> = cursor.try_collect().await?;
    println!("array result :{:#?}", results);
    Ok(())
}
