use crate::grid_utils::transpose;
use itertools::Itertools;
use std::cmp::Ordering;

pub fn solve(input: &str) {
    let patterns = input.split("\n\n");

    let mut result: usize = 0;

    for p in patterns {
        let grid = p
            .split("\n")
            .map(|row| row.chars().collect_vec())
            .filter(|row| !row.is_empty())
            .collect_vec();

        result += find_reflection(&grid);
    }

    println!("Part 1: {}", result)
}

fn find_reflection(grid: &Vec<Vec<char>>) -> usize {
    if let Some(i) = get_reflection_index(&transpose(grid)) {
        return i + 1;
    };

    if let Some(i) = get_reflection_index(grid) {
        return (i + 1) * 100;
    };

    return 0;
}

fn get_reflection_index(grid: &Vec<Vec<char>>) -> Option<usize> {
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
                return Some(i);
            }
        }
    }
    return None;
}
