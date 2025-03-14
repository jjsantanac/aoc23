use std::{collections::HashMap, rc::Rc};

pub fn solve(input: &str) {
    let split = input.split("\n");
    let mut arrangements_part1: u64 = 0;
    let mut arrangements_part2: u64 = 0;

    for l in split {
        let sub_split = l.split(" ").collect::<Vec<&str>>();

        let springs = sub_split.get(0).unwrap_or(&"");
        let contiguous_group = sub_split
            .get(1)
            .unwrap_or(&"")
            .split(",")
            .collect::<Vec<&str>>();

        let contiguous_group_unfolded = contiguous_group.repeat(5);

        let mut springs_unfolded = String::new();

        for n in 1..=5 {
            springs_unfolded.push_str(&(springs));
            if n != 5 {
                springs_unfolded.push_str("?");
            }
        }

        if springs.len() > 0 && contiguous_group.len() > 0 {
            let contiguous_group_numeric = contiguous_group
                .iter()
                .map(|g| g.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let contiguous_group_numeric_unfolded = contiguous_group_unfolded
                .iter()
                .map(|g| g.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let mut cache = HashMap::<(Rc<Vec<char>>, Rc<Vec<usize>>), usize>::new();

            let arrangements = get_arrangements(
                &springs.chars().collect::<Vec<char>>(),
                &contiguous_group_numeric,
                &mut cache,
            );
            let arrangements_unfolded = get_arrangements(
                &springs_unfolded.chars().collect::<Vec<char>>(),
                &contiguous_group_numeric_unfolded,
                &mut cache,
            );

            arrangements_part1 += arrangements;
            arrangements_part2 += arrangements_unfolded;
        }
    }
    println!("arrangements part1: {}", arrangements_part1);
    println!("arrangements part2: {}", arrangements_part2);
}

fn get_arrangements(
    springs: &Vec<char>,
    window_sizes: &Vec<usize>,
    cache: &mut HashMap<(Rc<Vec<char>>, Rc<Vec<usize>>), usize>,
) -> u64 {
    let mut arrangements: u64 = 0;

    if window_sizes.len() == 0 {
        if !springs.iter().any(|s| s.eq(&'#')) {
            arrangements += 1;
        }
    } else {
        for i in 0..springs.len() {
            let mut springs_replaced = springs.clone();

            if springs_replaced[0..i].iter().any(|s| s.eq(&'#')) {
                break;
            }

            let window_size = window_sizes.get(0).unwrap();
            let window_upper_bound = i + window_size;

            if window_upper_bound <= springs_replaced.len() {
                let current_window = &mut springs_replaced[i..window_upper_bound];

                current_window.iter_mut().for_each(|s| {
                    if (*s).eq(&'?') {
                        *s = '#';
                    }
                });

                if current_window.iter().all(|s| s.eq(&'#')) {
                    let remaining_slice: Rc<Vec<char>>;
                    let remaining_window = Rc::new(window_sizes[1..].to_vec());
                    if window_upper_bound < springs_replaced.len() {
                        if springs_replaced[window_upper_bound].eq(&'#') {
                            continue;
                        }

                        springs_replaced[window_upper_bound] = '.';

                        remaining_slice =
                            Rc::new(springs_replaced[window_upper_bound + 1..].to_vec());

                        if let Some(x) =
                            cache.get(&(Rc::clone(&remaining_slice), Rc::clone(&remaining_window)))
                        {
                            arrangements += *x as u64;
                        } else {
                            let arr = get_arrangements(&remaining_slice, &remaining_window, cache);
                            arrangements += arr;
                            cache.insert(
                                (Rc::clone(&remaining_slice), Rc::clone(&remaining_window)),
                                arr as usize,
                            );
                        }
                    } else {
                        remaining_slice = Rc::new(springs_replaced[window_upper_bound..].to_vec());

                        if let Some(x) =
                            cache.get(&(Rc::clone(&remaining_slice), Rc::clone(&remaining_window)))
                        {
                            arrangements += *x as u64;
                        } else {
                            let arr = get_arrangements(&remaining_slice, &remaining_window, cache);
                            arrangements += arr;

                            cache.insert(
                                (Rc::clone(&remaining_slice), Rc::clone(&remaining_window)),
                                arr as usize,
                            );
                        }
                    }
                }
            }
        }
    }
    return arrangements;
}
