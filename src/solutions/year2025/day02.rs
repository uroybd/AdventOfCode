fn is_invalid(id: &str) -> bool {
    let length = id.len();
    id[..length / 2] == id[length / 2..]
}

fn is_invalid_complex(id: &str) -> bool {
    let length = id.len();
    if length < 2 {
        return false;
    }
    let chars = id.chars().collect::<Vec<char>>();
    for i in 1..=length / 2 {
        let chunks = chars
            .chunks(i)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        let mut all_equal = true;
        for j in 1..chunks.len() {
            if chunks[j] != chunks[0] {
                all_equal = false;
                break;
            }
        }
        if all_equal {
            return true;
        }
    }
    false
}

fn get_range(range_string: &str) -> impl Iterator<Item = usize> {
    let mut parts = range_string.split('-');
    let start: usize = parts.next().unwrap().parse().unwrap();
    let end: usize = parts.next().unwrap().parse().unwrap();
    start..=end
}

pub fn solution_2025_02_01(filepath: String) -> Result<usize, Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string(filepath)?
        .trim_end()
        .split(',')
        .map(|range_str| {
            get_range(range_str)
                .filter(|id| is_invalid(&id.to_string()))
                .sum::<usize>()
        })
        .sum();
    Ok(result)
}

pub fn solution_2025_02_02(filepath: String) -> Result<usize, Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string(filepath)?
        .trim_end()
        .split(',')
        .map(|range_str| {
            get_range(range_str)
                .filter(|id| is_invalid_complex(&id.to_string()))
                .sum::<usize>()
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_02_01_example() {
        let file_path = String::from("inputs/2025/day02e.txt");
        let result = solution_2025_02_01(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_2025_02_01() {
        let file_path = String::from("inputs/2025/day02.txt");
        let result = solution_2025_02_01(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        println!("Result: {}", result);
        assert_eq!(result, 35367539282);
    }

    #[test]
    fn test_2025_02_02_example() {
        let file_path = String::from("inputs/2025/day02e.txt");
        let result = solution_2025_02_02(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_2025_02_02() {
        let file_path = String::from("inputs/2025/day02.txt");
        let result = solution_2025_02_02(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 45814076230);
    }
}
