use md5;

fn calculate(key: &str, offset: usize) -> usize {
    let offset_string = "0".repeat(offset);
    let mut n = 1;
    loop {
        let digest = md5::compute(format!("{}{}", key, n));
        let dstring = format!("{:x}", digest);
        if dstring.starts_with(offset_string.as_str()) {
            return n;
        }
        n += 1;
    }
}

pub fn solution_2015_04_01(filepath: String) -> Option<usize> {
    let key = std::fs::read_to_string(filepath)
        .expect("Invalid file.")
        .trim()
        .to_string();
    Some(calculate(&key, 5))
}

pub fn solution_2015_04_02(filepath: String) -> Option<usize> {
    let key = std::fs::read_to_string(filepath)
        .expect("Invalid file.")
        .trim()
        .to_string();
    Some(calculate(&key, 6))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution_2015_04_01() {
        let file_path = String::from("inputs/2015/day04.txt");
        assert_eq!(solution_2015_04_01(file_path), Some(282749));
    }

    #[test]
    fn test_solution_2015_04_02() {
        let file_path = String::from("inputs/2015/day04.txt");
        assert_eq!(solution_2015_04_02(file_path), Some(9962624));
    }
}
