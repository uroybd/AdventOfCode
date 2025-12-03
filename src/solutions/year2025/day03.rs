// Advent of Code 2025 - Day 3


pub fn solution_day_3_01(file_path: String) -> anyhow::Result<usize> {
    None
}

pub fn solution_day_3_02(file_path: String) -> anyhow::Result<usize> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_01() {
        let file_path: String = String::from("inputs/2025/day3e.txt");
        let result = solution_day_3_01(file_path).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_day_3_02() {
        let file_path: String = String::from("inputs/2025/day3e.txt");
        let result = solution_day_3_02(file_path).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn output_day_3_01() {
        let file_path: String = String::from("inputs/2025/day3.txt");
        let result = solution_day_3_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_3_02() {
        let file_path: String = String::from("inputs/2025/day3.txt");
        let result = solution_day_3_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
