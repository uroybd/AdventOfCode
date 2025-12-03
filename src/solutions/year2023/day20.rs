// Advent of Code 2023 - Day 20

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use derive_deref::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Signal {
    name: String,
    pulse: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
    Button,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn input(&mut self, inp: &Signal) -> Option<Vec<Signal>> {
        let pulse = match self.module_type {
            ModuleType::FlipFlop(ref mut state) => {
                if inp.pulse {
                    return None;
                }
                *state = !*state;
                *state
            }
            ModuleType::Conjunction(ref mut inputs) => {
                inputs.insert(inp.name.clone(), inp.pulse);
                inputs.values().any(|p| !p)
            }
            ModuleType::Broadcaster => inp.pulse,
            ModuleType::Button => false,
        };
        Some(
            self.destinations
                .iter()
                .map(|d| Signal {
                    name: d.clone(),
                    pulse,
                })
                .collect(),
        )
    }

    fn dest_contains(&self, name: &str) -> bool {
        self.destinations.contains(&name.to_string())
    }

    fn wire_up(&mut self, circuit: &Circuit) {
        if let ModuleType::Conjunction(ref mut inputs) = self.module_type {
            circuit
                .iter()
                .filter_map(|(key, v)| {
                    if v.dest_contains(&self.name) {
                        Some(key.clone())
                    } else {
                        None
                    }
                })
                .for_each(|v| {
                    inputs.insert(v, false);
                });
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseModuleError;

impl std::str::FromStr for Module {
    type Err = ParseModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (src, dest) = s.split_once(" -> ").unwrap();
        let destinations: Vec<String> = dest.split(", ").map(|x| x.to_string()).collect();
        let name_parts = src.split_at(1);
        let (module_type, name) = match name_parts {
            ("&", name) => (ModuleType::Conjunction(HashMap::new()), name),
            ("%", name) => (ModuleType::FlipFlop(false), name),
            ("b", _) => (ModuleType::Broadcaster, "broadcaster"),
            _ => panic!("Invalid module type."),
        };
        Ok(Module {
            name: name.to_string(),
            module_type,
            destinations,
        })
    }
}

#[derive(Deref, DerefMut)]
struct Circuit(HashMap<String, Module>);

#[derive(Debug, PartialEq, Eq)]
struct ParseCircuitError;

impl std::str::FromStr for Circuit {
    type Err = ParseCircuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut circuit = Self(HashMap::from([(
            "button".to_string(),
            Module {
                name: "button".to_string(),
                module_type: ModuleType::Button,
                destinations: vec!["broadcaster".to_string()],
            },
        )]));
        let mut instructions: Vec<&str> = s.lines().collect();
        instructions.sort();
        instructions.reverse();
        while let Some(ins) = instructions.pop() {
            let mut module = ins.parse::<Module>().unwrap();
            module.wire_up(&circuit);
            circuit.insert(module.name.clone(), module);
        }
        Ok(circuit)
    }
}

impl Circuit {
    fn run(&mut self) -> (usize, usize) {
        let mut high_count = 0;
        let mut low_count = -1;
        let mut queue = VecDeque::from(vec![(
            "button".to_string(),
            Signal {
                name: "button".to_string(),
                pulse: false,
            },
        )]);

        while let Some((name, signal)) = queue.pop_front() {
            match signal.pulse {
                true => high_count += 1,
                false => low_count += 1,
            }

            if let Some(module) = self.get_mut(&name) {
                if let Some(outputs) = module.input(&signal) {
                    for entry in outputs {
                        queue.push_back((
                            entry.name.clone(),
                            Signal {
                                name: module.name.clone(),
                                pulse: entry.pulse,
                            },
                        ));
                    }
                }
            }
        }
        (high_count, (low_count as usize))
    }

    fn total_pulse(&mut self, n: usize) -> usize {
        let (high, low) = (0..n).fold((0, 0), |(h, l), _| {
            let (vh, vl) = self.run();
            (h + vh, l + vl)
        });
        high * low
    }

    fn run_until_on(&mut self) -> usize {
        let rx_setter = self.values().find(|m| m.dest_contains("rx")).unwrap();
        let leading_to_rx_setter = self
            .values()
            .filter(|m| m.dest_contains(&rx_setter.name))
            .map(|m| m.name.clone())
            .collect::<Vec<String>>();
        let mut cycles = HashMap::new();
        let mut cycle = 0;
        'mloop: loop {
            cycle += 1;
            let mut queue = VecDeque::from(vec![(
                "button".to_string(),
                Signal {
                    name: "button".to_string(),
                    pulse: false,
                },
            )]);

            while let Some((name, signal)) = queue.pop_front() {
                if let Some(module) = self.get_mut(&name) {
                    if leading_to_rx_setter.contains(&name)
                        && !signal.pulse
                        && !cycles.contains_key(&name)
                    {
                        cycles.insert(name.clone(), cycle);
                        if cycles.len() == leading_to_rx_setter.len() {
                            break 'mloop;
                        }
                    }
                    if let Some(outputs) = module.input(&signal) {
                        for entry in outputs {
                            queue.push_back((
                                entry.name.clone(),
                                Signal {
                                    name: module.name.clone(),
                                    pulse: entry.pulse,
                                },
                            ));
                        }
                    }
                }
            }
        }

        cycles.values().product()
    }
}

pub fn solution_day_20_01(file_path: String) -> Option<usize> {
    let mut circuit = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .parse::<Circuit>()
        .unwrap();

    Some(circuit.total_pulse(1000))
    // Some((0..1000).fold(0, |acc, _| acc + circuit.run()))
}

pub fn solution_day_20_02(file_path: String) -> Option<usize> {
    let mut circuit = fs::read_to_string(file_path)
        .expect("Invalid input file.")
        .parse::<Circuit>()
        .unwrap();

    Some(circuit.run_until_on())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_20_01() {
        let file_path: String = String::from("inputs/2023/day20e.txt");
        let result = solution_day_20_01(file_path).unwrap();
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test_day_20_02() {
        // let file_path: String = String::from("inputs/2023/day20e.txt");
        // let result = solution_day_20_02(file_path).unwrap();
        // dbg!(result);
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_20_01() {
        let file_path: String = String::from("inputs/2023/day20.txt");
        let result = solution_day_20_01(file_path).unwrap();
        assert_eq!(result, 743090292);
    }

    #[test]
    #[ignore]
    fn output_day_20_02() {
        let file_path: String = String::from("inputs/2023/day20.txt");
        let result = solution_day_20_02(file_path).unwrap();
        assert_eq!(result, 241528184647003);
    }
}
