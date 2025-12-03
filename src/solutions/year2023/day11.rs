// Advent of Code 2023 - Day 11

use std::{collections::BTreeSet, fs};

struct Observation {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseObservationError;

impl std::str::FromStr for Observation {
    type Err = ParseObservationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies: Vec<(usize, usize)> = vec![];
        let mut xs: BTreeSet<usize> = BTreeSet::new();
        let mut ys: BTreeSet<usize> = BTreeSet::new();

        for (y, row) in s.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                    xs.insert(x);
                    ys.insert(y);
                }
            }
        }

        Ok(Observation {
            galaxies,
            empty_cols: (*xs.first().unwrap()..=*xs.last().unwrap())
                .filter(|i| !xs.contains(i))
                .collect(),
            empty_rows: (*ys.first().unwrap()..=*ys.last().unwrap())
                .filter(|i| !ys.contains(i))
                .collect(),
        })
    }
}

impl Observation {
    fn calculate_distance(
        &self,
        p1: &(usize, usize),
        p2: &(usize, usize),
        multiplier: usize,
    ) -> usize {
        let (mut xs, mut ys) = ([p1.0, p2.0], [p1.1, p2.1]);
        xs.sort();
        ys.sort();

        let expanded = self
            .empty_cols
            .iter()
            .filter(|x| x >= &&(xs[0] + 1) && x <= &&xs[1])
            .count()
            + self
                .empty_rows
                .iter()
                .filter(|y| y >= &&(ys[0] + 1) && y <= &&ys[1])
                .count();

        xs[1].abs_diff(xs[0]) + ys[1].abs_diff(ys[0]) + (expanded * multiplier) - expanded
    }

    fn get_all_galaxy_distances(&self, multiplier: usize) -> usize {
        let mut galaxies = self.galaxies.clone();

        let mut distances = 0;
        while let Some(v) = galaxies.pop() {
            distances += galaxies
                .iter()
                .map(|p| self.calculate_distance(p, &v, multiplier))
                .sum::<usize>();
        }
        distances
    }
}

pub fn solution_2023_11(file_path: String, multiplier: usize) -> Option<usize> {
    let data: Observation = fs::read_to_string(file_path)
        .expect("Invalid File")
        .parse()
        .unwrap();
    Some(data.get_all_galaxy_distances(multiplier))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_11_01() {
        let file_path: String = String::from("inputs/2023/day11e.txt");
        let result = solution_2023_11(file_path, 2).unwrap();
        assert_eq!(result, 374);
    }

    #[test]
    fn test_2023_11_02() {
        let file_path: String = String::from("inputs/2023/day11e.txt");
        let result = solution_2023_11(file_path, 100).unwrap();
        assert_eq!(result, 8410);
    }

    #[test]
    #[ignore]
    fn output_day_11_01() {
        let file_path: String = String::from("inputs/2023/day11.txt");
        let result = solution_2023_11(file_path, 2).unwrap();
        assert_eq!(result, 9742154);
    }

    #[test]
    #[ignore]
    fn output_day_11_02() {
        let file_path: String = String::from("inputs/2023/day11.txt");
        let result = solution_2023_11(file_path, 1000000).unwrap();
        assert_eq!(result, 411142919886);
    }
}
