// Advent of Code 2023 - Day 22

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Coordinate(usize, usize, usize);

impl Coordinate {
    fn from_string(s: &str) -> anyhow::Result<Coordinate> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 3 {
            anyhow::bail!("Invalid coordinate string: {}", s);
        }
        let x = parts[0].parse::<usize>()?;
        let y = parts[1].parse::<usize>()?;
        let z = parts[2].parse::<usize>()?;
        Ok(Coordinate(x, y, z))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick(Coordinate, Coordinate);

impl Brick {
    fn from_string(s: &str) -> anyhow::Result<Brick> {
        let parts: Vec<&str> = s.trim().split("~").collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid brick string: {}", s);
        }
        let start = Coordinate::from_string(parts[0])?;
        let end = Coordinate::from_string(parts[1])?;
        Ok(Brick(start, end))
    }
    fn xy_overlaps(&self, other: &Brick) -> bool {
        let Brick(Coordinate(x1_min, y1_min, _), Coordinate(x1_max, y1_max, _)) = self;
        let Brick(Coordinate(x2_min, y2_min, _), Coordinate(x2_max, y2_max, _)) = other;

        !(x1_max < x2_min || x2_max < x1_min || y1_max < y2_min || y2_max < y1_min)
    }
    fn move_down(&self, dz: usize) -> Brick {
        let Brick(Coordinate(x_min, y_min, z_min), Coordinate(x_max, y_max, z_max)) = self;
        Brick(
            Coordinate(*x_min, *y_min, z_min - dz),
            Coordinate(*x_max, *y_max, z_max - dz),
        )
    }
    fn min_z(&self) -> usize {
        self.0 .2
    }
    fn max_z(&self) -> usize {
        self.1 .2
    }
}

#[derive(Debug, Clone)]
struct Wall {
    bricks: Vec<Brick>,
    supports: HashMap<usize, HashSet<usize>>, // brick i supports these bricks
    supported_by: HashMap<usize, HashSet<usize>>, // brick i is supported by these bricks
}

impl Wall {
    fn from_string(inp: &str) -> anyhow::Result<Wall> {
        let mut bricks = Vec::new();
        for line in inp.lines() {
            let brick = Brick::from_string(line)?;
            bricks.push(brick);
        }
        bricks.sort_by_key(|b| b.min_z());
        Ok(Wall {
            bricks,
            supports: HashMap::new(),
            supported_by: HashMap::new(),
        })
    }

    fn settle(&mut self) {
        let n = self.bricks.len();
        let mut settled = Vec::new();

        for i in 0..n {
            let brick = &self.bricks[i];
            let mut max_z = 0;

            // Find the highest z this brick can rest on
            for settled_brick in &settled {
                if brick.xy_overlaps(settled_brick) {
                    max_z = max_z.max(settled_brick.max_z());
                }
            }

            let fall_distance = brick.min_z() - max_z - 1;
            let new_brick = if fall_distance > 0 {
                brick.move_down(fall_distance)
            } else {
                brick.clone()
            };

            settled.push(new_brick);
        }

        self.bricks = settled;
    }

    fn build_support_graph(&mut self) {
        let n = self.bricks.len();
        self.supports = HashMap::new();
        self.supported_by = HashMap::new();

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let brick_i = &self.bricks[i];
                let brick_j = &self.bricks[j];

                // Check if brick_i supports brick_j (j is directly above i)
                if brick_i.max_z() + 1 == brick_j.min_z() && brick_i.xy_overlaps(brick_j) {
                    self.supports
                        .entry(i)
                        .or_insert_with(HashSet::new)
                        .insert(j);
                    self.supported_by
                        .entry(j)
                        .or_insert_with(HashSet::new)
                        .insert(i);
                }
            }
        }
    }

    fn count_removable_bricks(&self) -> usize {
        let mut count = 0;
        for i in 0..self.bricks.len() {
            // A brick can be removed if all bricks it supports have at least one other support
            let can_remove = self
                .supports
                .get(&i)
                .map(|supported| {
                    supported.iter().all(|&j| {
                        self.supported_by
                            .get(&j)
                            .map(|s| s.len() > 1)
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(true);

            if can_remove {
                count += 1;
            }
        }
        count
    }

    fn count_chain_reaction(&self, brick_idx: usize) -> usize {
        let mut fallen = HashSet::new();
        fallen.insert(brick_idx);
        let mut changed = true;

        while changed {
            changed = false;
            for i in 0..self.bricks.len() {
                if fallen.contains(&i) {
                    continue;
                }
                // Check if brick i would fall
                if let Some(supporters) = self.supported_by.get(&i) {
                    if !supporters.is_empty() && supporters.iter().all(|s| fallen.contains(s)) {
                        fallen.insert(i);
                        changed = true;
                    }
                }
            }
        }

        fallen.len() - 1 // Don't count the original brick
    }

    fn count_supported_chained(&self) -> usize {
        (0..self.bricks.len())
            .map(|i| self.count_chain_reaction(i))
            .sum()
    }
}

pub fn solution_2023_22_01(file_path: String) -> anyhow::Result<usize> {
    let mut wall = Wall::from_string(&std::fs::read_to_string(file_path)?)?;
    wall.settle();
    wall.build_support_graph();
    Ok(wall.count_removable_bricks())
}

pub fn solution_2023_22_02(file_path: String) -> anyhow::Result<usize> {
    let mut wall = Wall::from_string(&std::fs::read_to_string(file_path)?)?;
    wall.settle();
    wall.build_support_graph();
    Ok(wall.count_supported_chained())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_22_01() {
        let file_path: String = String::from("inputs/2023/day22e.txt");
        let result = solution_2023_22_01(file_path).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2023_22_02() {
        let file_path: String = String::from("inputs/2023/day22e.txt");
        let result = solution_2023_22_02(file_path).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    #[ignore]
    fn output_2023_22_01() {
        let file_path: String = String::from("inputs/2023/day22.txt");
        let result = solution_2023_22_01(file_path).unwrap();
        assert_eq!(result, 416);
    }

    #[test]
    #[ignore]
    fn outnut_2023_22_02() {
        let file_path: String = String::from("inputs/2023/day22.txt");
        let result = solution_2023_22_02(file_path).unwrap();
        assert_eq!(result, 60963);
    }
}
