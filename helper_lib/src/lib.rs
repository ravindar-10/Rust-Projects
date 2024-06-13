use std::env;

use dotenv::dotenv;
pub fn fetch_var_from_env(var_name: &str) -> String {
    dotenv().ok();
    match env::var(var_name) {
        Ok(val) => val,
        Err(e) => panic!("Error: {} {}", var_name, e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn env_test() {
        let result = fetch_var_from_env("FILE_NAME");
        assert_eq!(result, String::from("input.txt"));
    }
}
