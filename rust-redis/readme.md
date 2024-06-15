**Redis Basics with Rust**  
This Rust program demonstrates basic Redis operations using the redis crate, covering functionalities like   strings, hashes, lists, sets, and sorted sets.
  
**Installation and Setup**  
Make sure you have Rust installed on your system. You can install the necessary dependencies by adding the   following command
*cargo add dotenv rand radis*  
**Usage**  
Clone this repository to your local machine.  
Navigate to the project directory and create a .env file with the following content:  
*REDIS_HOSTNAME=your_redis_hostname*  
*REDIS_PASSWORD=your_redis_password*  
Replace your_redis_hostname and your_redis_password with your Redis hostname and password.  
Run the program using cargo run.  
Explore the different Redis data structures and operations implemented in the program.  
**Functionality**  
The program is divided into several functions, each demonstrating operations on a specific Redis data   structure:  
**basics:** Demonstrates basic operations like setting and getting strings, incrementing and decrementing   integer values.  
**hash:** Shows operations on hash data structures, such as setting and getting hash fields.  
**list:** Illustrates list operations like left push,right push,left pop,right pop etc.
**set:** Illustrates set operations like Add,remove,count etc.
**sorted set:** Illustrates sorted set operations like add,remove,count,member check etc.
