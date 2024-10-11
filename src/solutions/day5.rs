use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct MappingDefinition {
    target: i64,
    source: i64,
    range: i64,
}

#[derive(Clone, Copy)]
struct Range {
    min: i64,
    range: i64,
}

pub fn solve(input: &str) {
    let mut split = input.split("\n\n");

    let seeds = split
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .trim()
        .split(" ")
        .collect::<Vec<&str>>();

    let mut mapping_matrix: Vec<Vec<MappingDefinition>> = vec![];

    for mapping in split {
        let mapping_split: Vec<&str> = mapping.split("\n").collect();
        let mut mappings: Vec<MappingDefinition> = vec![];

        for line in mapping_split {
            if line.len() == 0 {
                break;
            }

            if line.chars().any(|c| c.is_alphabetic()) {
                // println!("line: {}", line)
            } else {
                let mapping_definition = line
                    .split(" ")
                    .map(|value| value.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                mappings.push(MappingDefinition {
                    target: mapping_definition[0],
                    source: mapping_definition[1],
                    range: mapping_definition[2],
                })
            }
        }
        mapping_matrix.push(mappings)
    }

    println!("len: {}", seeds.len());

    // for chunk in seeds.chunks(2) {
    //     println!("chunk1: {}", chunk[0]);
    //     println!("chunk2: {}", chunk[1]);
    //     let min = chunk[0].parse::<i64>().unwrap();
    //     let max = min + chunk[1].parse::<i64>().unwrap() - 1;

    //     let seed_range = min..max;

    //     map_seed_location(
    //         &seed_range.collect::<Vec<i64>>(),
    //         &mapping_matrix,
    //         &mut seed_locations,
    //     );
    // }
    //
    let mut seed_range_locations: Vec<i64> = vec![];

    let i = 0;

    for s in seeds.chunks(2) {
        let seedrange = Range {
            min: s.get(0).unwrap().parse::<i64>().unwrap(),
            range: s.get(1).unwrap().parse::<i64>().unwrap(),
        };
        println!("seed min: {}", seedrange.min);
        println!("seed max: {}", seedrange.min + seedrange.range - 1);
        map_seed_range_location(seedrange, &mapping_matrix, &mut seed_range_locations);
    }

    let min_loc = seed_range_locations.iter().min().unwrap();
    println!("MIN LOC: {}", min_loc)
}

fn map_seed_location(
    seeds: &Vec<i64>,
    mapping_matrix: &Vec<Vec<MappingDefinition>>,
    seed_locations: &mut Vec<i64>,
) {
    for seed in seeds.iter() {
        // println!("seed: {}", seed);

        let mut mapped_value = seed.clone();

        for x in mapping_matrix.iter() {
            let mut matched = false;
            for y in x.iter() {
                if matched {
                    break;
                }

                let map_max = y.source + y.range - 1;
                let map_min = y.source;
                let offset = y.source - y.target;

                if mapped_value >= map_min && mapped_value <= map_max {
                    matched = true;
                    mapped_value = mapped_value - offset;
                }
            }
        }
        seed_locations.push(mapped_value);
    }
}

fn map_seed_range_location(
    seed_range: Range,
    mapping_matrix: &Vec<Vec<MappingDefinition>>,
    seed_range_locations: &mut Vec<i64>,
) {
    let mut mapped_ranges: Vec<Range> = vec![];

    let mut unmapped_ranges: VecDeque<Range> = VecDeque::new();
    unmapped_ranges.push_back(seed_range);

    for (i, mappings) in mapping_matrix.iter().enumerate() {
        while let Some(u_r) = unmapped_ranges.pop_front() {
            let unmapped_min = u_r.min;
            let unmapped_max = unmapped_min + u_r.range - 1;
            println!("current mapping level : {}", i);
            println!("current unmapped min: {}", unmapped_min);
            println!("current unmapped max: {}", unmapped_max);

            for (j, bucket) in mappings.iter().enumerate() {
                let bucket_min = bucket.source;
                let bucket_max = bucket_min + bucket.range - 1;
                println!("bucket min: {}", bucket_min);
                println!("bucket max: {}", bucket_max);

                if bucket_min <= unmapped_min
                    && unmapped_min <= bucket_max
                    && unmapped_max > bucket_max
                {
                    let diff = bucket_max - unmapped_min;

                    let unmapped_range = Range {
                        min: unmapped_min + diff + 1,
                        range: unmapped_max - bucket_max,
                    };

                    unmapped_ranges.push_back(unmapped_range);

                    let offset = bucket.source - bucket.target;

                    let mapped_range = Range {
                        min: unmapped_min - offset,
                        range: diff + 1,
                    };

                    mapped_ranges.push(mapped_range);

                    break;
                } else if bucket_min <= unmapped_max
                    && unmapped_max <= bucket_max
                    && unmapped_min < bucket_min
                {
                    let diff = unmapped_max - bucket_min;

                    let unmapped_range = Range {
                        min: unmapped_min,
                        range: u_r.range - diff - 1,
                    };

                    unmapped_ranges.push_back(unmapped_range);

                    let offset = bucket.source - bucket.target;

                    let mapped_range = Range {
                        min: bucket_min - offset,
                        range: diff + 1,
                    };

                    mapped_ranges.push(mapped_range);

                    break;
                } else if bucket_min <= unmapped_min && bucket_max >= unmapped_min {
                    let offset = bucket.source - bucket.target;

                    let mapped_range = Range {
                        min: unmapped_min - offset,
                        range: u_r.range,
                    };

                    mapped_ranges.push(mapped_range);

                    break;
                } else if unmapped_min < bucket_min && unmapped_max > bucket_max {
                    let offset = bucket.source - bucket.target;

                    let unmapped_range_1 = Range {
                        min: unmapped_min,
                        range: bucket_min - unmapped_min,
                    };

                    unmapped_ranges.push_back(unmapped_range_1);

                    let unmapped_range_2 = Range {
                        min: bucket_max + 1,
                        range: unmapped_max - bucket_max,
                    };

                    unmapped_ranges.push_back(unmapped_range_2);

                    let mapped_range = Range {
                        min: bucket_min - offset,
                        range: bucket.range,
                    };

                    mapped_ranges.push(mapped_range);

                    break;
                } else {
                    if unmapped_min == 78 && unmapped_max == 80 {}

                    if j == mappings.len() - 1 {
                        mapped_ranges.push(u_r);
                        println!("reached end and no match")
                    }
                }
            }

            // if mapped_ranges.is_empty() {
            //     println!("no match");
            //     mapped_ranges.push(u_r)
            // }
        }

        mapped_ranges
            .iter()
            .for_each(|r| unmapped_ranges.push_back(*r));
        mapped_ranges.clear();
    }

    let min = unmapped_ranges.iter().map(|u| u.min).min().unwrap();
    seed_range_locations.push(min)
}
