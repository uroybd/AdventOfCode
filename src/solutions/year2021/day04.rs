// Advent of Code 2021 - Day 04
// TODO: Manual conversion needed - this is a stub

pub fn solution_2021_04_01(_filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    // Original source: day4_1.rs
    todo!("Convert day 4 part 1")
}

pub fn solution_2021_04_02(_filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    // Original source: day4_2.rs
    todo!("Convert day 4 part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2021_04_01() {
        let result = solution_2021_04_01("inputs/2021/day04e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_04_01() {
        let result = solution_2021_04_01("inputs/2021/day04.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_04_02() {
        let result = solution_2021_04_02("inputs/2021/day04e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_04_02() {
        let result = solution_2021_04_02("inputs/2021/day04.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
