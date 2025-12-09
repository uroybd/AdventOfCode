// Advent of Code 2021 - Day 11

use std::cmp::{max, min};
use std::fs;

fn get_pos(x: isize, y: isize, size: isize) -> isize {
    (x * size) + y
}

fn get_axis(pos: usize, size: isize) -> (isize, isize) {
    (pos as isize / size, pos as isize % size)
}

fn get_adjacent(x: isize, y: isize, size: isize) -> Vec<isize> {
    let mut values: Vec<isize> = vec![];
    for nx in max(0, x - 1)..min(x + 2, size) {
        for ny in max(0, y - 1)..min(y + 2, size) {
            if !(nx == x && ny == y) {
                values.push(get_pos(nx, ny, size));
            }
        }
    }
    values
}

fn flash(octopuses: &mut Vec<isize>, size: isize) -> usize {
    let mut to_update: Vec<isize> = vec![];
    for (idx, octopus) in octopuses.iter_mut().enumerate() {
        *octopus += 1;
        if octopus > &mut 9 {
            let pos = get_axis(idx, size);
            let adjacent_elements: Vec<isize> = get_adjacent(pos.0, pos.1, size);
            to_update.extend(adjacent_elements);
        }
    }
    to_update = to_update
        .into_iter()
        .filter(|&pos| octopuses[pos as usize] < 10)
        .collect();

    while !to_update.is_empty() {
        let mut new_to_update: Vec<isize> = vec![];
        for value in &to_update {
            let val_u = *value as usize;
            let mut val = octopuses[val_u];

            if val < 10 {
                val += 1;
                if val == 10 {
                    let pos = get_axis(val_u, size);
                    let adjacent_elements: Vec<isize> = get_adjacent(pos.0, pos.1, size)
                        .into_iter()
                        .filter(|&pos| octopuses[pos as usize] < 10)
                        .collect();
                    new_to_update.extend(adjacent_elements);
                }
                octopuses[val_u] = val;
            }
        }
        to_update = new_to_update;
    }
    let mut flashes = 0;
    for octopus in octopuses.iter_mut() {
        if octopus == &10 {
            *octopus = 0;
            flashes += 1;
        }
    }
    flashes
}

pub fn solution_2021_11_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<String> = fs::read_to_string(filepath)?
        .lines()
        .map(|s| s.to_string())
        .collect();
    let size = data.len() as isize;
    let mut octopuses: Vec<isize> = data
        .iter()
        .flat_map(|s| s.chars().map(|c| c.to_string().parse::<isize>().unwrap()))
        .collect();
    let mut res = 0;
    for _ in 0..100 {
        let flashes = flash(&mut octopuses, size);
        res += flashes;
    }
    Ok(res as i64)
}

pub fn solution_2021_11_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<String> = fs::read_to_string(filepath)?
        .lines()
        .map(|s| s.to_string())
        .collect();
    let size = data.len() as isize;
    let mut octopuses: Vec<isize> = data
        .iter()
        .flat_map(|s| s.chars().map(|c| c.to_string().parse::<isize>().unwrap()))
        .collect();
    let len = octopuses.len();
    let mut steps = 0;
    loop {
        steps += 1;
        let flashes = flash(&mut octopuses, size);
        if flashes == len {
            break;
        }
    }
    Ok(steps as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_11_01() {
        let result = solution_2021_11_01("inputs/2021/day11e.txt".to_string()).unwrap();
        assert_eq!(result, 1656);
    }

    #[test]
    #[ignore]
    fn output_2021_11_01() {
        let result = solution_2021_11_01("inputs/2021/day11.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_11_02() {
        let result = solution_2021_11_02("inputs/2021/day11e.txt".to_string()).unwrap();
        assert_eq!(result, 195);
    }

    #[test]
    #[ignore]
    fn output_2021_11_02() {
        let result = solution_2021_11_02("inputs/2021/day11.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
