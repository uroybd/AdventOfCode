// Advent of Code 2021 - Day 14

use std::collections::HashMap;
use std::fs;

fn create_rules(data: &[String]) -> HashMap<String, char> {
    let rules: HashMap<String, char> = data
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            (
                parts[0].to_string(),
                parts[1].to_string().chars().next().unwrap(),
            )
        })
        .collect();
    rules
}

fn polymerize(
    template: &Vec<char>,
    current_step: usize,
    limit: usize,
    rules: &HashMap<String, char>,
    cache: &mut HashMap<String, HashMap<char, usize>>,
) -> HashMap<char, usize> {
    let mut count: HashMap<char, usize> = HashMap::new();

    let length = template.len();
    for i in 0..(length - 1) {
        let (current, end) = (template[i], template[i + 1]);
        let key: String = format!("{}{}", current, end);
        let cache_key = format!("{}-{}", key, current_step);
        let cached_val = cache.get(&cache_key);
        match cached_val {
            Some(cached_map) => {
                for (key, val) in cached_map {
                    *count.entry(*key).or_insert(0) += val;
                }
            }
            None => {
                let val = rules.get(&key);
                match val {
                    Some(&c) => {
                        if current_step == limit {
                            let mut result: Vec<char> = vec![];
                            result.push(current);
                            result.push(c);
                            let s_count = count_items(&result);
                            for (key, val) in s_count {
                                *count.entry(key).or_insert(0) += val;
                            }
                        } else {
                            let v = vec![current, c, end];
                            let nested_values =
                                polymerize(&v, current_step + 1, limit, rules, cache);

                            cache.insert(cache_key, nested_values.clone());
                            for (key, val) in nested_values {
                                *count.entry(key).or_insert(0) += val;
                            }
                        }
                    }
                    None => (),
                }
            }
        }
    }
    count
}

fn count_items(template: &Vec<char>) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    for item in template {
        *result.entry(*item).or_insert(0) += 1;
    }
    result
}

fn solve(filepath: String, limit: usize) -> usize {
    let data: Vec<String> = fs::read_to_string(filepath)
        .expect("Invalid File")
        .lines()
        .map(|s| s.to_string())
        .collect();
    let template: Vec<char> = data[0].chars().collect();
    let rules = create_rules(&data[2..]);
    let mut cache: HashMap<String, HashMap<char, usize>> = HashMap::new();
    let counts_map = polymerize(&template, 1, limit, &rules, &mut cache);
    let mut temp_counts = count_items(&template);
    for (key, val) in counts_map {
        *temp_counts.entry(key).or_insert(0) += val;
    }
    let mut counts: Vec<usize> = temp_counts.into_values().collect();
    counts.sort();
    counts[counts.len() - 1] - counts[0]
}

pub fn solution_2021_14_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(solve(filepath, 10) as i64)
}

pub fn solution_2021_14_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(solve(filepath, 40) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_14_01() {
        let result = solution_2021_14_01("inputs/2021/day14e.txt".to_string()).unwrap();
        assert_eq!(result, 1588);
    }

    #[test]
    #[ignore]
    fn output_2021_14_01() {
        let result = solution_2021_14_01("inputs/2021/day14.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_14_02() {
        let result = solution_2021_14_02("inputs/2021/day14e.txt".to_string()).unwrap();
        assert_eq!(result, 2188189693529);
    }

    #[test]
    #[ignore]
    fn output_2021_14_02() {
        let result = solution_2021_14_02("inputs/2021/day14.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
