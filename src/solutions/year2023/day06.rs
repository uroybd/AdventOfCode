use std::fs;

// Advent of Code 2023 - Day 06

fn winning_count(stat: &(f64, f64)) -> usize {
    let (b, c) = *stat;
    let delta = (b.powi(2) - (4.0 * c)).sqrt();
    let mut start = ((b - delta) / 2.0).ceil();
    let mut end = ((b + delta) / 2.0).floor();
    if start * (b - start) > c {
        start -= 1.0;
    }
    if end * (b - end) > c {
        end += 1.0
    }
    (end - start) as usize - 1
}

fn parse_line(l: &str) -> impl Iterator<Item = f64> + '_ {
    l.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
}

fn parse(data: &str) -> Vec<(f64, f64)> {
    let (times, distances) = data.split_once('\n').unwrap();
    parse_line(times).zip(parse_line(distances)).collect()
}

fn parse_combined(data: &str) -> (f64, f64) {
    let mut parts = data.split('\n').map(|v| {
        v.replace(' ', "")
            .split_once(':')
            .unwrap()
            .1
            .parse::<f64>()
            .unwrap()
    });
    (parts.next().unwrap(), parts.next().unwrap())
}

pub fn solution_2023_06_01(file_path: String) -> Option<usize> {
    Some(
        parse(&fs::read_to_string(file_path).expect("Invalid Input."))
            .iter()
            .fold(1, |acc, s| acc * winning_count(s)),
    )
}

pub fn solution_2023_06_02(file_path: String) -> Option<usize> {
    Some(winning_count(&parse_combined(
        &fs::read_to_string(file_path).expect("Invalid Input."),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_06_01() {
        let file_path: String = String::from("inputs/2023/day06e.txt");
        let result = solution_2023_06_01(file_path).unwrap();
        assert_eq!(result, 288);
    }

    #[test]
    fn test_2023_06_02() {
        let file_path: String = String::from("inputs/2023/day06e.txt");
        let result = solution_2023_06_02(file_path).unwrap();
        assert_eq!(result, 71503);
    }

    #[test]
    #[ignore]
    fn output_day_06_01() {
        let file_path: String = String::from("inputs/2023/day06.txt");
        let result = solution_2023_06_01(file_path).unwrap();
        assert_eq!(result, 1312850);
    }

    #[test]
    #[ignore]
    fn output_day_06_02() {
        let file_path: String = String::from("inputs/2023/day06.txt");
        let result = solution_2023_06_02(file_path).unwrap();
        assert_eq!(result, 36749103);
    }
}
