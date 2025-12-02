use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn from_str(s: &str) -> Option<Coordinate> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        let x = parts[0].parse::<usize>().ok()?;
        let y = parts[1].parse::<usize>().ok()?;
        Some(Coordinate(x, y))
    }
}

struct Range(Coordinate, Coordinate);
impl Range {
    fn from_str(s: &str) -> Option<Range> {
        let parts: Vec<&str> = s.split(" through ").collect();
        if parts.len() != 2 {
            return None;
        }
        let start = Coordinate::from_str(parts[0])?;
        let end = Coordinate::from_str(parts[1])?;
        Some(Range(start, end))
    }

    fn iter(&self) -> impl Iterator<Item = Coordinate> {
        let Coordinate(start_x, start_y) = self.0;
        let Coordinate(end_x, end_y) = self.1;
        (start_x..=end_x).flat_map(move |x| (start_y..=end_y).map(move |y| Coordinate(x, y)))
    }
}

struct Instruction(String, Range);
impl Instruction {
    fn from_str(s: &str) -> Option<Instruction> {
        match s {
            s if s.starts_with("turn on ") => {
                let range_str = &s[8..];
                let range = Range::from_str(range_str)?;
                Some(Instruction("turn on".to_string(), range))
            }
            s if s.starts_with("turn off ") => {
                let range_str = &s[9..];
                let range = Range::from_str(range_str)?;
                Some(Instruction("turn off".to_string(), range))
            }
            s if s.starts_with("toggle ") => {
                let range_str = &s[7..];
                let range = Range::from_str(range_str)?;
                Some(Instruction("toggle".to_string(), range))
            }
            _ => None,
        }
    }

    fn apply(&self, grid: &mut HashMap<Coordinate, bool>) {
        let Instruction(action, range) = self;
        for Coordinate(x, y) in range.iter() {
            match action.as_str() {
                "turn on" => {
                    grid.insert(Coordinate(x, y), true);
                }
                "turn off" => {
                    grid.insert(Coordinate(x, y), false);
                }
                "toggle" => {
                    let current = grid.get(&Coordinate(x, y)).cloned().unwrap_or(false);
                    grid.insert(Coordinate(x, y), !current);
                }
                _ => {}
            };
        }
    }

    fn apply_brightness(&self, grid: &mut HashMap<Coordinate, usize>) {
        let Instruction(action, range) = self;
        for Coordinate(x, y) in range.iter() {
            let current_brightness = grid.get(&Coordinate(x, y)).cloned().unwrap_or(0);
            match action.as_str() {
                "turn on" => {
                    grid.insert(Coordinate(x, y), current_brightness + 1);
                }
                "turn off" => {
                    grid.insert(Coordinate(x, y), current_brightness.saturating_sub(1));
                }
                "toggle" => {
                    grid.insert(Coordinate(x, y), current_brightness + 2);
                }
                _ => {}
            };
        }
    }
}

pub fn solution_2015_06_01(filepath: String) -> Result<usize, Box<dyn std::error::Error>> {
    let mut grid = HashMap::new();
    std::fs::read_to_string(filepath)?
        .trim_end()
        .lines()
        .filter_map(Instruction::from_str)
        .for_each(|instruction| {
            instruction.apply(&mut grid);
        });
    let count = grid.values().filter(|&&v| v).count();
    Ok(count)
}

pub fn solution_2015_06_02(filepath: String) -> Result<usize, Box<dyn std::error::Error>> {
    let mut grid = HashMap::new();
    std::fs::read_to_string(filepath)?
        .trim_end()
        .lines()
        .filter_map(Instruction::from_str)
        .for_each(|instruction| {
            instruction.apply_brightness(&mut grid);
        });
    let count = grid.values().sum();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_2015_06_01() {
        let file_path = String::from("inputs/2015/day06.txt");
        let result = solution_2015_06_01(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 569999);
    }

    #[test]
    fn test_solution_2015_06_02() {
        let file_path = String::from("inputs/2015/day06.txt");
        let result = solution_2015_06_02(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 17836115);
    }
}
