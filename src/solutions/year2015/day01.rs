pub fn solution_2015_01_01(filepath: String) -> Option<i32> {
    Some(
        std::fs::read_to_string(filepath)
            .expect("Invalid file.")
            .trim_end()
            .chars()
            .fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => unreachable!(),
            }),
    )
}

pub fn solution_2015_01_02(filepath: String) -> Option<usize> {
    let mut floor = 0;
    for (i, v) in std::fs::read_to_string(filepath)
        .expect("Invalid file.")
        .trim_end()
        .chars()
        .enumerate()
    {
        match v {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        };
        if floor < 0 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2015_01_01() {
        let file_path = String::from("inputs/2015/day01.txt");
        let result = solution_2015_01_01(file_path).unwrap();
        assert_eq!(result, 232);
    }

    #[test]
    fn test_2015_01_02() {
        let file_path = String::from("inputs/2015/day01.txt");
        let result = solution_2015_01_02(file_path).unwrap();
        assert_eq!(result, 1783);
    }
}
