// Advent of Code 2021 - Day 02

pub fn solution_2021_02_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = std::fs::read_to_string(filepath)?
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    
    let mut distance: i64 = 0;
    let mut depth: i64 = 0;
    
    for val in instructions {
        let mut splitted = val.split(' ');
        let instruction = splitted.next().unwrap();
        let value = splitted.next().unwrap().parse::<i64>()?;
        match instruction {
            "forward" => distance += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => (),
        }
    }
    
    Ok(distance * depth)
}

pub fn solution_2021_02_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = std::fs::read_to_string(filepath)?
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    
    let mut distance: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
    
    for val in instructions {
        let mut splitted = val.split(' ');
        let instruction = splitted.next().unwrap();
        let value = splitted.next().unwrap().parse::<i64>()?;
        match instruction {
            "forward" => {
                distance += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => (),
        }
    }
    
    Ok(distance * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_02_01() {
        let result = solution_2021_02_01("inputs/2021/day02e.txt".to_string()).unwrap();
        assert_eq!(result, 150);
    }

    #[test]
    #[ignore]
    fn output_2021_02_01() {
        let result = solution_2021_02_01("inputs/2021/day02.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_02_02() {
        let result = solution_2021_02_02("inputs/2021/day02e.txt".to_string()).unwrap();
        assert_eq!(result, 900);
    }

    #[test]
    #[ignore]
    fn output_2021_02_02() {
        let result = solution_2021_02_02("inputs/2021/day02.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
