// Advent of Code 2023 - Day 09

use std::fs;

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

fn generate_series(data: &[isize]) -> Vec<Vec<isize>> {
    let mut series: Vec<Vec<isize>> = vec![data.to_vec()];
    while series.last().unwrap().iter().any(|x| x != &0) {
        let last = series.last().unwrap();
        let new_series = (0..(last.len() - 1))
            .map(|i| last[i + 1] - last[i])
            .collect();
        series.push(new_series)
    }
    series
}

fn report(data: &[Vec<isize>], accumulator: fn(isize, &Vec<isize>) -> isize) -> isize {
    data.iter()
        .map(|series: &Vec<isize>| {
            generate_series(series)
                .iter()
                .rev()
                .skip(1)
                .fold(0, accumulator)
        })
        .sum()
}

pub fn solution_2023_09_01(file_path: String) -> Option<isize> {
    let val = parse(&fs::read_to_string(file_path).expect("Invalid Input File."));
    Some(report(&val, |acc, v| v.last().unwrap() + acc))
}

pub fn solution_2023_09_02(file_path: String) -> Option<isize> {
    let val = parse(&fs::read_to_string(file_path).expect("Invalid Input File."));
    Some(report(&val, |acc, v| v.first().unwrap() - acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_09_01() {
        let file_path: String = String::from("inputs/2023/day09e.txt");
        let result = solution_2023_09_01(file_path).unwrap();
        assert_eq!(result, 114);
    }

    #[test]
    fn test_2023_09_02() {
        let file_path: String = String::from("inputs/2023/day09e.txt");
        let result = solution_2023_09_02(file_path).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    #[ignore]
    fn output_day_09_01() {
        let file_path: String = String::from("inputs/2023/day09.txt");
        let result = solution_2023_09_01(file_path).unwrap();
        assert_eq!(result, 1666172641);
    }

    #[test]
    #[ignore]
    fn output_day_09_02() {
        let file_path: String = String::from("inputs/2023/day09.txt");
        let result = solution_2023_09_02(file_path).unwrap();
        assert_eq!(result, 933);
    }
}
