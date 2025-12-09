// Advent of Code 2021 - Day 10

use std::collections::HashMap;
use std::fs;

fn is_ending(c: char) -> bool {
    c == ']' || c == ')' || c == '>' || c == '}'
}

fn auto_complete(val: &Vec<char>) -> usize {
    val.iter().rev().fold(0, |acc, c| match c {
        '(' => (acc * 5) + 1,
        '[' => (acc * 5) + 2,
        '{' => (acc * 5) + 3,
        '<' => (acc * 5) + 4,
        _ => acc,
    })
}

fn find_corruptions(instruction: &str) -> (usize, usize) {
    let pair_map: HashMap<char, char> =
        HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);
    let mut stack: Vec<char> = vec![];
    for c in instruction.chars() {
        if is_ending(c) {
            if c == *pair_map.get(stack.last().unwrap()).unwrap() {
                stack.remove(stack.len() - 1);
            } else {
                let res = match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
                return (res, 0);
            }
        } else {
            stack.push(c);
        }
    }
    (0, auto_complete(&stack))
}

pub fn solution_2021_10_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let res = instructions.iter().fold(0, |acc, val| {
        let (corrupt, _) = find_corruptions(val);
        acc + corrupt
    });
    Ok(res as i64)
}

pub fn solution_2021_10_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let mut auto_completes: Vec<usize> = vec![];
    for val in instructions.iter() {
        let (corrupt, auto_complete) = find_corruptions(val);
        if corrupt == 0 && auto_complete > 0 {
            auto_completes.push(auto_complete);
        }
    }
    auto_completes.sort_unstable();
    Ok(auto_completes[auto_completes.len() / 2] as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_10_01() {
        let result = solution_2021_10_01("inputs/2021/day10e.txt".to_string()).unwrap();
        assert_eq!(result, 26397);
    }

    #[test]
    #[ignore]
    fn output_2021_10_01() {
        let result = solution_2021_10_01("inputs/2021/day10.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_10_02() {
        let result = solution_2021_10_02("inputs/2021/day10e.txt".to_string()).unwrap();
        assert_eq!(result, 288957);
    }

    #[test]
    #[ignore]
    fn output_2021_10_02() {
        let result = solution_2021_10_02("inputs/2021/day10.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
