
## dotenv-reader-rust
dotenv-reader-rust is a simple library for reading environment variables from a .env file using Rust. This   library leverages the dotenv crate to load environment variables and provides a function to fetch their values conveniently.  

## Features  
1. Load environment variables from a .env file.  
2. Fetch the value of a specified environment variable.  
3. Handle errors gracefully when a variable is not found.  

## Example
Create a .env file in your project's root directory with the following content:  
FILE_NAME=input.txt  
## Functions
fetch_var_from_env(var_name: &str) -> String  
Fetches the value of the specified environment variable. If the variable is not found, it will panic with an appropriate error message.  

## Parameters:  
var_name: The name of the environment variable to fetch.  
Returns:
A String containing the value of the environment variable. 

## Contributing  
Contributions are welcome! Please feel free to submit a pull request or open an issue.  

## License  
This project is licensed under the MIT License. See the LICENSE file for details.  

## Acknowledgments  
dotenv crate for loading environment variables from a .env file.  
