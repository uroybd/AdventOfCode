// Advent of Code 2025 - Day 04

use std::fs;

struct Diagram {
    grid: Vec<Vec<bool>>,
    height: i32,
    width: i32,
}

impl Diagram {
    fn from_string(inp: &str) -> Self {
        let grid: Vec<Vec<bool>> = inp
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect();
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;
        Self {
            grid,
            height,
            width,
        }
    }
    fn removable(&self) -> Vec<(usize, usize)> {
        let mut removable = vec![];
        for r in 0..self.height as usize {
            for c in 0..self.width as usize {
                if self.grid[r][c] && self.get_adjecent_count(r, c) < 4 {
                    removable.push((r, c))
                }
            }
        }
        removable
    }
    fn remove_some(&mut self) -> usize {
        let to_remove = self.removable();
        for (r, c) in to_remove.clone() {
            self.grid[r][c] = false;
        }
        to_remove.len()
    }
    fn get_adjecent_count(&self, row: usize, col: usize) -> usize {
        let directions = [
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
            .iter()
            .filter_map(|(r, c)| {
                let (nr, nc) = (r + (row as i32), c + (col as i32));
                if nr < 0 || nr >= self.width || nc < 0 || nc >= self.height {
                    return None;
                }
                let val = self.grid[nr as usize][nc as usize];
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
