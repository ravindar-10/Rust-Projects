use std::{io::Write, net::TcpListener};

use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    //Without Concurrency (once at a time)
    let body1 = fetch_url("https://example.com").await;

    let body2 = fetch_url("https://httpbin.org/get").await;
    println!(
        " body {} body {}",
        body1.unwrap().len(),
        body2.unwrap().len()
    );
    // With Concurrency launch two reqwest at once. without blocking
    let task1 = fetch_url("https://example.com");
    let task2 = fetch_url("https://httpbin.org/get");
    let (res1, res2) = tokio::join!(task1, task2);
    println!("Response 1: {:?}", res1.unwrap().len());
    println!("Response 2: {:?}", res2.unwrap().len());
    // Rust starts both tasks and waits for them together -- this is concurrent I/O
}
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
fn send_data() {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap(); // Bind to port 12345 on all interfaces
    println!("Server listener on port 12345");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("new client connected: {}", stream.peer_addr().unwrap());
                let mut buf = [0; 1024];
                match stream.read(&mut buf) {
                    Ok(n) => {
                        println!("recieve message: {}", String::from_utf8_lossy(&buf[..n]));
                        let response = b"Hellow from the server!";
                        match stream.write_all(response) {
                            Ok(_) => println!("Send response"),
                            Err(e) => println!("Error sending response: {}", e),
                        }
                    }
                    Err(e) => println!("Error reading from stream: {}", e),
                }
            }
            Err(e) => println!("Erro accepting incoming connection: {}", e),
        }
    }
}

// rust smart pointers
// 1. Box<T> : this is simple smart pointer that allows you to allocate memory on the heap and ensures that the memory is
// deallocated when the Box goes out od scope.

// 2. Rc<T> : This reference-counted smart pointer allows you to share data ownership
// between multiple program parts. The Rc<T> keeps track of the number of references
// to the data and automatically deallocates the memory when the last refernce goes out of scope.

// 3. Arc<T> : This thread -safe Rc<T> version can be used in concurrent programs.
// Arc<T> stands for "Atomatically Refernce Counted" and allows multiple threads
// to access the same data simultanesouly.

// 4. Mutex<T> and RwLock<T> : these smart pointers allos you to share data between threds safely.
// Mutex<T> provides exlusive access to the data,
// while RwLock<T> allows mulitple readers or a
// single writer to access the data at the same time.
