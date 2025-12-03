use std::fs;

// Advent of Code 2023 - Day 05

// src, length, and optional dest
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct AlmanacRange {
    start: usize,
    length: usize,
    dest: Option<usize>,
}

impl AlmanacRange {
    fn includes(&self, val: usize) -> bool {
        val >= self.start && val < self.start + self.length
    }

    fn end(&self) -> usize {
        self.start + self.length
    }
}

trait AlmanacRangeVec {
    fn merge(&self) -> Vec<AlmanacRange>;
    fn generate_ranges(&self, range: &AlmanacRange) -> Vec<AlmanacRange>;
}

impl AlmanacRangeVec for Vec<AlmanacRange> {
    fn merge(&self) -> Vec<AlmanacRange> {
        let mut merged = vec![];
        let mut current = self[0].clone();
        for r in self {
            if current.includes(r.start) {
                current.length = current.length.max(r.length + r.start - current.start);
            } else if r.start == current.start + current.length {
                current.length += r.length;
            } else {
                merged.push(current.clone());
                current = r.clone();
            }
        }
        merged.push(current);
        merged
    }

    fn generate_ranges(&self, range: &AlmanacRange) -> Vec<AlmanacRange> {
        let mut results = vec![];
        let mut cursor = range.start;
        let mut remaining = range.length;
        let mut index = match self.binary_search_by(|rule| rule.start.cmp(&range.start)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        let hard_start = AlmanacRange {
            start: 0,
            length: 0,
            dest: Some(0),
        };
        let hard_end = AlmanacRange {
            start: usize::MAX,
            length: usize::MAX,
            dest: Some(usize::MAX),
        };
        while remaining > 0 {
            let prev = if index > 0 {
                &self[index - 1]
            } else {
                &hard_start
            };
            let next = if index < self.len() {
                &self[index]
            } else {
                &hard_end
            };
            if prev.includes(cursor) {
                results.push(AlmanacRange {
                    start: prev.dest.unwrap() + cursor - prev.start,
                    length: remaining.min(prev.end() - cursor),
                    dest: None,
                });
                remaining -= remaining.min(prev.end() - cursor);
                cursor = prev.end();
            }
            results.push(AlmanacRange {
                start: cursor,
                length: remaining.min(next.start - cursor),
                dest: None,
            });
            remaining -= remaining.min(next.start - cursor);
            cursor = next.start;
            index += 1;
        }
        results
    }
}

struct Almanac {
    seeds: Vec<usize>,
    rules: Vec<Vec<AlmanacRange>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseAlmanacError;

impl std::str::FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rval = Self {
            seeds: vec![],
            rules: vec![],
        };
        let mut sections = s.split("\n\n");
        let (_, seeds) = sections.next().unwrap().split_once(": ").unwrap();
        for i in seeds.split(' ') {
            let v = i.parse::<usize>().unwrap();
            rval.seeds.push(v);
        }
        rval.rules = sections.map(Almanac::parse_section).collect();
        Ok(rval)
    }
}

impl Almanac {
    fn seeds_as_ranges(&self) -> Vec<AlmanacRange> {
        self.seeds
            .chunks(2)
            .map(|pair| AlmanacRange {
                start: pair[0],
                length: pair[1],
                dest: None,
            })
            .collect()
    }

    fn parse_section(inp: &str) -> Vec<AlmanacRange> {
        let mut res: Vec<AlmanacRange> = inp
            .lines()
            .skip(1)
            .map(|l| {
                let mut parts = l.splitn(3, ' ');
                let dest = parts.next().unwrap().parse::<usize>().unwrap();
                let src = parts.next().unwrap().parse::<usize>().unwrap();
                let count = parts.next().unwrap().parse::<usize>().unwrap();
                AlmanacRange {
                    start: src,
                    length: count,
                    dest: Some(dest),
                }
            })
            .collect();
        res.sort_by(|a, b| a.start.cmp(&b.start));
        res
    }

    fn find_location(&self, init: usize) -> usize {
        let mut current = init;
        for section in self.rules.iter() {
            for rule in section {
                if rule.includes(current) {
                    current = rule.dest.unwrap() + current - rule.start;
                    break;
                }
            }
        }
        current
    }
}

pub fn solution_2023_05_01(file_path: String) -> Option<usize> {
    let almanac: Almanac = fs::read_to_string(file_path)
        .expect("Invalid Input file.")
        .parse()
        .unwrap();

    let res = almanac
        .seeds
        .iter()
        .map(|seed| almanac.find_location(*seed))
        .min();
    res
}

pub fn solution_2023_05_02(file_path: String) -> Option<usize> {
    let almanac: Almanac = fs::read_to_string(file_path)
        .expect("Invalid Input file.")
        .parse()
        .unwrap();
    let seed_ranges = almanac.seeds_as_ranges();
    let res = almanac
        .rules
        .iter()
        .fold(seed_ranges, |ranges, section| {
            let mut vals: Vec<AlmanacRange> = ranges
                .iter()
                .flat_map(|sr| section.generate_ranges(sr))
                .filter(|r| r.length > 0)
                .collect();
            vals.sort_by(|a, b| a.start.cmp(&b.start));
            vals.merge()
        })
        .iter()
        .min()
        .unwrap()
        .start;
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_05_01() {
        let file_path: String = String::from("inputs/2023/day05e.txt");
        let result = solution_2023_05_01(file_path).unwrap();
        assert_eq!(result, 35);
    }

    #[test]
    fn test_2023_05_02() {
        let file_path: String = String::from("inputs/2023/day05e.txt");
        let result = solution_2023_05_02(file_path).unwrap();
        assert_eq!(result, 46);
    }

    #[test]
    #[ignore]
    fn output_day_05_01() {
        let file_path: String = String::from("inputs/2023/day05.txt");
        let result = solution_2023_05_01(file_path).unwrap();
        assert_eq!(result, 240320250);
    }

    #[test]
    #[ignore]
    fn output_day_05_02() {
        let file_path: String = String::from("inputs/2023/day05.txt");
        let result = solution_2023_05_02(file_path).unwrap();
        assert_eq!(result, 28580589);
    }
}
