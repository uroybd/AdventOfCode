fn is_nice_naive(s: &str) -> bool {
    let vowels = "aeiou";
    let forbidden = ["ab", "cd", "pq", "xy"];

    let vowel_count = s.chars().filter(|c| vowels.contains(*c)).count();
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    let has_forbidden = forbidden.iter().any(|f| s.contains(f));

    vowel_count >= 3 && has_double && !has_forbidden
}

fn is_nice_advanced(s: &str) -> bool {
    let has_repeated_pair = (0..s.len() - 1).any(|i| {
        let pair = &s[i..i + 2];
        s[i + 2..].contains(pair)
    });

    let has_sandwiched_letter = (0..s.len() - 2).any(|i| s.chars().nth(i) == s.chars().nth(i + 2));

    has_repeated_pair && has_sandwiched_letter
}

pub fn solution_2015_05_01(filepath: String) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filepath).expect("Invalid file.");
    let nice_count = content.lines().filter(|line| is_nice_naive(line)).count();
    Ok(nice_count)
}

pub fn solution_2015_05_02(filepath: String) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filepath).expect("Invalid file.");
    let nice_count = content
        .lines()
        .filter(|line| is_nice_advanced(line))
        .count();
    Ok(nice_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_2015_05_01_example() {
        let file_path = String::from("inputs/2015/day05e.txt");
        let result = solution_2015_05_01(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solution_2015_05_01() {
        let file_path = String::from("inputs/2015/day05.txt");
        let result = solution_2015_05_01(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 255);
    }

    #[test]
    fn test_solution_2015_05n02_example() {
        let file_path = String::from("inputs/2015/day05e2.txt");
        let result = solution_2015_05_02(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solution_2015_05_02() {
        let file_path = String::from("inputs/2015/day05.txt");
        let result = solution_2015_05_02(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 55);
    }
}
