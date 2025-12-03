fn parse_input(filepath: &str) -> Vec<Vec<i32>> {
    std::fs::read_to_string(filepath)
        .expect("Invalid file")
        .trim_end()
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut direction = 0;
    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];
        if diff.abs() > 3 || diff == 0 {
            return false;
        }
        match direction {
            0 => {
                if diff.is_negative() {
                    direction = -1;
                } else {
                    direction = 1
                }
            }
            1 => {
                if diff.is_negative() {
                    return false;
                }
            }
            -1 => {
                if diff.is_positive() {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    true
}

fn is_safe_with_dampner(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut skipped_report = report.clone();
        skipped_report.remove(i);
        if is_safe(&skipped_report) {
            return true;
        }
    }
    false
}

pub fn solution_2024_02_01(filepath: String) -> Option<i32> {
    let input = parse_input(&filepath);
    Some(input.into_iter().filter(is_safe).count() as i32)
}

pub fn solution_2024_02_02(filepath: String) -> Option<i32> {
    let input = parse_input(&filepath);
    Some(input.into_iter().filter(is_safe_with_dampner).count() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2024_02_01() {
        let file_path = String::from("inputs/2024/day02.txt");
        let result = solution_2024_02_01(file_path).unwrap();
        dbg!(result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2024_02_02() {
        let file_path = String::from("inputs/2024/day02.txt");
        let result = solution_2024_02_02(file_path).unwrap();
        assert_eq!(result, 296);
    }
}
