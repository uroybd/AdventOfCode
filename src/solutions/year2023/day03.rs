use std::fs;

// Advent of Code 2023 - Day 03
#[derive(Debug)]
struct PartIndex {
    num: usize,
    y: usize,
    x_start: usize,
    x_end: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Schema {
    parts: Vec<PartIndex>,
    symbols: Vec<Symbol>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSchemaError;

impl std::str::FromStr for Schema {
    type Err = ParseSchemaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed = Self {
            parts: vec![],
            symbols: vec![],
        };
        let width = s.split_once('\n').unwrap().0.len();
        let mut part_nums = vec![];
        for (y, row_str) in s.lines().enumerate() {
            for (x, v) in row_str.chars().enumerate() {
                if v.is_ascii_digit() {
                    part_nums.push(v);
                } else {
                    if !part_nums.is_empty() {
                        parsed.parts.push(PartIndex {
                            num: part_nums
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap(),
                            y,
                            x_start: x - part_nums.len(),
                            x_end: x - 1,
                        });
                        part_nums = vec![];
                    }
                    if v != '.' {
                        parsed.symbols.push(Symbol { symbol: v, x, y })
                    }
                }
            }
            if !part_nums.is_empty() {
                parsed.parts.push(PartIndex {
                    num: part_nums
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap(),
                    y,
                    x_start: width - part_nums.len() - 1,
                    x_end: width - 1,
                });
                part_nums = vec![];
            }
        }

        Ok(parsed)
    }
}

impl Schema {
    pub fn find_adjacent(&self, sym: &Symbol) -> Vec<&PartIndex> {
        let min_row_limit = if sym.y == 0 { 0 } else { sym.y - 1 };
        let min_col_limit = if sym.x == 0 { 0 } else { sym.x - 1 };
        self.parts
            .iter()
            .filter(|p| {
                if p.y > sym.y + 1 || p.y < min_row_limit {
                    return false;
                }
                for x in p.x_start..=p.x_end {
                    if x <= sym.x + 1 && x >= min_col_limit {
                        return true;
                    }
                }
                false
            })
            .collect()
    }

    pub fn find_all_valid_parts(&self) -> impl Iterator<Item = &PartIndex> {
        self.symbols.iter().flat_map(|sym| self.find_adjacent(sym))
    }

    pub fn get_gear_ratio(&self, sym: &Symbol) -> Option<usize> {
        if sym.symbol != '*' {
            return None;
        }
        let adj = self.find_adjacent(sym);
        if adj.len() != 2 {
            return None;
        }
        Some(adj.iter().fold(1, |acc, a| acc * a.num))
    }
}

pub fn solution_day_03_01(file_path: String) -> Option<usize> {
    let schema: Schema = fs::read_to_string(file_path)
        .expect("Invalid File.")
        .parse()
        .unwrap();
    let res = schema.find_all_valid_parts().map(|p| p.num).sum();
    Some(res)
}

pub fn solution_day_03_02(file_path: String) -> Option<usize> {
    let schema: Schema = fs::read_to_string(file_path)
        .expect("Invalid File.")
        .parse()
        .unwrap();
    let res = schema
        .symbols
        .iter()
        .filter_map(|s| schema.get_gear_ratio(s))
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_03_01() {
        let file_path: String = String::from("inputs/2023/day03e.txt");
        let result = solution_day_03_01(file_path).unwrap();
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_day_03_02() {
        let file_path: String = String::from("inputs/2023/day03e.txt");
        let result = solution_day_03_02(file_path).unwrap();
        assert_eq!(result, 467835);
    }

    #[test]
    #[ignore]
    fn output_day_03_01() {
        let file_path: String = String::from("inputs/2023/day03.txt");
        let result = solution_day_03_01(file_path).unwrap();
        assert_eq!(result, 532331);
    }

    #[test]
    #[ignore]
    fn output_day_03_02() {
        let file_path: String = String::from("inputs/2023/day03.txt");
        let result = solution_day_03_02(file_path).unwrap();
        assert_eq!(result, 82301120);
    }
}
