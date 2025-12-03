use std::fs;

// Advent of Code 2023 - Day 04
fn win_count(inp: &str) -> usize {
    let (_, numbers) = inp.split_once(':').unwrap();
    let (winning, available) = numbers.split_once(" | ").unwrap();
    let winning: Vec<usize> = winning
        .split_whitespace()
        .map(|n| n.trim().parse().unwrap())
        .collect();
    available
        .split_whitespace()
        .map(|n| n.trim().parse().unwrap())
        .filter(|a| winning.contains(a))
        .count()
}

pub fn points(count: usize) -> usize {
    if count < 3 {
        return count;
    }
    2_usize.pow((count - 1).try_into().unwrap())
}

fn total_won(cards: &Vec<usize>) -> usize {
    let mut count_cache = vec![1; cards.len()];
    for (index, card_win) in cards.iter().enumerate() {
        if card_win > &0 {
            for x in index + 1..=(index + card_win) {
                count_cache[x] += count_cache[index]
            }
        }
    }
    count_cache.iter().sum()
}

pub fn solution_day_04_01(file_path: String) -> Option<usize> {
    Some(
        fs::read_to_string(file_path)
            .expect("Invalid File")
            .lines()
            .map(|l| points(win_count(l)))
            .sum(),
    )
}

pub fn solution_day_04_02(file_path: String) -> Option<usize> {
    let cards: Vec<usize> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .lines()
        .map(win_count)
        .collect();
    Some(total_won(&cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_01() {
        let file_path: String = String::from("inputs/2023/day04e.txt");
        let result = solution_day_04_01(file_path).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_day_04_02() {
        let file_path: String = String::from("inputs/2023/day04e.txt");
        let result = solution_day_04_02(file_path).unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    #[ignore]
    fn output_day_04_01() {
        let file_path: String = String::from("inputs/2023/day04.txt");
        let result = solution_day_04_01(file_path).unwrap();
        assert_eq!(result, 25651);
    }

    #[test]
    #[ignore]
    fn output_day_04_02() {
        let file_path: String = String::from("inputs/2023/day04.txt");
        let result = solution_day_04_02(file_path).unwrap();
        assert_eq!(result, 19499881);
    }
}
