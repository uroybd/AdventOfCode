use regex::Regex;
use std::fs;

// Advent of Code 2023 - Day 01

// A regex-based solution is also possible, but too slow.
fn get_calibration_value(val: &str) -> u32 {
    let first = val
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last = val
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();
    (first * 10) + last
}

fn get_converted_value(s: &str) -> u32 {
    match s {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => todo!(),
    }
}

fn get_calibration_value_extended(val: &str, p: &Regex, rev_p: &Regex) -> u32 {
    let first = p.find(val).unwrap().as_str();
    let last: String = rev_p
        .find(&val.chars().rev().collect::<String>())
        .unwrap()
        .as_str()
        .chars()
        .rev()
        .collect();
    (get_converted_value(first) * 10) + get_converted_value(&last)
}

pub fn solution_day_01_01(file_path: String) -> Option<u32> {
    Some(
        fs::read_to_string(file_path)
            .expect("Invalid File")
            .lines()
            .map(get_calibration_value)
            .sum(),
    )
}

pub fn solution_day_01_02(file_path: String) -> Option<u32> {
    let pattern = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d")
        .expect("Unable to compile regex");
    let reverse_pattern = Regex::new(r"enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\d")
        .expect("Unable to compile regex");

    Some(
        fs::read_to_string(file_path)
            .expect("Invalid File")
            .lines()
            .map(|l| get_calibration_value_extended(l, &pattern, &reverse_pattern))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_01() {
        let file_path: String = String::from("inputs/2023/day01e.txt");
        let result = solution_day_01_01(file_path).unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_day_01_02() {
        let file_path: String = String::from("inputs/2023/day01e2.txt");
        let result = solution_day_01_02(file_path).unwrap();
        assert_eq!(result, 281);
    }

    #[test]
    #[ignore]
    fn output_day_01_01() {
        let file_path: String = String::from("inputs/2023/day01.txt");
        let result = solution_day_01_01(file_path).unwrap();
        assert_eq!(result, 53974);
    }

    #[test]
    #[ignore]
    fn output_day_01_02() {
        let file_path: String = String::from("inputs/2023/day01.txt");
        let result = solution_day_01_02(file_path).unwrap();
        assert_eq!(result, 52840);
    }
}
