use crate::grid_utils::transpose;
use itertools::Itertools;
use std::cmp::Ordering;

enum Reflection {
    ColumnWise(usize),
    RowWise(usize),
    None,
}

enum Direction {
    ColumnWise,
    RowWise,
}

pub fn solve(input: &str) {
    let patterns = input.split("\n\n");

    let mut result_part1: usize = 0;
    let mut result_part2: usize = 0;

    for p in patterns {
        let grid = p
            .split("\n")
            .map(|row| row.chars().collect_vec())
            .filter(|row| !row.is_empty())
            .collect_vec();

        let first_reflection = find_reflection(&grid);
        match first_reflection {
            Reflection::ColumnWise(i) => result_part1 += i + 1,
            Reflection::RowWise(i) => result_part1 += (i + 1) * 100,
            Reflection::None => result_part1 += 0,
        }

        match find_reflection_smudged(&grid, first_reflection) {
            Reflection::ColumnWise(i) => result_part2 += i + 1,
            Reflection::RowWise(i) => result_part2 += (i + 1) * 100,
            Reflection::None => result_part2 += 0,
        }
    }

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

fn find_reflection(grid: &[Vec<char>]) -> Reflection {
    if let Some(i) = get_reflection_index(&transpose(grid), None, Direction::ColumnWise) {
        return Reflection::ColumnWise(i);
    };

    if let Some(i) = get_reflection_index(grid, None, Direction::RowWise) {
        return Reflection::RowWise(i);
    };

    Reflection::None
}

fn find_reflection_smudged(grid: &[Vec<char>], first_reflection: Reflection) -> Reflection {
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let mut repaired_grid = grid.to_owned();

            match grid[i][j] {
                '#' => repaired_grid[i][j] = '.',
                '.' => repaired_grid[i][j] = '#',
                _ => {}
            }

            if let Some(x) = get_reflection_index(
                &transpose(&repaired_grid),
                Some(&first_reflection),
                Direction::ColumnWise,
            ) {
                return Reflection::ColumnWise(x);
            };

            if let Some(x) =
                get_reflection_index(&repaired_grid, Some(&first_reflection), Direction::RowWise)
            {
                return Reflection::RowWise(x);
            };
        }
    }

    Reflection::None
}

fn get_reflection_index(
    grid: &[Vec<char>],
    first_reflection: Option<&Reflection>,
    direction: Direction,
) -> Option<usize> {
    for i in 0..grid.len() {
        let current_slice = grid[0..=i].iter().rev().collect_vec();
        let second_slice = grid[i + 1..].to_vec();

        let max_len = match current_slice.len().cmp(&second_slice.len()) {
            Ordering::Less => current_slice.len(),
            Ordering::Greater => second_slice.len(),
            Ordering::Equal => current_slice.len(),
        };

        if max_len > 0 {
            let mut reflection_found = true;
            for j in 0..max_len {
                if current_slice[j].ne(&second_slice[j]) {
                    reflection_found = false;
                    break;
                }
            }
            if reflection_found {
                match direction {
                    Direction::RowWise => {
                        if let Some(Reflection::RowWise(i_f)) = first_reflection {
                            if i_f.eq(&i) {
                                continue;
                            }
                        }
                    }
                    Direction::ColumnWise => {
                        if let Some(Reflection::ColumnWise(i_f)) = first_reflection {
                            if i_f.eq(&i) {
                                continue;
                            }
                        }
                    }
                }
                return Some(i);
            }
        }
    }
    None
}
