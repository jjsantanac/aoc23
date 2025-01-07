enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    for (i, row) in matrix.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char.eq(&'S') {
                if i > 0 {
                    let top = matrix.get(i - 1).unwrap().get(j).unwrap();
                    println!("top: {}", top);
                }
                if j > 0 {
                    let left = row.get(j - 1).unwrap();
                    println!("left: {}", left);

                    // determine_direction(&matrix, i, j - 1, &Direction::Left);
                }
                if j < row.len() {
                    let right = row.get(j + 1).unwrap();
                    println!("right: {}", right);

                    // determine_direction_iterative(&matrix, i, j + 1, &Direction::Right);
                }
                if i < matrix.len() {
                    let bottom = matrix.get(i + 1).unwrap().get(j).unwrap();

                    println!("bottom: {}", bottom);
                    determine_direction_iterative(&matrix, i + 1, j, &Direction::Down, &(i, j));
                }
                println!("found it");
            }
        }
    }
}

fn get_current_char(grid: &Vec<Vec<char>>, i: usize, j: usize) -> char {
    let binding = Vec::<char>::new();
    let row = match grid.get(i) {
        Some(x) => x,
        None => &binding,
    };

    let c = match row.get(j) {
        Some(x) => x,
        None => &'\0',
    };

    return c.clone();
}

fn determine_direction_iterative(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    direction: &Direction,
    root_i: &(usize, usize),
) {
    let mut i_current = i;
    let mut j_current = j;
    let mut direction_current = direction;
    let mut n_steps = 1;
    let mut loop_indices: Vec<(usize, usize)> = vec![];

    loop_indices.push(*root_i);

    while !get_current_char(grid, i_current, j_current).eq(&'S') {
        n_steps += 1;
        loop_indices.push((i_current, j_current));
        let c = get_current_char(grid, i_current, j_current);

        // println!("c: {:?}", c);
        if c.eq(&'J') {
            match direction_current {
                Direction::Down => {
                    j_current -= 1;
                    direction_current = &Direction::Left;
                }
                Direction::Right => {
                    i_current -= 1;
                    direction_current = &Direction::Up;
                }
                _ => println!("direction not compatible"),
            }
        }
        if c.eq(&'7') {
            match direction_current {
                Direction::Up => {
                    j_current -= 1;
                    direction_current = &Direction::Left;
                }
                Direction::Right => {
                    i_current += 1;
                    direction_current = &Direction::Down;
                }
                _ => println!("direction not compatible"),
            }
        }
        if c.eq(&'F') {
            match direction_current {
                Direction::Up => {
                    j_current += 1;
                    direction_current = &Direction::Right;
                }
                Direction::Left => {
                    i_current += 1;
                    direction_current = &Direction::Down;
                }
                _ => println!("direction not compatible"),
            }
        }
        if c.eq(&'L') {
            match direction_current {
                Direction::Down => {
                    j_current += 1;
                    direction_current = &Direction::Right;
                }
                Direction::Left => {
                    i_current -= 1;
                    direction_current = &Direction::Up;
                }
                _ => println!("direction not compatible"),
            }
        }

        if c.eq(&'|') {
            match direction_current {
                Direction::Down => {
                    i_current += 1;
                    direction_current = &Direction::Down;
                }
                Direction::Up => {
                    i_current -= 1;
                    direction_current = &Direction::Up;
                }
                _ => println!("direction not compatible"),
            }
        }

        if c.eq(&'-') {
            match direction_current {
                Direction::Left => {
                    j_current -= 1;
                    direction_current = &Direction::Left;
                }
                Direction::Right => {
                    j_current += 1;
                    direction_current = &Direction::Right;
                }
                _ => println!("direction not compatible"),
            }
        }
    }

    let mut area: i32 = 0;

    for n in 0..loop_indices.len() {
        let (x1, y1) = loop_indices.get(n).unwrap();
        let (x2, y2) = loop_indices
            .get(n + 1)
            .unwrap_or(loop_indices.first().unwrap());

        // shoe lace
        area += (*x1 as i32 * *y2 as i32) - (*y1 as i32 * *x2 as i32);
    }

    let area_final = area.abs() / 2;

    // picks theorem
    println!("area: {}", area_final - (n_steps / 2) + 1);

    println!("steps: {}", n_steps);
    println!("steps to furthest: {}", n_steps / 2);
}
