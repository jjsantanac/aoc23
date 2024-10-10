use std::{fs, io::Read};

fn main() {
    let mut content = String::new();
    let _input = fs::File::open("inputs/input_day1.txt")
        .unwrap()
        .read_to_string(&mut content);

    let mut split = content.split("\n");

    let mut result: Vec<String> = vec![];

    while let Some(x) = split.next() {
        let a = match x.chars().find(|c| c.is_numeric()) {
            Some(x) => x.to_string(),
            None => break,
        };

        let b = match x.chars().rev().find(|c| c.is_numeric()) {
            Some(x) => x.to_string(),
            None => break,
        };

        let concat = a.clone() + &b;

        result.push(concat.clone());

        println!("{}", concat)
    }

    let sum = result
        .iter()
        .fold(0, |acc, v| v.parse::<i32>().unwrap() + acc);

    println!("{}", sum)
}
