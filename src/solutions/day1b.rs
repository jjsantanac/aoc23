pub fn solve(input: &str) {
    let split = input.split("\n");

    let mut result: Vec<String> = vec![];

    const ALPHANUMERIC: [(&str, &str); 9] = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    for line in split {
        let mut replaced = line.to_owned();
        for (alpha, numeric) in ALPHANUMERIC.iter() {
            replaced = replaced.replace(alpha, numeric);
        }

        let a = match replaced.chars().find(|c| c.is_numeric()) {
            Some(x) => x.to_string(),
            None => break,
        };

        let b = match replaced.chars().rev().find(|c| c.is_numeric()) {
            Some(x) => x.to_string(),
            None => break,
        };

        let concat = a.clone() + &b;

        result.push(concat.clone());
    }

    let sum = result
        .iter()
        .fold(0, |acc, v| v.parse::<i32>().unwrap() + acc);

    println!("result: {}", sum)
}
