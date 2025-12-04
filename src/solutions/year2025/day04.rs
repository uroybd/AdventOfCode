// Advent of Code 2025 - Day 04

use std::fs;

struct Diagram {
    grid: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Diagram {
    fn from_string(inp: &str) -> Self {
        let grid: Vec<Vec<bool>> = inp
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }
    fn removable(&self) -> Vec<(usize, usize)> {
        (0..self.height)
            .flat_map(|r| {
                (0..self.width).filter_map(move |c| {
                    if self.grid[r][c] && self.has_neighbors_less_than(r, c, 4) {
                        return Some((r, c));
                    }
                    None
                })
            })
            .collect()
    }
    fn remove_some(&mut self) -> usize {
        let to_remove = self.removable();
        for (r, c) in to_remove.clone() {
            self.grid[r][c] = false;
        }
        to_remove.len()
    }
    // A much elegant solution was to use map and filter map. This, however, returns early.
    fn has_neighbors_less_than(&self, row: usize, col: usize, limit: usize) -> bool {
        let mut count = 0;
        for (dr, dc) in DIRECTIONS {
            let (nr, nc) = (row.wrapping_add_signed(dr), col.wrapping_add_signed(dc));
            if nr >= self.width || nc >= self.height {
                continue;
            }
            if self.grid[nr][nc] {
                count += 1;
                if count >= limit {
                    return false;
                }
            }
        }
        count < limit
    }
}

pub fn solution_2025_04_01(file_path: String) -> anyhow::Result<usize> {
    let diagram = Diagram::from_string(&fs::read_to_string(file_path).expect("Input not found"));
    Ok(diagram.removable().len())
}

pub fn solution_2025_04_02(file_path: String) -> anyhow::Result<usize> {
    let mut diagram =
        Diagram::from_string(&fs::read_to_string(file_path).expect("Input not found"));
    let mut result = 0;
    loop {
        let c = diagram.remove_some();
        if c == 0 {
            break;
        }
        result += c;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_04_01() {
        let file_path: String = String::from("inputs/2025/day04e.txt");
        let result = solution_2025_04_01(file_path).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_2025_04_02() {
        let file_path: String = String::from("inputs/2025/day04e.txt");
        let result = solution_2025_04_02(file_path).unwrap();
        assert_eq!(result, 43);
    }

    #[test]
    #[ignore]
    fn output_2025_04_01() {
        let file_path: String = String::from("inputs/2025/day04.txt");
        let result = solution_2025_04_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_2025_04_02() {
        let file_path: String = String::from("inputs/2025/day04.txt");
        let result = solution_2025_04_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
