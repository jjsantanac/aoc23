use std::{fs, io::Read};

pub fn solve(input: &str) {
    let mut content = String::new();
    let _input = fs::File::open("inputs/input_day6.txt")
        .unwrap()
        .read_to_string(&mut content);
}
