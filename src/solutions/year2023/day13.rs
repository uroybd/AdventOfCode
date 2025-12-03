// Advent of Code 2023 - Day 13

use derive_deref::Deref;
use rayon::prelude::*;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Deref)]
struct Mirror(Vec<Vec<char>>);

#[derive(Debug, PartialEq, Eq)]
struct ParseMirrorError;

impl std::str::FromStr for Mirror {
    type Err = ParseMirrorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mirror(s.lines().map(|l| l.chars().collect()).collect()))
    }
}

fn symmetry_with_error(m1: &[Vec<char>], m2: &[Vec<char>], margin: usize) -> bool {
    let mut errors = 0;

    let height = m1.len();
    let width = m1[0].len();
    for i in 0..height {
        for j in 0..width {
            if m1[i][j] != m2[i][j] {
                errors += 1;
                if errors > margin {
                    return false;
                }
            }
        }
    }
    if margin > 0 {
        return errors > 0 && errors <= margin;
    }
    true
}

impl Mirror {
    fn split_h(&self, position: usize) -> (Mirror, Mirror) {
        let data = self.split_at(position);
        (Mirror(data.0.to_vec()), Mirror(data.1.to_vec()))
    }

    fn split_v(&self, position: usize) -> (Mirror, Mirror) {
        let mut top: Vec<Vec<char>> = Vec::new();
        let mut bottom: Vec<Vec<char>> = Vec::new();
        for row in self.iter() {
            let data = row.split_at(position);
            top.push(data.0.to_vec());
            bottom.push(data.1.to_vec());
        }
        (Mirror(top), Mirror(bottom))
    }

    fn flip_v(&self) -> Mirror {
        let data: Vec<Vec<char>> = self
            .iter()
            .map(|row| row.iter().rev().copied().collect())
            .collect();
        Mirror(data)
    }

    fn flip_h(&self) -> Mirror {
        let data: Vec<Vec<char>> = self.iter().rev().cloned().collect();
        Mirror(data)
    }

    fn horizontal_symmetry(&self, margin: usize) -> Option<(Mirror, Mirror)> {
        let height = self.len();
        for i in 1..height {
            let (top, bottom) = self.split_h(i);
            let bottom = bottom.flip_h();

            let min = top.len().min(bottom.len());
            if symmetry_with_error(
                &top[top.len() - min..],
                &bottom[bottom.len() - min..],
                margin,
            ) {
                return Some((top, bottom));
            }
        }
        None
    }

    fn vertical_symmetry(&self, margin: usize) -> Option<(Mirror, Mirror)> {
        let width = self[0].len();
        for i in 1..width {
            let (left, right) = self.split_v(i);
            let right = right.flip_v();
            let min = left[0].len().min(right[0].len());
            let left_data: Vec<Vec<char>> = left
                .iter()
                .map(|row| row[row.len() - min..].to_vec())
                .collect();
            let right_data: Vec<Vec<char>> = right
                .iter()
                .map(|row| row[row.len() - min..].to_vec())
                .collect();
            if symmetry_with_error(&left_data, &right_data, margin) {
                return Some((left, right));
            }
        }
        None
    }

    fn symmetry(&self, margin: usize) -> Option<((Mirror, Mirror), String)> {
        if let Some(result) = self.horizontal_symmetry(margin) {
            return Some((result, String::from("horizontal")));
        }
        if let Some(result) = self.vertical_symmetry(margin) {
            return Some((result, String::from("vertical")));
        }

        None
    }
    fn score(&self, margin: usize) -> usize {
        if let Some((data, s_type)) = self.symmetry(margin) {
            return match s_type.as_str() {
                "horizontal" => 100 * data.0.len(),
                "vertical" => data.0[0].len(),
                _ => 0,
            };
        }
        0
    }
}

pub fn solution_2023_13_01(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
        .expect("Invalid Input file.")
        .split("\n\n")
        .par_bridge()
        .map(|l| l.parse::<Mirror>().unwrap().score(0))
        .sum();
    Some(result)
}

pub fn solution_2023_13_02(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
        .expect("Invalid Input file.")
        .split("\n\n")
        .par_bridge()
        .map(|l| l.parse::<Mirror>().unwrap().score(1))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_13_01() {
        let file_path: String = String::from("inputs/2023/day13e.txt");
        let result = solution_2023_13_01(file_path).unwrap();
        assert_eq!(result, 405);
    }

    #[test]
    fn test_2023_13_02() {
        let file_path: String = String::from("inputs/2023/day13e.txt");
        let result = solution_2023_13_02(file_path).unwrap();
        assert_eq!(result, 400);
    }

    #[test]
    #[ignore]
    fn output_day_13_01() {
        let file_path: String = String::from("inputs/2023/day13.txt");
        let result = solution_2023_13_01(file_path).unwrap();
        assert_eq!(result, 34918);
    }

    #[test]
    #[ignore]
    fn output_day_13_02() {
        let file_path: String = String::from("inputs/2023/day13.txt");
        let result = solution_2023_13_02(file_path).unwrap();
        assert_eq!(result, 33054);
    }
}
