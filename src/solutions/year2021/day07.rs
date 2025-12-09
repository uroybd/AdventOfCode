// Advent of Code 2021 - Day 07

use std::fs;

pub fn solution_2021_07_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut input: Vec<i64> = fs::read_to_string(filepath)?
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    input.sort_unstable();
    let length = input.len();
    let median = input[(length / 2) + (length % 2)];
    Ok(input.iter().fold(0, |acc, x| {
        let diff = x - median;
        acc + diff.abs()
    }))
}

pub fn solution_2021_07_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let input: Vec<f64> = fs::read_to_string(filepath)?
        .split(',')
        .map(|v| v.parse::<f64>().unwrap())
        .collect();
    let length = input.len();
    let mean: f64 = input.iter().sum::<f64>() / length as f64;
    let mean_round = mean.round();
    let mean_floor = mean.floor();
    let floor_output = input.iter().fold(0.0, |acc, x| {
        let diff = x - mean_floor;
        let diff_abs = diff.abs();
        acc + (diff_abs * (diff_abs + 1.0) / 2.0)
    });
    if mean_floor == mean_round {
        Ok(floor_output as i64)
    } else {
        let round_output = input.iter().fold(0.0, |acc, x| {
            let diff = x - mean_round;
            let diff_abs = diff.abs();
            acc + (diff_abs * (diff_abs + 1.0) / 2.0)
        });
        if round_output > floor_output {
            Ok(floor_output as i64)
        } else {
            Ok(round_output as i64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_07_01() {
        let result = solution_2021_07_01("inputs/2021/day07e.txt".to_string()).unwrap();
        assert_eq!(result, 37);
    }

    #[test]
    #[ignore]
    fn output_2021_07_01() {
        let result = solution_2021_07_01("inputs/2021/day07.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_07_02() {
        let result = solution_2021_07_02("inputs/2021/day07e.txt".to_string()).unwrap();
        assert_eq!(result, 168);
    }

    #[test]
    #[ignore]
    fn output_2021_07_02() {
        let result = solution_2021_07_02("inputs/2021/day07.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
