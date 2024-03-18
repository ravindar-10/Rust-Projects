**Greet App**
This repository contains the implementation of a gRPC-based client-server application for greeting users. The application is written in Rust and utilizes the Tonic framework for gRPC communication.

**Files**
client.rs: Contains the implementation of the client application that sends requests to the gRPC server.
server.rs: Implements the gRPC server that listens for incoming requests and responds with greetings.
build.rs: A build script used to generate Rust code from the protocol buffer (proto) file.
proto/greeter.proto: Protocol buffer file defining the service and message types for the gRPC API.
**How to Build and Run**
Generate Rust Code: Run the build script to generate Rust code from the proto file.
cargo build
**Run Server: Start the gRPC server.**
cargo run --bin server
Run Client: Start the client application to send requests to the server.
cargo run --bin client