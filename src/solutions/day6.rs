use std::i64;

use regex::Regex;

pub fn solve(input: &str) {
    let split: Vec<&str> = input.split("\n").collect();

    let race_times = get_values_from_line(&split, 0);
    let record_distances = get_values_from_line(&split, 1);

    let mut result: Vec<i64> = vec![];

    for (time, distance) in race_times.iter().zip(record_distances.to_owned()) {
        result.push(get_ways_to_win(time, &distance))
    }

    let ways_to_win = get_ways_to_win(&join_number(&race_times), &join_number(&record_distances));
    let product = result.iter().fold(1, |acc, r| acc * r);

    println!("(Part 1) PRODUCT: {}", product);
    println!("(Part 2) WAYS TO WIN: {}", ways_to_win);
}

fn get_values_from_line(input: &Vec<&str>, line_n: usize) -> Vec<i64> {
    let line = input
        .get(line_n)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .trim();

    return Regex::new(" +")
        .unwrap()
        .replace_all(line, " ")
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}

fn get_ways_to_win(race_time: &i64, distance_record: &i64) -> i64 {
    let max_hold = match race_time % 2 {
        0 => race_time / 2,
        _ => ((race_time + 1) / 2) - 1,
    };

    let mut n_ways_to_win = 0;

    for n in (0..=max_hold).rev() {
        let distance_covered = match n {
            n if n == 0 => *race_time,
            _ => n * (race_time - n),
        };

        if distance_covered <= *distance_record {
            break;
        } else {
            n_ways_to_win += 1;
        }
    }
    if race_time % 2 != 0 {
        return n_ways_to_win * 2;
    } else {
        return n_ways_to_win * 2 - 1;
    }
}

fn join_number(numbers: &Vec<i64>) -> i64 {
    return numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
}
