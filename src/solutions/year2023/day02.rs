use std::fs;

use derive_deref::Deref;
// Advent of Code 2023 - Day 02
#[derive(Deref)]
struct Game([usize; 3]);

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl std::str::FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, turns) = s.split_once(": ").unwrap();
        let turn_max = turns.split([';', ',']).fold([0; 3], |mut rec, cdef| {
            let (val, color) = cdef.trim().split_once(' ').unwrap();
            let val = val.parse::<usize>().unwrap();
            let idx = match color {
                "red" => 0,
                "blue" => 1,
                "green" => 2,
                _ => unreachable!(),
            };
            rec[idx] = rec[idx].max(val);
            rec
        });
        Ok(Self(turn_max))
    }
}

impl Game {
    pub fn is_valid(&self, caps: &[usize; 3]) -> bool {
        for (idx, val) in self.iter().enumerate() {
            if val > &caps[idx] {
                return false;
            }
        }
        true
    }

    pub fn power(&self) -> usize {
        self.iter().copied().reduce(|acc, a| acc * a).unwrap()
    }
}

pub fn solution_2023_02_01(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .lines()
        .enumerate()
        .filter_map(|(idx, l)| {
            let g: Game = l.parse().unwrap();
            if g.is_valid(&[12, 14, 13]) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

pub fn solution_2023_02_02(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .lines()
        .map(|l| l.parse::<Game>().unwrap().power())
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_02_01() {
        let file_path: String = String::from("inputs/2023/day02e.txt");
        let result = solution_2023_02_01(file_path).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2023_02_02() {
        let file_path: String = String::from("inputs/2023/day02e.txt");
        let result = solution_2023_02_02(file_path).unwrap();
        assert_eq!(result, 2286);
    }

    #[test]
    #[ignore]
    fn output_day_02_01() {
        let file_path: String = String::from("inputs/2023/day02.txt");
        let result = solution_2023_02_01(file_path).unwrap();
        assert_eq!(result, 2727);
    }

    #[test]
    #[ignore]
    fn output_day_02_02() {
        let file_path: String = String::from("inputs/2023/day02.txt");
        let result = solution_2023_02_02(file_path).unwrap();
        assert_eq!(result, 56580);
    }
}
