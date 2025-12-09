// Advent of Code 2021 - Day 08

use std::collections::HashMap;
use std::fs;

fn sort_chars(val: String) -> String {
    let mut chars: Vec<char> = val.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn diffs(val1: String, val2: String) -> Vec<char> {
    let mut diff: HashMap<char, bool> = HashMap::new();
    let val1_chars: Vec<char> = val1.chars().collect();
    let val2_chars: Vec<char> = val2.chars().collect();
    for v in val1_chars.iter() {
        if !val2_chars.contains(v) {
            diff.insert(*v, true);
        }
    }
    for v in val2_chars.iter() {
        if !val1_chars.contains(v) {
            diff.insert(*v, true);
        }
    }
    diff.iter().map(|(&k, _)| k).collect()
}

fn parse(data: &str) -> (Vec<String>, Vec<String>) {
    let mut splitted = data.split(" | ");
    let entries: Vec<String> = splitted
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect();
    let to_decode: Vec<String> = splitted
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect();
    (entries, to_decode)
}

fn count_unique(data: Vec<String>) -> usize {
    data.iter().fold(0, |acc, x| match x.len() {
        2 | 4 | 3 | 7 => acc + 1,
        _ => acc,
    })
}

pub fn solution_2021_08_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<String> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let result = data
        .iter()
        .fold(0, |acc, x| acc + count_unique(parse(x).1));
    Ok(result as i64)
}

fn decode_pattern(data: (Vec<String>, Vec<String>)) -> usize {
    let (clues, to_decode) = data;
    let (mut one, mut two, mut four, mut five, mut six, mut seven, mut eight, mut nine) = (
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    );
    for clue in clues.clone() {
        match clue.len() {
            2 => one = clue,
            4 => four = clue,
            3 => seven = clue,
            7 => eight = clue,
            _ => (),
        }
    }
    let one_chars: Vec<char> = one.chars().collect();
    let five_longs: Vec<String> = clues.clone().into_iter().filter(|x| x.len() == 5).collect();
    let three = five_longs
        .clone()
        .into_iter()
        .find(|x| {
            let chars: Vec<char> = x.chars().collect();
            chars.contains(&one_chars[0]) && chars.contains(&one_chars[1])
        })
        .unwrap();
    let b_and_e = diffs(three.clone(), eight.clone());
    let mut sides = vec![];
    sides.extend(one_chars);
    sides.extend(b_and_e.clone());
    let four_chars: Vec<char> = four.chars().collect();
    let d_char = four_chars.iter().find(|&x| !sides.contains(x)).unwrap();
    let six_longs: Vec<String> = clues.into_iter().filter(|x| x.len() == 6).collect();
    let (sixty_nine, zeros): (Vec<String>, Vec<String>) = six_longs.into_iter().partition(|x| {
        let chars: Vec<char> = x.chars().collect();
        chars.contains(d_char)
    });
    let zero = zeros[0].clone();
    for number in sixty_nine.into_iter() {
        let chars: Vec<char> = number.chars().collect();
        if chars.contains(&one.chars().next().unwrap())
            && chars.contains(&one.chars().nth(1).unwrap())
        {
            nine = number;
        } else {
            six = number;
        }
    }
    let e_diff = diffs(eight.clone(), nine.clone());
    let e_char = e_diff.first().unwrap();
    let b_char = b_and_e.into_iter().find(|&x| x != *e_char).unwrap();
    let two_and_five: Vec<String> = five_longs.into_iter().filter(|x| x != &three).collect();
    for number in two_and_five.into_iter() {
        let chars: Vec<char> = number.chars().collect();
        if chars.contains(e_char) {
            two = number;
        } else if chars.contains(&b_char) {
            five = number;
        }
    }
    let num_map = HashMap::from([
        (sort_chars(zero), '0'),
        (sort_chars(one), '1'),
        (sort_chars(two), '2'),
        (sort_chars(three), '3'),
        (sort_chars(four), '4'),
        (sort_chars(five), '5'),
        (sort_chars(six), '6'),
        (sort_chars(seven), '7'),
        (sort_chars(eight), '8'),
        (sort_chars(nine), '9'),
    ]);
    to_decode
        .into_iter()
        .map(|digit| {
            let sorted = sort_chars(digit);
            let val = num_map.get(&sorted).unwrap();
            *val
        })
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

pub fn solution_2021_08_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<String> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let result = data
        .iter()
        .fold(0, |acc, x| acc + decode_pattern(parse(x)));
    Ok(result as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_08_01() {
        let result = solution_2021_08_01("inputs/2021/day08e.txt".to_string()).unwrap();
        assert_eq!(result, 26);
    }

    #[test]
    #[ignore]
    fn output_2021_08_01() {
        let result = solution_2021_08_01("inputs/2021/day08.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_08_02() {
        let result = solution_2021_08_02("inputs/2021/day08e.txt".to_string()).unwrap();
        assert_eq!(result, 61229);
    }

    #[test]
    #[ignore]
    fn output_2021_08_02() {
        let result = solution_2021_08_02("inputs/2021/day08.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
