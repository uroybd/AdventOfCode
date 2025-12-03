// Advent of Code 2023 - Day 14

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};

use derive_deref::{Deref, DerefMut};

#[derive(Debug, Clone, Hash, Deref, DerefMut)]
struct Platform(Vec<Vec<char>>);

#[derive(Debug, PartialEq, Eq)]
struct ParsePlatformError;

impl std::str::FromStr for Platform {
    type Err = ParsePlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Platform(s.lines().map(|l| l.chars().collect()).collect()))
    }
}

impl Platform {
    fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    fn tilt_north(&mut self) {
        let width = self[0].len();
        let height = self.len();
        for y in 0..height {
            for x in 0..width {
                match self[y][x] {
                    '#' | '.' => (),
                    'O' => {
                        let mut move_to = y;
                        for i in (0..y).rev() {
                            match self[i][x] {
                                '#' | 'O' => break,
                                '.' => move_to = i,
                                _ => (),
                            }
                        }
                        let old = self[move_to][x];
                        self[move_to][x] = 'O';
                        self[y][x] = old;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let width = self[0].len();
        let height = self.len();
        for y in (0..height).rev() {
            for x in 0..width {
                match self[y][x] {
                    '#' | '.' => (),
                    'O' => {
                        let mut move_to = y;
                        for i in y + 1..height {
                            match self[i][x] {
                                '#' | 'O' => break,
                                '.' => move_to = i,
                                _ => (),
                            }
                        }

                        let old = self[move_to][x];
                        self[move_to][x] = 'O';
                        self[y][x] = old;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let width = self[0].len();
        let height = self.len();
        for y in 0..height {
            for x in (0..width).rev() {
                match self[y][x] {
                    '#' | '.' => (),
                    'O' => {
                        let mut move_to = x;
                        for i in x + 1..width {
                            match self[y][i] {
                                '#' | 'O' => break,
                                '.' => move_to = i,
                                _ => (),
                            }
                        }

                        let old = self[y][move_to];
                        self[y][move_to] = 'O';
                        self[y][x] = old;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let width = self[0].len();
        let height = self.len();
        for y in 0..height {
            for x in 0..width {
                match self[y][x] {
                    '#' | '.' => (),
                    'O' => {
                        let mut move_to = x;
                        for i in (0..x).rev() {
                            match self[y][i] {
                                '#' | 'O' => break,
                                '.' => move_to = i,
                                _ => (),
                            }
                        }

                        let old = self[y][move_to];
                        self[y][move_to] = 'O';
                        self[y][x] = old;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn tilt_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn calculate_load(&self) -> usize {
        let mut load = 0;
        let height = self.len();
        for (y, row) in self.iter().enumerate() {
            for c in row.iter() {
                if *c == 'O' {
                    load += height - y;
                }
            }
        }
        load
    }
}

pub fn solution_2023_14_01(file_path: String) -> Option<usize> {
    let mut platform = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .parse::<Platform>()
        .unwrap();
    platform.tilt_north();
    Some(platform.calculate_load())
}

pub fn solution_2023_14_02(file_path: String) -> Option<usize> {
    let mut platform = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .parse::<Platform>()
        .unwrap();
    let mut seen: HashMap<u64, usize> = HashMap::new();
    for i in 0..1000000000 {
        let hash = platform.get_hash();
        if let std::collections::hash_map::Entry::Vacant(e) = seen.entry(hash) {
            platform.tilt_cycle();
            e.insert(i);
        } else {
            let cycle = i - seen.get(&hash).unwrap();
            let remaining = 1000000000 - i;
            let remaining = remaining % cycle;
            for _ in 0..remaining {
                platform.tilt_cycle();
            }
            break;
        }
    }
    Some(platform.calculate_load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_14_01() {
        let file_path: String = String::from("inputs/2023/day14e.txt");
        let result = solution_2023_14_01(file_path).unwrap();
        assert_eq!(result, 136);
    }

    #[test]
    fn test_2023_14_02() {
        let file_path: String = String::from("inputs/2023/day14e.txt");
        let result = solution_2023_14_02(file_path).unwrap();
        assert_eq!(result, 64);
    }

    #[test]
    #[ignore]
    fn output_day_14_01() {
        let file_path: String = String::from("inputs/2023/day14.txt");
        let result = solution_2023_14_01(file_path).unwrap();
        assert_eq!(result, 108889);
    }

    #[test]
    #[ignore]
    fn output_day_14_02() {
        let file_path: String = String::from("inputs/2023/day14.txt");
        let result = solution_2023_14_02(file_path).unwrap();
        assert_eq!(result, 104671);
    }
}
