pub fn solve(input: &str) {
    let split = input.split("\n");

    let mut valid_games: Vec<i32> = vec![];
    let mut min_cubes_product: Vec<i32> = vec![];

    for line in split {
        if line.len() == 0 {
            continue;
        }
        let sub_split = line.split(":").collect::<Vec<&str>>();

        let games = sub_split[1].replace(";", ",");

        let mut exceeds = false;

        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;

        for n in games.split(",") {
            let n_and_color = n.trim().split(" ");
            let a = n_and_color.collect::<Vec<&str>>();
            let amount = a.get(0).unwrap();
            let color = a.get(1).unwrap();

            if (*color).eq("blue") {
                let amount_parsed = amount.parse::<i32>().unwrap();
                if !exceeds {
                    exceeds = amount_parsed > 14;
                }
                if amount_parsed > blue_max {
                    blue_max = amount_parsed.clone();
                }
                if exceeds {
                    // break;
                }
            }
            if (*color).eq("red") {
                let amount_parsed = amount.parse::<i32>().unwrap();
                if !exceeds {
                    exceeds = amount.parse::<i32>().unwrap() > 12;
                }

                if amount_parsed > red_max {
                    red_max = amount_parsed.clone();
                }
                if exceeds {
                    // break;
                }
            }
            if (*color).eq("green") {
                let amount_parsed = amount.parse::<i32>().unwrap();
                if !exceeds {
                    exceeds = amount.parse::<i32>().unwrap() > 13;
                }

                if amount_parsed > green_max {
                    green_max = amount_parsed.clone();
                }

                if exceeds {
                    // break;
                }
            }
        }

        println!("line : {}", line);
        println!("red_max: {}", red_max);
        println!("green_max: {}", green_max);
        println!("blue_max: {}", blue_max);

        min_cubes_product.push(red_max * green_max * blue_max);

        if !exceeds {
            let g: Vec<&str> = sub_split.get(0).unwrap().split(" ").collect();
            println!("{}", g[1].parse::<i32>().unwrap());
            valid_games.push(g[1].parse::<i32>().unwrap());
        }
    }

    let result: i32 = valid_games.iter().sum();
    let min_cubes_result: i32 = min_cubes_product.iter().sum();
    println!("final: {}", result);
    println!("min_cubes_final: {}", min_cubes_result);
}
