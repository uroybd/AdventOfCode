// Advent of Code 2025 - Day 04

use std::fs;

struct Diagram {
    grid: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

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
                    if self.grid[r][c] && self.get_adjecent_count(r, c) < 4 {
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
    fn get_adjecent_count(&self, row: usize, col: usize) -> usize {
        let directions: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        directions
            .into_iter()
            .filter_map(|(r, c)| {
                let (nr, nc) = (row.wrapping_add_signed(r), col.wrapping_add_signed(c));
                if nr >= self.width || nc >= self.height {
                    return None;
                }
                let val = self.grid[nr][nc];
                if val {
                    return Some(val);
                }
                None
            })
            .count()
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
