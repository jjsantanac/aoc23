use std::fs;

pub fn parse_input(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(parsed) => parsed,
        Err(e) => panic!("{}", e),
    }
}
