use std::char;

use itertools::Itertools;

pub fn solve(input: &str) {
    let split = input.split("\n");

    let mut matrix: Vec<Vec<char>> = vec![];

    for line in split {
        let line_vec: Vec<char> = line.chars().collect();

        if line_vec.len() > 0 {
            matrix.push(line_vec);
        }
    }

    println!("original");
    println!("{}", matrix_to_string(&matrix));

    println!("transposed");
    println!("{}", matrix_to_string(&transpose(&matrix)));

    let rows_to_expand: Vec<usize> = get_empty_rows(&matrix);
    let cols_to_expand: Vec<usize> = get_empty_rows(&transpose(&matrix));

    println!("rows to expand: {:?}", rows_to_expand);
    println!("cols to expand: {:?}", cols_to_expand);

    rows_to_expand
        .iter()
        .enumerate()
        .for_each(|(i, r)| matrix.insert(*r + i, vec!['.'; matrix.first().unwrap().len()]));

    println!("original rows expanded");
    println!("{}", matrix_to_string(&matrix));

    let mut matrix_t = transpose(&matrix);

    cols_to_expand
        .iter()
        .enumerate()
        .for_each(|(i, c)| matrix_t.insert(*c + i, vec!['.'; matrix_t.first().unwrap().len()]));

    println!("transposed expanded");
    println!("{}", matrix_to_string(&matrix_t));

    println!("original cols expanded");
    println!("{}", matrix_to_string(&transpose(&matrix_t)));

    let final_expanded = &transpose(&matrix_t);

    println!("result");
    println!("{}", matrix_to_string(&final_expanded));

    let galaxies = final_expanded
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, c)| {
                if c.eq(&'#') {
                    return Some((i, j));
                } else {
                    return None;
                }
            })
        })
        .collect::<Vec<(usize, usize)>>();

    let galaxies_pairs: Vec<Vec<&(usize, usize)>> = galaxies.iter().combinations(2).collect();

    println!("galaxies: {:?}", galaxies);
    println!("galaxies pairs: {:?}", galaxies_pairs.len());

    let mut distances: Vec<usize> = vec![];

    for ((start_row, start_col), (target_row, target_col)) in
        galaxies_pairs.iter().map(|p| (p[0], p[1]))
    {
        let distance = target_row.abs_diff(*start_row) + target_col.abs_diff(*start_col);
        distances.push(distance);
    }

    println!("result {}", distances.iter().sum::<usize>());
}

fn matrix_to_string(matrix: &Vec<Vec<char>>) -> String {
    return matrix
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    return (0..matrix.first().unwrap().len())
        .into_iter()
        .map(|j| {
            (0..matrix.len())
                .into_iter()
                .map(move |i| matrix[i][j])
                .collect::<Vec<char>>()
        })
        .collect();
}

fn get_empty_rows(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    return matrix
        .iter()
        .enumerate()
        .filter(|(_i, col)| col.iter().all(|c| c.eq(&'.')))
        .map(|(i, _col)| i)
        .collect();
}
