use dotenv::dotenv;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};
use std::{env, fs};

fn main() {
    dotenv().ok();
    let input_file_path: String = fetch_var_from_env("INPUT_FILE_PATH");
    //read as string
    let contents =
        fs::read_to_string(&input_file_path).expect("Should have been able to read the file");
    println!("{}", contents);
    //read line by line
    match read_lines(&input_file_path) {
        Ok(lines) => {
            for (line_num, line) in lines.enumerate() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => panic!(
                        "{}",
                        &format!(
                            "Error in {} file in the line number {} Error: {}",
                            input_file_path, line_num, error
                        )
                    ),
                }
            }
        }
        Err(e) => println!("Error reading lines: {}", e),
    }
    let output_file_path = fetch_var_from_env("OUTPUT_FILE_NAME");
    //create a output
    let mut output_file:File = match File::create(&output_file_path) {
        Ok(val) => val,
        Err(e) => panic!("output file can not be created Error: {}", e),
    };
    //write the content in the file line by line
    match writeln!(output_file, "{}", contents) {
        Ok(_) => println!("File written successfully"),
        Err(e) => panic!("The content can not written Error: {}", e),
    };
    //Write content at once
    match  output_file.write_all(contents.as_bytes()){
       Ok(_)=>println!("File written successfully"),
       Err(e)=>panic!("The content can not written Error: {}",e)
    }
    let append_file_path = fetch_var_from_env("APPEND_FILE_NAME");
    match append_to_file(&append_file_path, &contents) {
        Ok(_) => println!("Content appended successfully"),
        Err(e) => println!("Error appending to file: {}", e),
    }
}

fn read_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = match File::open(file_path) {
        Ok(val) => val,
        Err(e) => panic!("Error in file reading: {}", e),
    };
    Ok(BufReader::new(file).lines())
}

fn append_to_file(file_path: &str, contents: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)?;

    file.write_all(contents.as_bytes())
}
fn fetch_var_from_env(var_name: &str) -> String {
    let var_value: String = match env::var(var_name) {
        Ok(val) => val,
        Err(e) => panic!("{}", &format!("{} {}", var_name, e)),
    };
    var_value
}
