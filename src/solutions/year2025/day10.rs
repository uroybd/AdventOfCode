// Advent of Code 2025 - Day 10

use num_rational::Rational32;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct Instruction {
    requirement: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_requirement: Vec<usize>,
}

impl Instruction {
    fn from_str(inp: &str) -> Self {
        let parts: Vec<&str> = inp.split(" ").collect();
        let requirement: Vec<bool> = parts[0]
            .strip_prefix("[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("Invalid character in requirement"),
            })
            .collect();
        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|s| {
                s.strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let joltage_requirement: Vec<usize> = parts[parts.len() - 1]
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();
        Self {
            requirement,
            buttons,
            joltage_requirement,
        }
    }
}

struct Machine {
    state: Vec<bool>,
    instruction: Instruction,
}

impl Machine {
    fn new(instruction: Instruction) -> Self {
        let state = vec![false; instruction.requirement.len()];
        Self { state, instruction }
    }

    fn find_least_joltage_presses(&mut self) -> Option<usize> {
        let mut buttons = self.instruction.buttons.clone();
        let joltages: Vec<Rational32> = self
            .instruction
            .joltage_requirement
            .iter()
            .map(|&j| Rational32::from_integer(j as i32))
            .collect();

        let mut rhs = joltages.clone();
        let rows = rhs.len();
        let columns = buttons.len();
        // A 2D grid but built as a flat Vec for easier indexing
        let mut grid: Vec<Rational32> = (0..rows)
            .flat_map(|r| {
                buttons.iter().map(move |button| {
                    if button.contains(&r) {
                        Rational32::from_integer(1)
                    } else {
                        0.into()
                    }
                })
            })
            .collect();

        // Gaussian elimination with partial pivoting
        for i in 0..rows.min(columns) {
            let pivot = (i..columns)
                .flat_map(|column| (i..rows).map(move |row| (row, column)))
                .find(|(row, column)| grid[row * columns + column] != 0.into());
            if pivot.is_none() {
                break;
            }
            let (row, column) = pivot.unwrap();
            if row != i {
                for c in 0..columns {
                    grid.swap(i * columns + c, row * columns + c);
                }
                rhs.swap(i, row);
            }

            if column != i {
                for r in 0..rows {
                    grid.swap(r * columns + i, r * columns + column);
                }
                buttons.swap(i, column);
            }

            if grid[i * columns + i] != 1.into() {
                let denom = grid[i * columns + i];
                for c in i..columns {
                    grid[i * columns + c] /= denom;
                }
                rhs[i] /= denom;
            }

            for r in 0..rows {
                if r != i && grid[r * columns + i] != 0.into() {
                    let factor = grid[r * columns + i] / grid[i * columns + i];
                    for c in i..columns {
                        let subtrahend = factor * grid[i * columns + c];
                        grid[r * columns + c] -= subtrahend;
                    }
                    let subtrahend = factor * rhs[i];
                    rhs[r] -= subtrahend;
                }
            }
        }

        let num_nonzero_rows = (0..rows)
            .rev()
            .find(|&r| (0..columns).any(|c| grid[r * columns + c] != 0.into()))
            .unwrap_or(0)
            + 1;

        rhs.truncate(num_nonzero_rows);
        grid.truncate(num_nonzero_rows * columns);
        let rows = rhs.len();

        let max_presses_per_trailing_button: Vec<u32> = (rows..columns)
            .map(|c| {
                joltages
                    .iter()
                    .enumerate()
                    .filter_map(|(n, &joltage)| {
                        if buttons[c].contains(&n) {
                            Some(joltage.to_integer() as u32)
                        } else {
                            None
                        }
                    })
                    .min()
                    .unwrap()
            })
            .collect();

        let press_difference_per_press: Vec<Rational32> = (rows..columns)
            .map(|c| {
                Rational32::from_integer(1)
                    - (0..rows).map(|r| grid[r * columns + c]).sum::<Rational32>()
            })
            .collect();

        let mut factors = Vec::new();
        let mut factor = 1u32;
        for &max in &max_presses_per_trailing_button {
            factors.push(factor);
            factor *= max + 1;
        }

        let start = press_difference_per_press
            .iter()
            .zip(&max_presses_per_trailing_button)
            .map(|(&diff, &max)| if diff < 0.into() { max } else { 0 })
            .enumerate()
            .fold(0, |acc, (i, presses)| acc + presses * factors[i]);

        let mut to_check = BinaryHeap::new();
        let mut checked_states = HashSet::new();
        to_check.push((Reverse(Rational32::from_integer(0)), start));

        while let Some((Reverse(added_presses), at)) = to_check.pop() {
            let trailing_presses: Vec<u32> = factors
                .iter()
                .zip(&max_presses_per_trailing_button)
                .map(|(f, max)| at / f % (max + 1))
                .collect();

            let presses: Vec<Rational32> = (0..rows)
                .map(|r| {
                    rhs[r]
                        - (0..(columns - rows))
                            .map(|i| {
                                grid[r * columns + (rows + i)]
                                    * Rational32::from_integer(trailing_presses[i] as i32)
                            })
                            .sum::<Rational32>()
                })
                .collect();

            if presses.iter().all(|&p| p >= 0.into() && p.is_integer()) {
                let tot_presses = presses.iter().map(|&p| p.to_integer() as u32).sum::<u32>()
                    + trailing_presses.iter().sum::<u32>();
                return Some(tot_presses as usize);
            }

            for (((&presses_val, &diff), &factor), &max) in trailing_presses
                .iter()
                .zip(&press_difference_per_press)
                .zip(&factors)
                .zip(&max_presses_per_trailing_button)
            {
                if let Some((new_presses, new_added_presses)) =
                    if diff < 0.into() && presses_val > 0 {
                        Some((presses_val - 1, added_presses - diff))
                    } else if presses_val < max {
                        Some((presses_val + 1, added_presses + diff))
                    } else {
                        None
                    }
                {
                    let new_at = at - presses_val * factor + new_presses * factor;
                    if checked_states.insert(new_at) {
                        to_check.push((Reverse(new_added_presses), new_at));
                    }
                }
            }
        }

        None
    }

    fn press_button(&mut self, button_index: usize) {
        for &pos in &self.instruction.buttons[button_index] {
            self.state[pos] = !self.state[pos];
        }
    }

    fn find_least_presses(&mut self) -> Option<usize> {
        let mut min_presses: Option<usize> = None;
        let total_buttons = self.instruction.buttons.len();
        for presses in 0..(1 << total_buttons) {
            self.state = vec![false; self.instruction.requirement.len()];
            let mut press_count = 0;
            for button_index in 0..total_buttons {
                if (presses & (1 << button_index)) != 0 {
                    self.press_button(button_index);
                    press_count += 1;
                }
            }
            if self.state == self.instruction.requirement {
                match min_presses {
                    Some(min) => {
                        if press_count < min {
                            min_presses = Some(press_count);
                        }
                    }
                    None => {
                        min_presses = Some(press_count);
                    }
                }
            }
        }
        min_presses
    }
}

pub fn solution_2025_10_01(file_path: String) -> anyhow::Result<usize> {
    let machines = std::fs::read_to_string(file_path)?
        .lines()
        .map(Instruction::from_str)
        .map(Machine::new)
        .collect::<Vec<Machine>>();
    Ok(machines
        .into_iter()
        .map(|mut machine| machine.find_least_presses().unwrap())
        .sum())
}

pub fn solution_2025_10_02(file_path: String) -> anyhow::Result<usize> {
    let machines = std::fs::read_to_string(file_path)?
        .lines()
        .map(Instruction::from_str)
        .map(Machine::new)
        .collect::<Vec<Machine>>();
    Ok(machines
        .into_iter()
        .map(|mut machine| machine.find_least_joltage_presses().unwrap())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_10_01() {
        let file_path: String = String::from("inputs/2025/day10e.txt");
        let result = solution_2025_10_01(file_path).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_2025_10_02() {
        let file_path: String = String::from("inputs/2025/day10e.txt");
        let result = solution_2025_10_02(file_path).unwrap();
        assert_eq!(result, 33);
    }

    #[test]
    #[ignore]
    fn output_2025_10_01() {
        let file_path: String = String::from("inputs/2025/day10.txt");
        let result = solution_2025_10_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_2025_10_02() {
        let file_path: String = String::from("inputs/2025/day10.txt");
        let result = solution_2025_10_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
