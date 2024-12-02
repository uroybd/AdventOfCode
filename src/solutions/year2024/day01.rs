use std::collections::HashMap;

fn parse_line(line: &str) -> Vec<i32> {
    return line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();
}

pub fn parse_input(filepath: String) -> Vec<Vec<i32>> {
    let mut left = vec![];
    let mut right = vec![];
    std::fs::read_to_string(filepath)
        .expect("Invalid file")
        .trim_end()
        .lines()
        .for_each(|line| {
            let vals = parse_line(line);
            left.push(vals[0]);
            right.push(vals[1]);
        });
    left.sort();
    right.sort();
    return vec![left, right];
}

pub fn solution_2024_01_01(filepath: String) -> Option<i32> {
    let input = parse_input(filepath);
    let result = input[0].iter().zip(input[1].iter()).fold(0, |acc, (l, r)| {
        return acc + (l - r).abs();
    });
    Some(result)
}

pub fn solution_2024_01_02(filepath: String) -> Option<i32> {
    let input = parse_input(filepath);
    let result = input[0].iter().fold(0, |acc, key| {
        let count = input[1].iter().filter(|&v| v == key).count();
        return acc + (count as i32 * key);
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2024_01_01() {
        let file_path = String::from("inputs/2024/day01.txt");
        let result = solution_2024_01_01(file_path).unwrap();
        assert_eq!(result, 2066446);
    }

    #[test]
    fn test_2024_01_02() {
        let file_path = String::from("inputs/2024/day01.txt");
        let result = solution_2024_01_02(file_path).unwrap();
        assert_eq!(result, 24931009);
    }
}
