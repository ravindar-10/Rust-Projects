use crate::greeter::{HelloReply, HelloRequest};
use greeter::greeter_server::{Greeter, GreeterServer};
use tonic::{transport::Server, Request, Response, Status};

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[derive(Debug, Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let req = request.into_inner();
        println!("Got a request {:?}", req);
        let reply = HelloReply {
            message: format!(
                "Hello {} your gender {} age {}!",
                req.name.to_owned(),
                req.gender.to_owned(),
                req.age.to_owned()
            ),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
