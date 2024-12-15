use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

fn parse_input(file_path: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let rules = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line
                .split('|')
                .map(|x| {
                    x.parse()
                        .unwrap_or_else(|_| panic!("Failed to parse rule: {}", line))
                })
                .collect();
            if parts.len() != 2 {
                panic!("Rule format is incorrect: {}", line);
            }
            (parts[0], parts[1])
        })
        .collect();

    let updates = sections[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| {
                    x.parse()
                        .unwrap_or_else(|_| panic!("Failed to parse update: {}", line))
                })
                .collect()
        })
        .collect();

    (rules, updates)
}

fn build_dependency_graph(rules: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut graph = HashMap::new();

    for &(a, b) in rules {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new); // Ensure all nodes exist in the graph
    }

    graph
}

fn is_valid_order(graph: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    let mut position = HashMap::new();

    for (index, &page) in update.iter().enumerate() {
        position.insert(page, index);
    }

    for (&page, dependencies) in graph {
        if let Some(&page_pos) = position.get(&page) {
            for &dependent in dependencies {
                if let Some(&dependent_pos) = position.get(&dependent) {
                    if page_pos >= dependent_pos {
                        return false; // A dependency rule is violated
                    }
                }
            }
        }
    }

    true
}

fn sort_update(graph: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> Vec<u32> {
    let mut position = HashMap::new();
    for &page in update {
        position.insert(page, 0);
    }

    let mut sorted = Vec::new();
    let mut visited = HashSet::new();

    fn visit(
        node: u32,
        graph: &HashMap<u32, HashSet<u32>>,
        visited: &mut HashSet<u32>,
        sorted: &mut Vec<u32>,
        position: &HashMap<u32, u32>,
    ) {
        if !visited.contains(&node) {
            visited.insert(node);
            if let Some(dependencies) = graph.get(&node) {
                for &dependent in dependencies {
                    if position.contains_key(&dependent) {
                        visit(dependent, graph, visited, sorted, position);
                    }
                }
            }
            sorted.push(node);
        }
    }

    for &page in update {
        visit(page, graph, &mut visited, &mut sorted, &position);
    }

    sorted.reverse();
    sorted
}

pub fn solve(input: &str) -> io::Result<(u32, u32)> {
    let (rules, updates) = parse_input(input);
    let graph = build_dependency_graph(&rules);

    let mut valid = 0;
    let mut corrected = 0;
    for update in updates.iter() {
        if is_valid_order(&graph, update) {
            valid += update[update.len() / 2];
        } else {
            let fixed = sort_update(&graph, update);
            corrected += fixed[fixed.len() / 2];
        }
    }

    Ok((valid, corrected))
}
