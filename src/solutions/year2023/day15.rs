// Advent of Code 2023 - Day 15

use std::{collections::HashMap, fs};

use derive_deref::{Deref, DerefMut};
use indexmap::IndexMap;

fn get_hash(seq: &str) -> usize {
    let res = seq
        .chars()
        .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256);
    res as usize
}
enum Operation {
    Set((String, usize)),
    Remove(String),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOperationError;

impl std::str::FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            Ok(Operation::Remove(s[0..s.len() - 1].to_string()))
        } else {
            let (name, val) = s.split_once('=').unwrap();
            Ok(Operation::Set((
                name.to_string(),
                val.parse::<usize>().unwrap(),
            )))
        }
    }
}

#[derive(Deref, DerefMut)]
struct BoxArray(HashMap<usize, IndexMap<String, usize>>);

impl BoxArray {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn operate(&mut self, op: Operation) {
        match op {
            Operation::Set((name, val)) => {
                let hash = get_hash(&name);
                let entry = self.entry(hash).or_default();
                entry.insert(name, val);
            }
            Operation::Remove(name) => {
                let hash = get_hash(&name);
                if let Some(entry) = self.get_mut(&hash) {
                    entry.shift_remove(&name);
                }
            }
        }
    }

    fn calculate_focusing_power(&self) -> usize {
        self.iter()
            .flat_map(|(n, v)| {
                v.iter()
                    .enumerate()
                    .map(move |(i, (_, f))| (n + 1) * f * (i + 1))
            })
            .sum::<usize>()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBoxArrayError;

impl std::str::FromStr for BoxArray {
    type Err = ParseBoxArrayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self::new();
        for line in s.lines() {
            for ins in line.split(',') {
                res.operate(ins.parse::<Operation>().unwrap());
            }
        }
        Ok(res)
    }
}

pub fn solution_2023_15_01(file_path: String) -> Option<usize> {
    let res = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .lines()
        .flat_map(|l| l.split(',').map(get_hash))
        .sum::<usize>();
    Some(res)
}

pub fn solution_2023_15_02(file_path: String) -> Option<usize> {
    let boxes: BoxArray = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .parse()
        .unwrap();

    Some(boxes.calculate_focusing_power())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_15_01() {
        let file_path: String = String::from("inputs/2023/day15e.txt");
        let result = solution_2023_15_01(file_path).unwrap();
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_2023_15_02() {
        let file_path: String = String::from("inputs/2023/day15e.txt");
        let result = solution_2023_15_02(file_path).unwrap();
        assert_eq!(result, 145);
    }

    #[test]
    #[ignore]
    fn output_day_15_01() {
        let file_path: String = String::from("inputs/2023/day15.txt");
        let result = solution_2023_15_01(file_path).unwrap();
        assert_eq!(result, 510388);
    }

    #[test]
    #[ignore]
    fn output_day_15_02() {
        let file_path: String = String::from("inputs/2023/day15.txt");
        let result = solution_2023_15_02(file_path).unwrap();
        assert_eq!(result, 291774);
    }
}
