use std::collections::HashMap;

// TODO
// 1. parse nodes [X]
// 2. create graph from nodes as a hashmap
// 3. hashmap keys represent graph nodes
// 4. hashmap values represent adjacent nodes -> left, right
// 5. traverse graph by given directions -> iterate through graph nodes
// 6. we start at AAA and want to reach ZZZ
// 7. in case we deplete our given directions we just start over until the ZZZ is reached
// 8. track number of steps as well
//
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
        println!("node: {}", node);
        let node_split: Vec<&str> = node.split("=").collect();

        let current_node = node_split.get(0).unwrap().trim();
        let neighbours: Vec<&str> = node_split.get(1).unwrap().trim().split(",").collect();

        graph.add_node(
            current_node,
            &neighbours.get(0).unwrap().trim().replace("(", ""),
            &neighbours.get(1).unwrap().trim().replace(")", ""),
        );

        match graph.get_adjacent(current_node) {
            Some(x) => {
                println!("node inserted: {:?}", x)
            }
            None => {}
        }
    }

    let mut is_dest = false;
    let mut current = "AAA";

    let mut steps: i64 = 0;

    while !is_dest {
        for direction in directions.iter() {
            match graph.get_adjacent(current) {
                Some(x) => {
                    println!("Currently at: {}", current);
                    println!("Going: {}", direction);
                    if current.eq("ZZZ") {
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
    println!("steps: {}", steps)
}
