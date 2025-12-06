// Advent of Code 2025 - Day 06

use std::cmp::Reverse;

#[derive(Debug)]
struct Group {
    op: String,
    numbers: Vec<String>,
}

impl Group {
    fn get_numbers(&self, vertical: bool) -> Vec<usize> {
        if !vertical {
            return self
                .numbers
                .iter()
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect();
        }

        let mut numbers = self.numbers.clone();
        // Sort by descending order by length.
        numbers.sort_by_key(|a| Reverse(a.len()));
        let max_len = numbers[0].len();
        let mut new_numbers = vec!["".to_string(); max_len];
        for n in self.numbers.iter() {
            for (i, c) in n.chars().rev().enumerate() {
                new_numbers[i] = format!("{}{}", new_numbers[i], c).trim().to_string();
            }
        }
        println!("Before: {:?}, After: {:?}", self.numbers, new_numbers);
        new_numbers
            .iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect()
    }
    fn evaluate(&self, vertical: bool) -> Option<usize> {
        let numbers = self.get_numbers(vertical);
        match self.op.as_str() {
            "+" => Some(numbers.iter().sum()),
            "*" => Some(numbers.iter().product()),
            _ => None,
        }
    }
}

fn parse_input(inp: &str) -> anyhow::Result<Vec<Group>> {
    let mut lines = inp.lines().collect::<Vec<&str>>();
    let op_line = lines.pop().unwrap();
    let mut groups: Vec<Group> = Vec::new();
    // "*   +   *   +  "
    // Numbers are being aligned by operators. For every operator the digit range is from that to one whitespace before the next.
    let mut ranges = vec![];
    let op_line_chars = op_line.chars();
    for (i, c) in op_line_chars.enumerate() {
        if c != ' ' {
            ranges.push(i);
            groups.push(Group {
                op: c.to_string(),
                numbers: Vec::new(),
            });
        }
    }
    println!("Ranges: {:?}", ranges);
    for line in lines.iter() {
        for i in 0..ranges.len() {
            let start = ranges[i];
            let end = if i + 1 < ranges.len() {
                ranges[i + 1] - 1
            } else {
                line.len()
            };
            let number_str = line[start..end].to_string();
            groups[i].numbers.push(number_str);
        }
    }
    println!("Parsed groups: {:?}", groups);
    Ok(groups)
}

pub fn solution_2025_06_01(file_path: String) -> anyhow::Result<usize> {
    let groups = parse_input(&std::fs::read_to_string(file_path)?)?;
    Ok(groups.iter().filter_map(|g| g.evaluate(false)).sum())
}

pub fn solution_2025_06_02(file_path: String) -> anyhow::Result<usize> {
    let groups = parse_input(&std::fs::read_to_string(file_path)?)?;
    Ok(groups.iter().filter_map(|g| g.evaluate(true)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_06_01() {
        let file_path: String = String::from("inputs/2025/day06e.txt");
        let result = solution_2025_06_01(file_path).unwrap();
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_2025_06_02() {
        let file_path: String = String::from("inputs/2025/day06e.txt");
        let result = solution_2025_06_02(file_path).unwrap();
        assert_eq!(result, 3263827);
    }

    #[test]
    #[ignore]
    fn output_2025_06_01() {
        let file_path: String = String::from("inputs/2025/day06.txt");
        let result = solution_2025_06_01(file_path).unwrap();
        assert_eq!(result, 5877594983578);
    }

    #[test]
    #[ignore]
    fn output_2025_06_02() {
        let file_path: String = String::from("inputs/2025/day06.txt");
        let result = solution_2025_06_02(file_path).unwrap();
        assert_eq!(result, 11159825706149);
    }
}
