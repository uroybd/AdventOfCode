// Advent of Code 2021 - Day 15

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(PartialEq, Eq, Ord, PartialOrd)]
struct Entry {
    value: usize,
    pos: (usize, usize),
}

fn traverse(cavern: &Vec<Vec<usize>>) -> usize {
    let mut prev: HashMap<(usize, usize), usize> = HashMap::new();
    prev.entry((0, 0)).or_insert(0);
    let mut walk = BinaryHeap::new();
    walk.push(Reverse(Entry {
        pos: (0, 0),
        value: 0,
    }));
    let last = (cavern[0].len() - 1, cavern.len() - 1);
    loop {
        let min = walk.pop().unwrap().0;
        if min.pos == last {
            break;
        }
        for neighbor in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (x, y) = (
                (min.pos.0 as i32 + neighbor.0) as usize,
                (min.pos.1 as i32 + neighbor.1) as usize,
            );
            if prev.get(&(x, y)).is_some() {
                continue;
            }
            let value = match cavern.get(y).and_then(|p| p.get(x)) {
                Some(v) => *v as usize,
                None => continue,
            };
            let entry = Entry {
                pos: (x, y),
                value: min.value + value,
            };
            walk.push(Reverse(entry));
            prev.entry((x, y)).or_insert(min.value + value);
        }
    }
    prev[&last]
}

fn add_risk(cavern: &Vec<Vec<usize>>, amount: usize) -> Vec<Vec<usize>> {
    if amount == 0 {
        return cavern.clone();
    }
    let res: Vec<Vec<usize>> = cavern
        .iter()
        .map(|line| {
            line.iter()
                .map(|&v| {
                    let v = (v + amount) % 10;
                    if v == 0 {
                        return 1;
                    } else {
                        return v;
                    }
                })
                .collect()
        })
        .collect();
    res
}

fn merge(cavern: &mut Vec<Vec<usize>>, to_merge: &Vec<Vec<usize>>) {
    for (idx, row) in to_merge.iter().enumerate() {
        cavern[idx].extend(row);
    }
}

fn create_full_map(cavern: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut maps: Vec<Vec<Vec<usize>>> = vec![cavern.clone()];
    let mut last = cavern.clone();
    for _ in 1..9 {
        let new = add_risk(&last, 1);
        maps.push(new.clone());
        last = new;
    }
    let mut res = vec![];
    for i in 0..5 {
        let mut first = maps[i + 0].clone();
        for j in 1..5 {
            merge(&mut first, &maps[i + j])
        }
        res.extend(first);
    }
    res
}

pub fn solution_2021_15_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<Vec<usize>> = fs::read_to_string(filepath)?
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    Ok(traverse(&data) as i64)
}

pub fn solution_2021_15_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<Vec<usize>> = fs::read_to_string(filepath)?
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let map = create_full_map(&data);
    Ok(traverse(&map) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_15_01() {
        let result = solution_2021_15_01("inputs/2021/day15e.txt".to_string()).unwrap();
        assert_eq!(result, 40);
    }

    #[test]
    #[ignore]
    fn output_2021_15_01() {
        let result = solution_2021_15_01("inputs/2021/day15.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_15_02() {
        let result = solution_2021_15_02("inputs/2021/day15e.txt".to_string()).unwrap();
        assert_eq!(result, 315);
    }

    #[test]
    #[ignore]
    fn output_2021_15_02() {
        let result = solution_2021_15_02("inputs/2021/day15.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
