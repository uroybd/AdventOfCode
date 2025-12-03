use num::integer::lcm;
use rayon::prelude::*;
use std::{collections::HashMap, fs};
// Advent of Code 2023 - Day 08

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl std::convert::From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(PartialEq, Eq, Debug)]
struct ParseNodeError;

impl std::str::FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, pair) = s.split_once(" = ").unwrap();
        let mut pair = pair.to_string();
        pair.pop();
        pair.remove(0);
        let (left, right) = pair.split_once(", ").unwrap();
        Ok(Self {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[derive(PartialEq, Eq, Debug)]
struct ParseMapError;

impl std::str::FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, node_strings) = s.split_once("\n\n").unwrap();
        let instructions: Vec<Direction> = ins.chars().map(|c| c.into()).collect();
        let mut nodes = HashMap::new();
        for l in node_strings.lines() {
            let node: Node = l.parse().unwrap();
            nodes.insert(node.name.clone(), node);
        }
        Ok(Self {
            instructions,
            nodes,
        })
    }
}

impl Map {
    fn walk(&self, n: &str, tester: impl Fn(&String) -> bool) -> usize {
        let mut steps = 0;
        let mut current_node = n.to_string();
        let mut instructions = self.instructions.iter().cycle();

        while !tester(&current_node) {
            let val = self.nodes.get(&current_node).unwrap();
            current_node = match instructions.next().unwrap() {
                Direction::Left => val.left.clone(),
                Direction::Right => val.right.clone(),
            };
            steps += 1;
        }
        steps
    }

    fn traverse(&self) -> usize {
        self.walk("AAA", |x| x == "ZZZ")
    }

    fn traverse_like_a_ghost(&self) -> usize {
        self.nodes
            .keys()
            .clone()
            .filter(|k| k.ends_with('A'))
            .par_bridge()
            .into_par_iter()
            .map(|v| self.walk(v, |x| x.ends_with('Z')))
            .reduce_with(lcm)
            .unwrap()
    }
}

pub fn solution_2023_08_01(file_path: String) -> Option<usize> {
    let map: Map = fs::read_to_string(file_path)
        .expect("Invalid input.")
        .parse()
        .unwrap();
    Some(map.traverse())
}

pub fn solution_2023_08_02(file_path: String) -> Option<usize> {
    let map: Map = fs::read_to_string(file_path)
        .expect("Invalid input.")
        .parse()
        .unwrap();
    Some(map.traverse_like_a_ghost())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_08_01() {
        let file_path: String = String::from("inputs/2023/day08e.txt");
        let result = solution_2023_08_01(file_path).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2023_08_02() {
        let file_path: String = String::from("inputs/2023/day08e2.txt");
        let result = solution_2023_08_02(file_path).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    #[ignore]
    fn output_day_08_01() {
        let file_path: String = String::from("inputs/2023/day08.txt");
        let result = solution_2023_08_01(file_path).unwrap();
        assert_eq!(result, 14681);
    }

    #[test]
    #[ignore]
    fn output_day_08_02() {
        let file_path: String = String::from("inputs/2023/day08.txt");
        let result = solution_2023_08_02(file_path).unwrap();
        assert_eq!(result, 14321394058031);
    }
}
