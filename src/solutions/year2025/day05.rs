// Advent of Code 2025 - Day 05

use crate::utils::range::Range;

fn parse(inp: &str) -> (Vec<Range<usize>>, Vec<usize>) {
    let mut sections = inp.splitn(2, "\n\n");
    let mut ranges_section = sections
        .next()
        .unwrap()
        .lines()
        .map(|s| Range::from_string(s.trim(), '-').unwrap())
        .collect::<Vec<Range<usize>>>();
    ranges_section.sort();
    let ids_section = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect::<Vec<usize>>();
    (ranges_section, ids_section)
}

pub fn solution_2025_05_01(file_path: String) -> anyhow::Result<usize> {
    let (ranges, ids) = parse(&std::fs::read_to_string(file_path)?);
    let result = ids
        .iter()
        .filter(|&&id| {
            for range in ranges.iter() {
                if range.contains(id) {
                    return true;
                }
            }
            false
        })
        .count();
    Ok(result)
}

pub fn solution_2025_05_02(file_path: String) -> anyhow::Result<usize> {
    let (ranges, _) = parse(&std::fs::read_to_string(file_path)?);
    let compact_range = Range::compact(ranges);
    Ok(compact_range.iter().map(|r| r.length_inclusive()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_05_01() {
        let file_path: String = String::from("inputs/2025/day05e.txt");
        let result = solution_2025_05_01(file_path).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2025_05_02() {
        let file_path: String = String::from("inputs/2025/day05e.txt");
        let result = solution_2025_05_02(file_path).unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    #[ignore]
    fn output_2025_05_01() {
        let file_path: String = String::from("inputs/2025/day05.txt");
        let result = solution_2025_05_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_2025_05_n2() {
        let file_path: String = String::from("inputs/2025/day05.txt");
        let result = solution_2025_05_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
