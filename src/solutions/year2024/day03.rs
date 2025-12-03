use regex::Regex;

pub fn solution_2024_03_01(filepath: String) -> Option<i32> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = mul_regex
        .captures_iter(&std::fs::read_to_string(filepath).unwrap())
        .fold(0, |acc, captures| {
            let (_, [num1, num2]) = captures.extract();
            acc + (num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap())
        });
    Some(result)
}

pub fn solution_2024_03_02(filepath: String) -> Option<i32> {
    let instructions_regex = Regex::new(r"(mul\((\d+),(\d+)\))|(don't\(\))|(do\(\))").unwrap();
    let mut enabled = true;
    let result = instructions_regex
        .captures_iter(&std::fs::read_to_string(filepath).unwrap())
        .fold(0, |acc, captures| {
            let name = captures.get(0).unwrap().as_str();
            match name {
                "don't()" => enabled = false,
                "do()" => enabled = true,
                _ => {
                    if enabled {
                        return acc
                            + (captures.get(2).unwrap().as_str().parse::<i32>().unwrap()
                                * captures.get(3).unwrap().as_str().parse::<i32>().unwrap());
                    }
                }
            }
            acc
        });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2024_03_01() {
        let file_path = String::from("inputs/2024/day03.txt");
        let result = solution_2024_03_01(file_path).unwrap();
        dbg!(result);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_2024_03_02() {
        let file_path = String::from("inputs/2024/day03.txt");
        let result = solution_2024_03_02(file_path).unwrap();
        assert_eq!(result, 93465710);
    }
}
