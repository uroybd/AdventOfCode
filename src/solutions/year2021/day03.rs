// Advent of Code 2021 - Day 03

fn reducer(bins: &[String], idx: usize, least: bool) -> String {
    let length = bins.len();
    if length == 1 {
        return bins[0].to_string();
    }
    
    let mut zeros: Vec<String> = vec![];
    let mut ones: Vec<String> = vec![];
    for binary in bins {
        if binary.chars().nth(idx) == Some('1') {
            ones.push(binary.to_string());
        } else {
            zeros.push(binary.to_string())
        }
    }
    if least {
        if zeros.len() <= ones.len() {
            reducer(&zeros, idx + 1, least)
        } else {
            reducer(&ones, idx + 1, least)
        }
    } else if ones.len() >= zeros.len() {
        reducer(&ones, idx + 1, least)
    } else {
        reducer(&zeros, idx + 1, least)
    }
}

pub fn solution_2021_03_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let bins: Vec<String> = std::fs::read_to_string(filepath)?
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let half = bins.len() / 2;
    let mut ones = vec![0; bins[0].len()];

    for binary in &bins {
        for (idx, char) in binary.chars().enumerate() {
            if char == '1' {
                ones[idx] += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for val in ones {
        if val > half {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    let gamma_int = i64::from_str_radix(&gamma, 2)?;
    let epsilon_int = i64::from_str_radix(&epsilon, 2)?;

    Ok(gamma_int * epsilon_int)
}

pub fn solution_2021_03_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let bins: Vec<String> = std::fs::read_to_string(filepath)?
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let o2_val = reducer(&bins, 0, false);
    let co2_val = reducer(&bins, 0, true);

    let o2_int = i64::from_str_radix(&o2_val, 2)?;
    let co2_int = i64::from_str_radix(&co2_val, 2)?;
    Ok(o2_int * co2_int)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_03_01() {
        let result = solution_2021_03_01("inputs/2021/day03e.txt".to_string()).unwrap();
        assert_eq!(result, 198);
    }

    #[test]
    #[ignore]
    fn output_2021_03_01() {
        let result = solution_2021_03_01("inputs/2021/day03.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_03_02() {
        let result = solution_2021_03_02("inputs/2021/day03e.txt".to_string()).unwrap();
        assert_eq!(result, 230);
    }

    #[test]
    #[ignore]
    fn output_2021_03_02() {
        let result = solution_2021_03_02("inputs/2021/day03.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
