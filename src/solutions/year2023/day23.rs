// Advent of Code 2023 - Day 23

use std::collections::{HashMap, HashSet, VecDeque};

enum Tile {
    Path,
    Tree,
    Up,
    Down,
    Left,
    Right,
}

struct Forest {
    grid: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
}

impl Forest {
    fn from_string(s: &str) -> anyhow::Result<Forest> {
        let mut grid: Vec<Vec<Tile>> = Vec::new();

        for (y, line) in s.lines().enumerate() {
            let mut row: Vec<Tile> = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let tile = match ch {
                    '.' => Tile::Path,
                    '#' => Tile::Tree,
                    '^' => Tile::Up,
                    'v' => Tile::Down,
                    '<' => Tile::Left,
                    '>' => Tile::Right,
                    _ => anyhow::bail!("Invalid character in forest: {}", ch),
                };
                row.push(tile);
            }
            grid.push(row);
        }

        // Start is the first Path tile in the top row
        let mut start = None;
        for (x, tile) in grid[0].iter().enumerate() {
            if let Tile::Path = tile {
                start = Some((x, 0));
                break;
            }
        }
        // End is the last Path tile in the bottom row
        let mut end = None;
        let last_row = grid.len() - 1;
        for (x, tile) in grid[last_row].iter().enumerate().rev() {
            if let Tile::Path = tile {
                end = Some((x, last_row));
                break;
            }
        }
        let height = grid.len();
        let width = grid[0].len();
        Ok(Forest {
            grid,
            start: start.unwrap(),
            end: end.unwrap(),
            height,
            width,
        })
    }

    fn available_moves(
        &self,
        position: (usize, usize),
        disregard_slopes: bool,
    ) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = position;

        // Check current tile for slope restrictions
        if !disregard_slopes {
            match &self.grid[y][x] {
                Tile::Up => {
                    if y > 0 {
                        moves.push((x, y - 1));
                    }
                    return moves;
                }
                Tile::Down => {
                    if y < self.height - 1 {
                        moves.push((x, y + 1));
                    }
                    return moves;
                }
                Tile::Left => {
                    if x > 0 {
                        moves.push((x - 1, y));
                    }
                    return moves;
                }
                Tile::Right => {
                    if x < self.width - 1 {
                        moves.push((x + 1, y));
                    }
                    return moves;
                }
                _ => {}
            }
        }

        let directions = vec![
            (0isize, -1isize), // Up
            (0, 1),            // Down
            (-1, 0),           // Left
            (1, 0),            // Right
        ];

        for (dx, dy) in directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0
                && new_y >= 0
                && (new_y as usize) < self.height
                && (new_x as usize) < self.width
            {
                match self.grid[new_y as usize][new_x as usize] {
                    Tile::Tree => {}
                    _ => {
                        moves.push((new_x as usize, new_y as usize));
                    }
                }
            }
        }
        moves
    }

    // Build a compressed graph of junctions and their connections
    fn build_graph(
        &self,
        disregard_slopes: bool,
    ) -> HashMap<(usize, usize), Vec<((usize, usize), usize)>> {
        let mut graph: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();
        let mut junctions = HashSet::new();

        // Find all junctions (nodes with != 2 neighbors)
        junctions.insert(self.start);
        junctions.insert(self.end);

        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.grid[y][x], Tile::Tree) {
                    continue;
                }
                let neighbors = self.available_moves((x, y), disregard_slopes);
                if neighbors.len() > 2 {
                    junctions.insert((x, y));
                }
            }
        }

        // For each junction, explore paths to other junctions
        for &junction in &junctions {
            let mut edges = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back((junction, 0, junction));
            let mut visited = HashSet::new();

            while let Some((pos, dist, prev)) = queue.pop_front() {
                if dist > 0 && junctions.contains(&pos) {
                    edges.push((pos, dist));
                    continue;
                }

                for next in self.available_moves(pos, disregard_slopes) {
                    if next == prev {
                        continue;
                    }
                    if visited.insert((next, pos)) {
                        queue.push_back((next, dist + 1, pos));
                    }
                }
            }

            graph.insert(junction, edges);
        }

        graph
    }

    fn find_longest_path_in_graph(
        &self,
        graph: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
    ) -> usize {
        let mut max_dist = 0;
        let mut stack = vec![(self.start, 0, 0u128)];

        // Map positions to bit indices
        let mut pos_to_bit: HashMap<(usize, usize), u32> = HashMap::new();
        for (idx, &pos) in graph.keys().enumerate() {
            pos_to_bit.insert(pos, idx as u32);
        }

        let start_bit = 1u128 << pos_to_bit[&self.start];

        while let Some((pos, dist, visited)) = stack.pop() {
            if pos == self.end {
                max_dist = max_dist.max(dist);
                continue;
            }

            if let Some(edges) = graph.get(&pos) {
                for &(next_pos, edge_dist) in edges {
                    let next_bit = 1u128 << pos_to_bit[&next_pos];
                    if visited & next_bit == 0 {
                        stack.push((next_pos, dist + edge_dist, visited | next_bit));
                    }
                }
            }
        }

        max_dist
    }
}

struct Explorer {
    position: (usize, usize),
    visited: HashSet<(usize, usize)>,
}

impl Explorer {
    fn new(start: (usize, usize)) -> Explorer {
        Explorer {
            position: start,
            visited: HashSet::new(),
        }
    }

    fn find_longest_path(&mut self, forest: &Forest, disregard_slopes: bool) -> usize {
        self.visited.insert(self.position);
        let mut max_length = 0;

        for next_pos in forest.available_moves(self.position, disregard_slopes) {
            if !self.visited.contains(&next_pos) {
                let mut new_explorer = Explorer {
                    position: next_pos,
                    visited: self.visited.clone(),
                };
                let path_length = new_explorer.find_longest_path(forest, disregard_slopes);
                max_length = max_length.max(path_length);
            }
        }

        self.visited.remove(&self.position);
        if self.position == forest.end {
            return self.visited.len();
        }
        max_length
    }
}

pub fn solution_2023_23_01(file_path: String) -> anyhow::Result<usize> {
    let input = std::fs::read_to_string(file_path)?;
    let forest = Forest::from_string(&input)?;
    let mut explorer = Explorer::new(forest.start);
    let longest_path_length = explorer.find_longest_path(&forest, false);
    Ok(longest_path_length)
}

pub fn solution_2023_23_02(file_path: String) -> anyhow::Result<usize> {
    let input = std::fs::read_to_string(file_path)?;
    let forest = Forest::from_string(&input)?;
    let graph = forest.build_graph(true);
    let longest_path_length = forest.find_longest_path_in_graph(&graph);
    Ok(longest_path_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_23_01() {
        let file_path: String = String::from("inputs/2023/day23e.txt");
        let result = solution_2023_23_01(file_path).unwrap();
        assert_eq!(result, 94);
    }

    #[test]
    fn test_2023_23_02() {
        let file_path: String = String::from("inputs/2023/day23e.txt");
        let result = solution_2023_23_02(file_path).unwrap();
        assert_eq!(result, 154);
    }

    #[test]
    #[ignore]
    fn output_2023_23_01() {
        let file_path: String = String::from("inputs/2023/day23.txt");
        let result = solution_2023_23_01(file_path).unwrap();
        assert_eq!(result, 2074);
    }

    #[test]
    #[ignore]
    fn output_2023_23_02() {
        let file_path: String = String::from("inputs/2023/day23.txt");
        let result = solution_2023_23_02(file_path).unwrap();
        assert_eq!(result, 6494);
    }
}
