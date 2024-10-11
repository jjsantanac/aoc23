pub fn solve(input: &str) {
    let mut split = input.split("\n");

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
    }

    let sum = result
        .iter()
        .fold(0, |acc, v| v.parse::<i32>().unwrap() + acc);

    println!("{}", sum)
}
