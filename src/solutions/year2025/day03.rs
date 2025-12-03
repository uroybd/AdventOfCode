use std::fs;

// Advent of Code 2025 - Day 3
fn get_jolts(val: &str, digits: usize) -> usize {
    let length = val.len();
    let bytes = val.as_bytes();
    let mut start = 0;
    (0..digits).rev().fold(0, |acc, i| {
        let scope = &bytes[start..(length - i)];
        let mut highest = 0;
        let mut highest_pos = 0;
        for (pos, &b) in scope.iter().enumerate() {
            if b > highest {
                highest = b;
                highest_pos = pos;
            }
            // Can't be larger than '9'
            if b == b'9' {
                break;
            }
        }
        start += highest_pos + 1;
        acc * 10 + (highest - b'0') as usize
    })
}

pub fn solution_2025_03_01(file_path: String) -> anyhow::Result<usize> {
    let res = fs::read_to_string(file_path)
        .expect("Invalid file")
        .lines()
        .map(|line| get_jolts(line, 2))
        .sum();
    Ok(res)
}

pub fn solution_2025_03_02(file_path: String) -> anyhow::Result<usize> {
    let res = fs::read_to_string(file_path)
        .expect("Invalid file")
        .lines()
        .map(|line| get_jolts(line, 12))
        .sum();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_03_01() {
        let file_path: String = String::from("inputs/2025/day03e.txt");
        let result = solution_2025_03_01(file_path).unwrap();
        assert_eq!(result, 357);
    }

    #[test]
    fn test_2025_03_02() {
        let file_path: String = String::from("inputs/2025/day03e.txt");
        let result = solution_2025_03_02(file_path).unwrap();
        assert_eq!(result, 3121910778619);
    }

    #[test]
    #[ignore]
    fn output_2025_03_01() {
        let file_path: String = String::from("inputs/2025/day03.txt");
        let result = solution_2025_03_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_2025_03_02() {
        let file_path: String = String::from("inputs/2025/day03.txt");
        let result = solution_2025_03_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
