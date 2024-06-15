**Rust Firebase Realtime Database Example**  
This Rust example demonstrates how to perform CRUD (Create, Read, Update, Delete) operations with Firebase Realtime Database using  the **firebase_rs** library.  
**Overview**  
Firebase Realtime Database is a cloud-hosted NoSQL database that lets you store and sync data between your users in real-time.   This example showcases how to integrate Firebase Realtime Database into a Rust application to perform basic CRUD operations on user data.   
**Prerequisites**  
Before running the example, ensure you have the following:  
Rust installed on your system.  
A Firebase project set up with Realtime Database.  
Firebase project credentials (service account credentials or database URL).  
Setup  
Clone this repository to your local machine.  
Navigate to the root directory of the cloned repository.  
Open the Cargo.toml file and ensure the required dependencies are listed, including firebase_rs and helper_lib.  
Set up your Firebase project and obtain the database URL.  
Set the DB_URL environment variable to your Firebase database URL.  
**Usage**  
Run the application using Cargo:  
cargo run  
*The application will perform the following operations:*  
*Create a new user in the Firebase Realtime Database.*  
*Retrieve the created user by ID.*  
*Retrieve all users from the database.*  
*Update the created user's email address.*  
*Delete the created user from the database.*  
The code is structured as follows:  

src/main.rs: Entry point of the application.  
src/structs.rs: Definition of the User struct.  
src/helper.rs: Helper functions for interacting with Firebase services.  
**Dependencies**
This example utilizes the following external dependencies:
**firebase_rs:** A Rust library for interacting with Firebase Realtime Database.  
**helper_lib:** A custom library for fetching environment variables.  
**Configuration**
Ensure the following environment variable is set before running the application:  
**DB_URL:** The URL of your Firebase Realtime Database.  
**License**
This example is licensed under the MIT License. See the [LICENSE ](https://github.com/emreyalvac/firebase-rs/blob/HEAD/LICENSE)file for details.
