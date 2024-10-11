struct NeighbourWithDiags {
    left: Option<String>,
    center: Option<String>,
    right: Option<String>,
}

pub fn solve(input: &str) {
    let split = input.split("\n");

    let mut matrix: Vec<Vec<char>> = vec![];

    for line in split {
        let line_vec: Vec<char> = line.chars().collect();

        if line_vec.len() > 0 {
            matrix.push(line_vec);
        }
    }

    let mut parts: Vec<String> = vec![];
    let mut gear_ratio: Vec<i32> = vec![];

    for (i, row) in matrix.iter().enumerate() {
        if row.len() == 0 {
            continue;
        }
        let mut current_digit = String::from("");

        let mut is_part = false;

        println!("row: {}", row.iter().collect::<String>());

        for (j, character) in row.iter().enumerate() {
            if character.eq(&'*') {
                let left_digit = get_left_neighbour(row, j).chars().rev().collect();
                let right_digit = get_right_neighbour(row, j);
                let top = get_top_neighbours(&matrix[i - 1], j);
                let bottom = get_top_neighbours(&matrix[i + 1], j);

                let (top_first, top_second) = combine_top_neigbours(top);
                let (bottom_first, bottom_second) = combine_top_neigbours(bottom);

                let r = vec![
                    left_digit,
                    right_digit,
                    top_first,
                    top_second,
                    bottom_first,
                    bottom_second,
                ];

                let filter: Vec<&String> = r.iter().filter(|d| !d.is_empty()).collect();

                if filter.len() == 2 {
                    let ratio =
                        filter[0].parse::<i32>().unwrap() * filter[1].parse::<i32>().unwrap();

                    println!("dig 1: {}", filter[0]);
                    println!("dig 2: {}", filter[1]);
                    gear_ratio.push(ratio)
                }
            }

            if character.is_numeric() {
                current_digit = String::from(current_digit + &character.to_string());

                if is_part {
                    continue;
                }
                let neighbours: Vec<char> = get_neighbours(&matrix, row, i, j);

                is_part = neighbours
                    .iter()
                    .any(|neighbour| !(*neighbour).eq(&'.') && !neighbour.is_numeric());
            } else {
                if !current_digit.is_empty() && is_part {
                    parts.push(current_digit);
                }

                current_digit = String::from("");
                is_part = false;
            }
        }

        if !current_digit.is_empty() && is_part {
            parts.push(current_digit);
        }
    }

    let sum: i32 = parts.iter().map(|p| p.parse::<i32>().unwrap()).sum();

    let gear_ratio_sum: i32 = gear_ratio.iter().sum();

    println!("SUM: {}", sum);
    println!("GEAR_RATIO_SUM: {}", gear_ratio_sum)
}

fn get_neighbours(matrix: &Vec<Vec<char>>, row: &Vec<char>, i: usize, j: usize) -> Vec<char> {
    let mut neighbours: Vec<char> = vec![];
    if i > 0 {
        neighbours.push(matrix[i - 1][j]);
        if j > 0 {
            neighbours.push(matrix[i - 1][j - 1]);
        }
        if j < row.len() - 1 {
            neighbours.push(matrix[i - 1][j + 1]);
        }
    }

    if j > 0 {
        neighbours.push(row[j - 1]);
    }

    if j < row.len() - 1 {
        neighbours.push(row[j + 1]);
    }

    if i < matrix.len() - 1 {
        neighbours.push(matrix[i + 1][j]);

        if j > 0 {
            neighbours.push(matrix[i + 1][j - 1]);
        }

        if j < row.len() - 1 {
            neighbours.push(matrix[i + 1][j + 1]);
        }
    }

    return neighbours;
}

fn get_left_neighbour(row: &Vec<char>, i: usize) -> String {
    if i == 0 {
        return String::new();
    }

    match row.get(i - 1) {
        Some(x) => {
            if x.is_numeric() {
                return x.to_string() + &get_left_neighbour(row, i - 1);
            } else {
                return String::new();
            }
        }
        None => return String::new(),
    }
}

fn get_right_neighbour(row: &Vec<char>, i: usize) -> String {
    match row.get(i + 1) {
        Some(x) => {
            if x.is_numeric() {
                return x.to_string() + &get_right_neighbour(row, i + 1);
            } else {
                return String::new();
            }
        }
        None => return String::new(),
    }
}

fn get_top_neighbours(row: &Vec<char>, i: usize) -> NeighbourWithDiags {
    match row.get(i) {
        Some(x) => {
            let c = match x.is_numeric() {
                true => Some(x.to_string()),
                false => None,
            };

            return NeighbourWithDiags {
                left: Some(get_left_neighbour(row, i).chars().rev().collect::<String>()),
                center: c,
                right: Some(get_right_neighbour(row, i)),
            };
        }
        None => {
            return NeighbourWithDiags {
                left: None,
                center: None,
                right: None,
            }
        }
    }
}

fn combine_top_neigbours(result: NeighbourWithDiags) -> (String, String) {
    match (&result.left, &result.center, &result.right) {
        (Some(left), Some(center), Some(right)) => {
            return (left.to_owned() + center + right, String::new())
        }
        (Some(left), Some(center), None) => return (left.to_owned() + center, String::new()),
        (None, Some(center), Some(right)) => return (center.to_owned() + right, String::new()),
        (None, Some(center), None) => return (center.to_owned(), String::new()),
        (Some(left), None, Some(right)) => return (left.to_owned(), right.to_owned()),
        _ => (String::new(), String::new()),
    }
}
