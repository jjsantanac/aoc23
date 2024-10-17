use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Adjacency>,
}

impl Graph {
    fn new() -> Graph {
        return Graph {
            nodes: HashMap::new(),
        };
    }

    fn get_adjacent(&self, node: &str) -> Option<&Adjacency> {
        return self.nodes.get(node);
    }

    fn add_node(&mut self, node: &str, left: &str, right: &str) -> bool {
        match self.nodes.insert(
            node.to_owned(),
            Adjacency {
                left: left.to_string(),
                right: right.to_string(),
            },
        ) {
            Some(_) => return true,
            None => return false,
        };
    }
}

#[derive(Debug)]
struct Adjacency {
    left: String,
    right: String,
}

pub fn solve(input: &str) {
    let split: Vec<&str> = input.split("\n\n").collect();

    let directions: Vec<char> = split.get(0).unwrap().chars().collect();
    let nodes: Vec<&str> = split
        .get(1)
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();

    let mut graph = Graph::new();

    for node in nodes {
        let node_split: Vec<&str> = node.split("=").collect();

        let current_node = node_split.get(0).unwrap().trim();
        let neighbours: Vec<&str> = node_split.get(1).unwrap().trim().split(",").collect();

        graph.add_node(
            current_node,
            &neighbours.get(0).unwrap().trim().replace("(", ""),
            &neighbours.get(1).unwrap().trim().replace(")", ""),
        );
    }

    let steps = get_steps_to_end("AAA", &directions, &graph);

    println!("(Part 1) Steps: {}", steps);

    let mut starting_nodes: Vec<i64> = graph
        .nodes
        .keys()
        .filter(|key| key.chars().last().unwrap().eq(&'A'))
        .map(|node| get_steps_to_end(node, &directions, &graph))
        .collect();

    let mut lcm_current = starting_nodes.remove(0);

    for node in starting_nodes.iter() {
        lcm_current = lcm(&lcm_current, node);
    }

    println!("(Part 2) Steps: {}", lcm_current)
}

fn get_steps_to_end(start_node: &str, directions: &Vec<char>, graph: &Graph) -> i64 {
    let mut is_dest = false;
    let mut current = start_node;

    let mut steps: i64 = 0;

    while !is_dest {
        for direction in directions.iter() {
            match graph.get_adjacent(current) {
                Some(x) => {
                    if current.chars().last().unwrap().eq(&'Z') {
                        is_dest = true;
                        break;
                    }

                    if direction.eq(&'L') {
                        current = &x.left;
                    };

                    if direction.eq(&'R') {
                        current = &x.right;
                    }
                    steps += 1;
                }
                None => {}
            }
        }
    }
    return steps;
}

fn gcd(a: &i64, b: &i64) -> i64 {
    if b.eq(&0) {
        return *a;
    } else {
        return gcd(b, &(a % b));
    }
}

fn lcm(a: &i64, b: &i64) -> i64 {
    return (a * b).abs() / gcd(a, b);
}
