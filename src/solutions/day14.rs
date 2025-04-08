use crate::grid_utils::{to_grid, transpose};

pub fn solve(input: &str) {
    let grid = to_grid(input);

    let mut transposed = transpose(&grid);

    transposed.iter_mut().for_each(|column| {
        for i in 0..column.len() {
            if column[i].ne(&'O') {
                continue;
            }
            let mut free_spaces = 0;

            for spot in column[..i].iter().rev() {
                if spot.eq(&'.') {
                    free_spaces += 1;
                } else {
                    break;
                }
            }

            column.swap(i, i - free_spaces);
        }
    });

    let mut result = 0;

    let original = transpose(&transposed);

    for (i, row) in original.iter().enumerate() {
        result += row
            .iter()
            .filter(|s| (*s).eq(&'O'))
            .collect::<Vec<_>>()
            .len()
            * (row.len() - i);
    }

    println!("Part 1: {}", result);

    // Part 2
    // need grid utils to rotate grid 90°
    // existing code should work no problem
}
