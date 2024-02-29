use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;
use std::io;
pub mod greeter {
    tonic::include_proto!("greeter");
}
struct Employee {
    name: String,
    gender: String,
    age: String,
    company: String,
    department: String,
    id: String,
    location: String,
}
impl Employee {
    fn new() -> Employee {
        Employee {
            name: String::new(),
            gender: String::new(),
            age: String::new(),
            company: String::new(),
            department: String::new(),
            id: String::new(),
            location: String::new(),
        }
    }
    fn get_input(x: &str) -> String {
        println!("{:?}", x);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
    fn collect_info(&mut self) {
        self.name = Employee::get_input("Enter your name");
        self.gender = Employee::get_input("Enter your gender");
        self.age = Employee::get_input("Enter your age");
        self.company = Employee::get_input("Enter your company name");
        self.department = Employee::get_input("Enter your department");
        self.id = Employee::get_input("Enter your employee id");
        self.location = Employee::get_input("Enter your location");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    println!("If you want to start the conversation enter START");
    let mut conv_status = String::new();
    io::stdin().read_line(&mut conv_status).unwrap();
    if conv_status.trim() == "START" {
        let mut employee = Employee::new();
        employee.collect_info();
        let request = tonic::Request::new(HelloRequest {
            name: employee.name.into(),
            gender: employee.gender.into(),
            age: employee.age.into(),
            company: employee.company.into(),
            department: employee.department.into(),
            id: employee.id.into(),
            location: employee.location.into(),
        });
        println!("Sending the Request to gRPC server ....");
        let response = client.say_hello(request).await?;
        println!("RESPONSE={:?}", response);
    } else {
        panic!("Application will not start due to wrong command")
    }
    let request = tonic::Request::new(HelloRequest {
        name: "PSN".to_string(),
        gender: "female".to_string(),
        age: 23.to_string(),
        company: "SFL".to_string(),
        department: "Engineering".to_string(),
        id: 21.to_string(),
        location: "Bengaluru".to_string(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
