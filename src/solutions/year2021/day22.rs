// Advent of Code 2021 - Day 22

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

type Cube = [[isize; 2]; 3];
type Point = [isize; 3];

trait Intersection {
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl Intersection for Cube {
    fn intersection(&self, other: &Self) -> Option<Self> {
        let mut new_cube = [[0; 2]; 3];
        for axis in 0..3 {
            let (start, end) = (
                max(self[axis][0], other[axis][0]),
                min(self[axis][1], other[axis][1]),
            );
            if start > end {
                return None;
            }
            new_cube[axis] = [start, end];
        }
        Some(new_cube)
    }
}

struct Instruction {
    cube: Cube,
    value: bool,
}

impl Instruction {
    fn from_string(line: String) -> Self {
        let mut splitted = line.splitn(2, " ");
        let value = match splitted.next().unwrap() {
            "on" => true,
            _ => false,
        };
        let all_axis: Vec<[isize; 2]> = splitted
            .next()
            .unwrap()
            .split(",")
            .map(|axis| {
                let mut splitted = axis.splitn(2, "=");
                let numbers: Vec<isize> = splitted
                    .nth(1)
                    .unwrap()
                    .split("..")
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                [numbers[0], numbers[1]]
            })
            .collect();
        Self {
            cube: [all_axis[0], all_axis[1], all_axis[2]],
            value,
        }
    }
}

pub fn solution_2021_22_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = fs::read_to_string(filepath)?
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut reactor: HashMap<Point, bool> = (-50..51)
        .flat_map(|x| (-50..51).flat_map(move |y| (-50..51).map(move |z| ([x, y, z], false))))
        .collect();
    
    for ins in instructions {
        let instruction = Instruction::from_string(ins);
        for x in isize::max(instruction.cube[0][0], -50)..isize::min(instruction.cube[0][1] + 1, 51) {
            for y in isize::max(instruction.cube[1][0], -50)..isize::min(instruction.cube[1][1] + 1, 51) {
                for z in isize::max(instruction.cube[2][0], -50)..isize::min(instruction.cube[2][1] + 1, 51) {
                    reactor.insert([x, y, z], instruction.value);
                }
            }
        }
    }
    Ok(reactor.values().filter(|v| v == &&true).count() as i64)
}

pub fn solution_2021_22_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: Vec<String> = fs::read_to_string(filepath)?
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut reactor: HashMap<Cube, isize> = HashMap::new();
    
    for ins in instructions {
        let instruction = Instruction::from_string(ins);
        let mut new_cubes = reactor.clone();
        for (cube, weight) in reactor.iter() {
            if let Some(intersection) = cube.intersection(&instruction.cube) {
                *new_cubes.entry(intersection).or_insert(0) -= weight;
            }
        }
        if instruction.value {
            *new_cubes.entry(instruction.cube).or_insert(0) += 1;
        }
        reactor = new_cubes;
    }
    
    let count = reactor.iter().fold(0, |acc, (cube, weight)| {
        acc + match weight {
            0 => 0,
            _ => {
                cube.into_iter()
                    .fold(1, |acc2, axis| acc2 * (axis[1] - axis[0] + 1))
                    * weight
            }
        }
    });
    Ok(count as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2021_22_01() {
        let result = solution_2021_22_01("inputs/2021/day22e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_22_01() {
        let result = solution_2021_22_01("inputs/2021/day22.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_22_02() {
        let result = solution_2021_22_02("inputs/2021/day22e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_22_02() {
        let result = solution_2021_22_02("inputs/2021/day22.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
