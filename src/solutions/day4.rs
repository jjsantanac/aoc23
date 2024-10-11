use std::collections::HashMap;

pub fn solve(input: &str) {
    let split = input.split("\n");

    let mut total_score = 0;

    let mut card_copies: HashMap<usize, i32> = HashMap::new();

    let n_lines = split
        .clone()
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>()
        .len();

    // println!("first line: {}", n_lines.first().unwrap());
    // println!("last line: {}", n_lines.last().unwrap());
    //

    for (i_game, line) in split.enumerate() {
        let sub_split = line.split(":").collect::<Vec<&str>>();

        let card: Vec<&str> = match sub_split.get(1) {
            Some(c) => c.split("|").collect(),
            None => continue,
        };

        let winning_numbers = card.get(0).unwrap().trim().split(" ");
        let mut drawn_numbers: Vec<&str> = card.get(1).unwrap().trim().split(" ").collect();

        println!("Winning numbers: {}", card.get(0).unwrap());
        println!("Drawn numbers: {}", card.get(1).unwrap());

        let mut score = 0;
        let mut matching_numbers = 0;

        for wn in winning_numbers.filter(|n| n.len() > 0) {
            //     println!("current wn: {}", wn);

            //     println!("current wn len: {}", wn.len());
            match drawn_numbers.iter().position(|c| *c == wn) {
                Some(n) => {
                    if score == 0 {
                        score += 1
                    } else {
                        score *= 2;
                    }
                    drawn_numbers.remove(n);
                    matching_numbers += 1;
                }
                None => (),
            };
        }

        let game_range = i_game + 2..i_game + 2 + matching_numbers;

        println!("current game: {}", i_game + 1);
        println!("matching numbers: {}", matching_numbers);

        let current_game_copies = match card_copies.get(&(i_game + 1)) {
            Some(x) => *x,
            None => 0,
        };

        // println!("current_game_copies: {}", current_game_copies);

        for i in game_range.clone() {
            println!("won 1 + {} copy of game: {}", current_game_copies, i);
            match card_copies.get_mut(&i) {
                Some(x) => {
                    *x += 1 + current_game_copies;
                }
                None => {
                    card_copies.insert(i, 1 + current_game_copies);
                }
            }
        }

        total_score += score;
        if score > 0 {
            // println!("line score: {}", score)
        }
    }

    let n_copies: i32 = card_copies.values().sum();

    println!("TOTAL SCORE: {}", total_score);

    println!("N_COPIES: {}", n_copies);
    println!("N_LINES: {}", n_lines);
    println!("TOTAL_CARD: {}", n_lines as i32 + n_copies);
}
