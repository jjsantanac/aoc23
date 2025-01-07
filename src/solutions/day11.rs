use core::panic;
use std::{
    char,
    collections::{HashMap, VecDeque},
};

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

    // let final_expanded = &transpose(&matrix_t)
    //     .clone()
    //     .iter()
    //     .map(|r| {
    //         r.iter()
    //             .map(|c| {
    //                 if (*c).eq(&'#') {
    //                     counter += 1;
    //                     println!("counter: {}", counter);
    //                     return char::from_digit(counter - 1, 10).unwrap();
    //                     // return counter.to_string();
    //                 } else {
    //                     return *c;
    //                     // return String::from(*c);
    //                 }
    //             })
    //             .collect::<Vec<char>>()
    //     })
    //     .collect::<Vec<Vec<char>>>();

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

    for (n, ((start_row, start_col), (target_row, target_col))) in
        galaxies_pairs.iter().map(|p| (p[0], p[1])).enumerate()
    {
        println!("start ({},{})", start_row, start_col);
        println!("target ({},{})", target_row, target_col);
        let distance = target_row.abs_diff(*start_row) + target_col.abs_diff(*start_col);
        distances.push(distance);
        println!("distance: {}", distance);
        // println!("pair no {}", n);

        // let pos = (*start_row, *start_col);

        // let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        // q.push_back(pos);
        // let mut visited: Vec<(usize, usize)> = vec![];
        // let mut distances: HashMap<(usize, usize), i32> = HashMap::from([(pos, 0)]);

        // while let Some(current) = q.pop_front() {
        //     let (i_row, i_col) = current;

        //     let col_max = final_expanded.first().unwrap().len() - 1;
        //     let row_max = final_expanded.len() - 1;

        //     // println!("current {:?}", current);

        //     if i_row.eq(target_row) && i_col.eq(target_col) {
        //         println!("start: {:?}", pos);
        //         println!("target: {:?}", (target_row, target_col));
        //         println!("distance: {}", distances.get(&current).unwrap());
        //         break;
        //     }

        //     let current_distance = match distances.get_mut(&current) {
        //         Some(x) => *x,
        //         None => 0,
        //     };

        //     // top
        //     if i_col > 0 {
        //         let top = (i_row, i_col - 1);
        //         if !visited.contains(&top) {
        //             distances.insert(top, current_distance + 1);
        //             visited.push(top);
        //             q.push_back(top);
        //         }
        //     }

        //     // bottom
        //     if i_col < col_max {
        //         let bottom = (i_row, i_col + 1);
        //         if !visited.contains(&bottom) {
        //             distances.insert(bottom, current_distance + 1);
        //             visited.push(bottom);
        //             q.push_back(bottom);
        //         }
        //     }

        //     // left
        //     if i_row > 0 {
        //         let left = (i_row - 1, i_col);
        //         if !visited.contains(&left) {
        //             distances.insert(left, current_distance + 1);
        //             visited.push(left);
        //             q.push_back(left);
        //         }
        //     }

        //     // right
        //     if i_row < row_max {
        //         let right = (i_row + 1, i_col);
        //         if !visited.contains(&right) {
        //             distances.insert(right, current_distance + 1);
        //             visited.push(right);
        //             q.push_back(right);
        //         }
        //     }
        // }
    }

    //  for (start, target) in pairs.map(|p| (p[0], p[1])) {
    // let mut pos: (usize, usize) = (usize::MAX, usize::MAX);
    // for (i, row) in final_expanded.iter().enumerate() {
    //     //match row.iter().position(|c| c.eq(&'8')) {
    //     match row
    //         .iter()
    //         .position(|c| c.eq(&char::from_digit(start, 10).unwrap()))
    //     {
    //         Some(x) => {
    //             pos = (i, x);
    //             break;
    //         }
    //         None => continue,
    //     };
    // }

    // // println!("pos {:?}", pos);

    // let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    // q.push_back(pos);
    // let mut visited: Vec<(usize, usize)> = vec![];
    // let mut distances: HashMap<(usize, usize), i32> = HashMap::from([(pos, 0)]);

    // while let Some(current) = q.pop_front() {
    //     let (i_row, i_col) = current;

    //     let col_max = final_expanded.first().unwrap().len() - 1;
    //     let row_max = final_expanded.len() - 1;

    //     // println!("current {:?}", current);

    //     // if final_expanded[i_row][i_col].eq(&'2') {
    //     if final_expanded[i_row][i_col].eq(&char::from_digit(target, 10).unwrap()) {
    //         println!("start: {:?}", start);
    //         println!("target: {:?}", target);
    //         println!("distance: {}", distances.get(&current).unwrap());
    //         break;
    //     }

    //     let current_distance = match distances.get_mut(&current) {
    //         Some(x) => *x,
    //         None => 0,
    //     };

    //     // top
    //     if i_col > 0 {
    //         let top = (i_row, i_col - 1);
    //         if !visited.contains(&top) {
    //             distances.insert(top, current_distance + 1);
    //             visited.push(top);
    //             q.push_back(top);
    //         }
    //     }

    //     // bottom
    //     if i_col < col_max {
    //         let bottom = (i_row, i_col + 1);
    //         if !visited.contains(&bottom) {
    //             distances.insert(bottom, current_distance + 1);
    //             visited.push(bottom);
    //             q.push_back(bottom);
    //         }
    //     }

    //     // left
    //     if i_row > 0 {
    //         let left = (i_row - 1, i_col);
    //         if !visited.contains(&left) {
    //             distances.insert(left, current_distance + 1);
    //             visited.push(left);
    //             q.push_back(left);
    //         }
    //     }

    //     // right
    //     if i_row < row_max {
    //         let right = (i_row + 1, i_col);
    //         if !visited.contains(&right) {
    //             distances.insert(right, current_distance + 1);
    //             visited.push(right);
    //             q.push_back(right);
    //         }
    //     }
    // }
    //}
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
