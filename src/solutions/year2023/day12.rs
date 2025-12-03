// Advent of Code 2023 - Day 12
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct DamageReport(String, Vec<usize>);

#[derive(Debug, PartialEq, Eq)]
struct ParseDamageReportError;

impl std::str::FromStr for DamageReport {
    type Err = ParseDamageReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, report) = s.split_once(' ').unwrap();
        Ok(DamageReport(
            map.to_string(),
            report.split(',').map(|x| x.parse().unwrap()).collect(),
        ))
    }
}

fn create_key(inp: &str, report: &[usize]) -> String {
    format!(
        "{}-{}",
        inp,
        report
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("/")
    )
}

fn find_combinations(inp: &str, report: &[usize], cache: &mut HashMap<String, usize>) -> usize {
    let key = create_key(inp, report);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let result = match inp.chars().next() {
        None if report.is_empty() => 1,
        None => 0,
        Some('.') => find_combinations(inp.strip_prefix('.').unwrap(), report, cache),
        Some('?') => {
            find_combinations(&inp.replacen('?', ".", 1), report, cache)
                + find_combinations(&inp.replacen('?', "#", 1), report, cache)
        }
        Some('#') => {
            // First checking if the report is empty
            // Or if the current input is smaller than first candidate
            // Or if the input's substring from 0 to first candidate contains a '.'
            // And returning 0
            if report.is_empty() || inp.len() < report[0] || inp[0..report[0]].contains('.') {
                0
            } else if report.len() > 1 {
                // Check if input has padding to the right
                // Or if the input's substring from first candidate to end contains a '#'
                // And returning 0
                if (inp.len() < report[0] + 1) || (inp.chars().nth(report[0]).unwrap() == '#') {
                    0
                } else {
                    find_combinations(&inp[report[0] + 1..], &report[1..], cache)
                }
            } else {
                find_combinations(&inp[report[0]..], &report[1..], cache)
            }
        }
        _ => unreachable!(),
    };
    cache.insert(key.clone(), result);
    result
}

pub fn solution_2023_12_01(file_path: String) -> Option<usize> {
    let mut cache = HashMap::new();
    let total = fs::read_to_string(file_path)
        .expect("Invalid File")
        .lines()
        .map(|x| {
            let entry = x.parse::<DamageReport>().unwrap();
            find_combinations(&entry.0, &entry.1, &mut cache)
        })
        .sum();

    Some(total)
}

pub fn solution_2023_12_02(file_path: String) -> Option<usize> {
    let mut cache = HashMap::new();
    let total = fs::read_to_string(file_path)
        .expect("Invalid File")
        .lines()
        .map(|x| {
            let entry = x.parse::<DamageReport>().unwrap();
            let map = [entry.0.as_str(); 5].join("?");
            let report = entry.1.repeat(5);
            find_combinations(&map, &report, &mut cache)
        })
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_12_01() {
        let file_path: String = String::from("inputs/2023/day12e.txt");
        let result = solution_2023_12_01(file_path).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_2023_12_02() {
        let file_path: String = String::from("inputs/2023/day12e.txt");
        let result = solution_2023_12_02(file_path).unwrap();
        assert_eq!(result, 525152);
    }

    #[test]
    #[ignore]
    fn output_day_12_01() {
        let file_path: String = String::from("inputs/2023/day12.txt");
        let result = solution_2023_12_01(file_path).unwrap();
        assert_eq!(result, 7753);
    }

    #[test]
    #[ignore]
    fn output_day_12_02() {
        let file_path: String = String::from("inputs/2023/day12.txt");
        let result = solution_2023_12_02(file_path).unwrap();
        assert_eq!(result, 280382734828319);
    }
}
