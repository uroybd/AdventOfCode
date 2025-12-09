// Advent of Code 2021 - Day 06

use std::fs;

pub fn solution_2021_06_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<usize> = fs::read_to_string(filepath)?
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut fishes: [usize; 9] = [0; 9];
    for f in data {
        fishes[f] += 1;
    }
    for i in 0..80 {
        fishes[(i + 7) % 9] += fishes[i % 9];
    }
    let result: usize = fishes.iter().sum();
    Ok(result as i64)
}

pub fn solution_2021_06_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<usize> = fs::read_to_string(filepath)?
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut fishes: [usize; 9] = [0; 9];
    for f in data {
        fishes[f] += 1;
    }
    for day in 0..256 {
        fishes[(day + 7) % 9] += fishes[day % 9];
    }
    let result: usize = fishes.iter().sum();
    Ok(result as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_06_01() {
        let result = solution_2021_06_01("inputs/2021/day06e.txt".to_string()).unwrap();
        assert_eq!(result, 5934);
    }

    #[test]
    #[ignore]
    fn output_2021_06_01() {
        let result = solution_2021_06_01("inputs/2021/day06.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_06_02() {
        let result = solution_2021_06_02("inputs/2021/day06e.txt".to_string()).unwrap();
        assert_eq!(result, 26984457539);
    }

    #[test]
    #[ignore]
    fn output_2021_06_02() {
        let result = solution_2021_06_02("inputs/2021/day06.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
