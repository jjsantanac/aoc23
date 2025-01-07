pub fn solve(input: &str) {
    let split = input.split("\n").filter(|l| l.len() > 0).map(|l| {
        l.split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut result = 0;
    let mut result2 = 0;

    for l in split.clone() {
        let mut diffs: Vec<i32> = l.clone();
        let mut diff_last = 0;

        println!("line: {:?}", l);
        while !diffs.iter().all(|d| d.eq(&0)) {
            let mut t: Vec<i32> = vec![];

            println!("diffs: {:?}", diffs);
            for (i, n) in diffs.iter().enumerate() {
                if let Some(m) = diffs.get(i + 1) {
                    t.push(m - n)
                }
            }

            println!("t: {:?}", t);

            diffs = t.clone();
            diff_last += t.last().unwrap();
        }
        result += l.last().unwrap() + diff_last;
        println!("total diff last: {:?}", diff_last)
    }

    println!("result a: {}", result);

    for l in split {
        let mut diffs: Vec<i32> = l.clone();
        let mut diffs_collected: Vec<i32> = vec![];

        println!("line: {:?}", l);
        while !diffs.iter().all(|d| d.eq(&0)) {
            let mut t: Vec<i32> = vec![];

            println!("diffs: {:?}", diffs);
            for (i, n) in diffs.iter().enumerate() {
                if let Some(m) = diffs.get(i + 1) {
                    t.push(m - n)
                }
            }
            diffs_collected.push(*diffs.first().unwrap());

            println!("t: {:?}", t);

            diffs = t.clone();
        }
        println!("diffs collected: {:?}", diffs_collected);
        let result_b = diffs_collected.iter().rev().fold(0, |acc, d| d - acc);
        // let result_b_test: Vec<i32> = diffs_collected
        //     .iter()
        //     .rev()
        //     .scan(0, |acc, &d| {
        //         println!("d: {}", d);
        //         println!("acc: {}", acc);
        //         *acc = d - *acc;

        //         Some(*acc)
        //     })
        //     .collect();
        result2 += result_b;
        println!("extrapolated: {:?}", result_b)
    }

    println!("result b: {}", result2);
}
