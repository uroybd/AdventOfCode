// Advent of Code 2025 - Day 07

use std::collections::HashSet;

struct TachyonManifold {
    schema: Vec<Vec<char>>,
    start: (usize, usize),
}

impl TachyonManifold {
    fn new(schema: Vec<Vec<char>>, start: (usize, usize)) -> Self {
        Self { schema, start }
    }

    fn from_str(inp: &str) -> Self {
        let mut schema: Vec<Vec<char>> = Vec::new();
        let mut start: (usize, usize) = (0, 0);

        for (i, line) in inp.lines().enumerate() {
            let row: Vec<char> = line.chars().collect();
            for (j, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    start = (j, i);
                }
            }
            schema.push(row);
        }
        Self::new(schema, start)
    }

    fn count_splits(&self) -> usize {
        let mut split_count = 0;
        let mut tachyons = HashSet::new();
        tachyons.insert(self.start);
        let height = self.schema.len();
        while !tachyons.is_empty() {
            println!("Tachyons at positions: {:?}", tachyons);
            let mut new_tachyons = HashSet::new();
            for (x, y) in tachyons.iter() {
                if y + 1 >= height {
                    continue;
                }
                let below = self.schema[y + 1][*x];
                match below {
                    '^' => {
                        new_tachyons.insert((*x + 1, y + 1));
                        new_tachyons.insert((*x - 1, y + 1));
                        split_count += 1;
                    }
                    _ => {
                        new_tachyons.insert((*x, y + 1));
                    }
                }
            }
            println!("New tachyons at positions: {:?}", new_tachyons);
            tachyons = new_tachyons;
        }
        split_count
    }

    // Find all possible timelines from start to the bottom of the manifold.
    // Using dynamic programming to count the number of ways to reach each cell.
    fn count_timeline(&self) -> usize {
        let height = self.schema.len();
        let width = self.schema[0].len();
        let mut dp = vec![vec![0usize; width]; height];
        dp[self.start.1][self.start.0] = 1;

        for row in self.start.1..height - 1 {
            for col in 0..width {
                let count = dp[row][col];
                if count == 0 {
                    continue;
                }

                let below = self.schema[row + 1][col];
                match below {
                    '^' => {
                        // Split: distribute timelines to left and right
                        if col > 0 {
                            dp[row + 1][col - 1] += count;
                        }
                        if col < width - 1 {
                            dp[row + 1][col + 1] += count;
                        }
                    }
                    _ => {
                        // Continue straight down
                        dp[row + 1][col] += count;
                    }
                }
            }
        }

        // Sum all timelines that reached the last row
        dp[height - 1].iter().sum()
    }
}

pub fn solution_2025_07_01(file_path: String) -> anyhow::Result<usize> {
    let manifold = TachyonManifold::from_str(&std::fs::read_to_string(file_path)?);
    Ok(manifold.count_splits())
}

pub fn solution_2025_07_02(file_path: String) -> anyhow::Result<usize> {
    let manifold = TachyonManifold::from_str(&std::fs::read_to_string(file_path)?);
    Ok(manifold.count_timeline())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_07_01() {
        let file_path: String = String::from("inputs/2025/day07e.txt");
        let result = solution_2025_07_01(file_path).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_2025_07_02() {
        let file_path: String = String::from("inputs/2025/day07e.txt");
        let result = solution_2025_07_02(file_path).unwrap();
        assert_eq!(result, 40);
    }

    #[test]
    #[ignore]
    fn output_2025_07_01() {
        let file_path: String = String::from("inputs/2025/day07.txt");
        let result = solution_2025_07_01(file_path).unwrap();
        assert_eq!(result, 1587);
    }

    #[test]
    #[ignore]
    fn output_2025_07_02() {
        let file_path: String = String::from("inputs/2025/day07.txt");
        let result = solution_2025_07_02(file_path).unwrap();
        assert_eq!(result, 5748679033029);
    }
}
