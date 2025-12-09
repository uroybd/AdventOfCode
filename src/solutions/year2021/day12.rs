// Advent of Code 2021 - Day 12

use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Node {
    symbol: String,
    max_visit: usize,
    connections: HashSet<String>,
}

impl Node {
    fn can_visit(&self, visit_map: &mut HashMap<String, usize>) -> bool {
        let visited = visit_map.entry(self.symbol.to_string()).or_insert(0);
        *visited < self.max_visit
    }

    fn can_visit_extended(&self, visit_map: &mut HashMap<String, usize>, extend: &String) -> bool {
        let visited = visit_map.entry(self.symbol.to_string()).or_insert(0);
        if extend == &self.symbol {
            *visited < 2
        } else {
            *visited < self.max_visit
        }
    }

    fn new(sym: String) -> (Node, bool) {
        let (node, cave) = match sym.as_str() {
            "start" | "end" => (
                Node {
                    symbol: sym,
                    max_visit: 1,
                    connections: HashSet::new(),
                },
                false,
            ),
            x => {
                let x_string = x.to_string();
                let is_uppercase = x_string.chars().next().unwrap().is_uppercase();
                if is_uppercase {
                    (
                        Node {
                            symbol: x_string,
                            max_visit: usize::MAX,
                            connections: HashSet::new(),
                        },
                        false,
                    )
                } else {
                    (
                        Node {
                            symbol: x_string,
                            max_visit: 1,
                            connections: HashSet::new(),
                        },
                        true,
                    )
                }
            }
        };
        (node, cave)
    }
}

fn create_nodes_part1(data: &Vec<String>) -> HashMap<String, Node> {
    let mut node_map: HashMap<String, Node> = HashMap::new();
    for path in data {
        let symbols: Vec<String> = path.split('-').map(|x| x.to_string()).collect();
        for sym in symbols.into_iter() {
            let node = node_map
                .entry(sym.clone())
                .or_insert_with(|| Node::new(sym.clone()).0);
            node.connections.extend(
                path.split('-')
                    .map(|x| x.to_string())
                    .filter(|x| x != &sym.clone()),
            )
        }
    }
    node_map
}

fn create_nodes_part2(data: &Vec<String>) -> (HashMap<String, Node>, HashSet<String>) {
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut small_caves: HashSet<String> = HashSet::new();
    for path in data {
        let symbols: Vec<String> = path.split('-').map(|x| x.to_string()).collect();
        for sym in symbols.into_iter() {
            if node_map.contains_key(&sym) {
                let node = node_map.get_mut(&sym.clone()).unwrap();
                node.connections.extend(
                    path.split('-')
                        .map(|x| x.to_string())
                        .filter(|x| x != &sym.clone()),
                )
            } else {
                let (mut node, smaller) = Node::new(sym.clone());
                node.connections.extend(
                    path.split('-')
                        .map(|x| x.to_string())
                        .filter(|x| x != &sym.clone()),
                );
                if smaller {
                    small_caves.insert(sym.clone());
                }
                node_map.insert(sym.clone(), node);
            }
        }
    }
    (node_map, small_caves)
}

fn traverse_part1(
    nodes: &HashMap<String, Node>,
    start: &String,
    visit_map: &HashMap<String, usize>,
) -> Vec<Vec<String>> {
    let mut visit_map: HashMap<String, usize> = visit_map.clone();
    if start == &String::from("end") {
        return vec![vec![String::from("end")]];
    }
    let current_node = visit_map.entry(start.to_string()).or_insert(0);
    *current_node += 1;
    let start_node = &nodes[&start.clone()];
    let connections: Vec<&String> = start_node
        .connections
        .iter()
        .filter(|x| nodes[&x.to_string()].can_visit(&mut visit_map))
        .collect();
    if !connections.is_empty() {
        let mut results: Vec<Vec<String>> = vec![];
        for conn in connections {
            let val = traverse_part1(nodes, conn, &visit_map);
            for v in val {
                let mut cur_vec = vec![start.to_string()];
                cur_vec.extend(v);
                results.push(cur_vec);
            }
        }
        results
    } else {
        vec![vec![start.to_string()]]
    }
}

fn traverse_part2(
    nodes: &HashMap<String, Node>,
    start: &String,
    extend_visit: &String,
    visit_map: &HashMap<String, usize>,
) -> Vec<Vec<String>> {
    let mut visit_map: HashMap<String, usize> = visit_map.clone();
    if start == &String::from("end") {
        return vec![vec![String::from("end")]];
    }
    let current_node = visit_map.entry(start.to_string()).or_insert(0);
    *current_node += 1;
    let start_node = &nodes[&start.clone()];
    let connections: Vec<&String> = start_node
        .connections
        .iter()
        .filter(|x| nodes[&x.to_string()].can_visit_extended(&mut visit_map, extend_visit))
        .collect();
    if !connections.is_empty() {
        let mut results: Vec<Vec<String>> = vec![];
        for conn in connections {
            let val = traverse_part2(nodes, conn, extend_visit, &visit_map);
            for v in val {
                let mut cur_vec = vec![start.to_string()];
                cur_vec.extend(v);
                results.push(cur_vec);
            }
        }
        results
    } else {
        vec![vec![start.to_string()]]
    }
}

pub fn solution_2021_12_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let paths: Vec<String> = fs::read_to_string(filepath)?
        .lines()
        .map(|s| s.to_string())
        .collect();
    let nodes = create_nodes_part1(&paths);
    let visit_map: HashMap<String, usize> = HashMap::new();
    let values = traverse_part1(&nodes, &"start".to_string(), &visit_map);
    let count = values
        .iter()
        .filter(|x| x.last().unwrap() == &String::from("end"))
        .count();
    Ok(count as i64)
}

pub fn solution_2021_12_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let paths: Vec<String> = fs::read_to_string(filepath)?
        .lines()
        .map(|s| s.to_string())
        .collect();
    let (nodes, small_caves) = create_nodes_part2(&paths);
    let mut results: Vec<String> = vec![];
    for cave in small_caves {
        let visit_map: HashMap<String, usize> = HashMap::new();
        let values = traverse_part2(&nodes, &"start".to_string(), &cave, &visit_map);
        let set_val: Vec<String> = values
            .iter()
            .filter(|x| x.last().unwrap() == &String::from("end"))
            .map(|x| x.join(","))
            .collect();
        results.extend(set_val);
    }
    results.sort();
    results.dedup();
    Ok(results.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_12_01() {
        let result = solution_2021_12_01("inputs/2021/day12e.txt".to_string()).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    #[ignore]
    fn output_2021_12_01() {
        let result = solution_2021_12_01("inputs/2021/day12.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_12_02() {
        let result = solution_2021_12_02("inputs/2021/day12e.txt".to_string()).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    #[ignore]
    fn output_2021_12_02() {
        let result = solution_2021_12_02("inputs/2021/day12.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
