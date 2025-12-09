// Advent of Code 2021 - Day 01

pub fn solution_2021_01_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let content: Vec<i64> = std::fs::read_to_string(filepath)?
        .trim()
        .lines()
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    
    let mut last = content[0];
    let mut count = 0;
    for &item in &content[1..] {
        if item > last {
            count += 1;
        }
        last = item;
    }
    Ok(count)
}

pub fn solution_2021_01_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let content: Vec<i64> = std::fs::read_to_string(filepath)?
        .trim()
        .lines()
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    
    let mut last: i64 = content[0..3].iter().sum();
    let mut count = 0;
    for n in 1..content.len() - 2 {
        let sum: i64 = content[n..n + 3].iter().sum();
        if sum > last {
            count += 1;
        }
        last = sum;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_01_01() {
        let result = solution_2021_01_01("inputs/2021/day01e.txt".to_string()).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    #[ignore]
    fn output_2021_01_01() {
        let result = solution_2021_01_01("inputs/2021/day01.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert_eq!(result, 1696);
    }

    #[test]
    fn test_2021_01_02() {
        let result = solution_2021_01_02("inputs/2021/day01e.txt".to_string()).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    #[ignore]
    fn output_2021_01_02() {
        let result = solution_2021_01_02("inputs/2021/day01.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert_eq!(result, 1737);
    }
}
