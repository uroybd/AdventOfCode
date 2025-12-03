// Advent of Code 2023 - Day 19

use std::{collections::HashMap, fs};

use derive_deref::{Deref, DerefMut};

#[derive(Debug, Clone, Deref)]
struct Part(HashMap<char, usize>);

#[derive(Debug, Eq, PartialEq)]
struct ParsePartError;

impl std::str::FromStr for Part {
    type Err = ParsePartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        s.trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .for_each(|s| {
                let (c, n) = s.split_once('=').unwrap();
                map.insert(c.chars().next().unwrap(), n.parse::<usize>().unwrap());
            });
        Ok(Self(map))
    }
}

impl Part {
    fn get_total(&self) -> usize {
        self.values().sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut)]
struct PartRange(HashMap<char, (usize, usize)>);

impl PartRange {
    fn new_with_range(start: usize, end: usize) -> Self {
        Self(HashMap::from([
            ('x', (start, end)),
            ('m', (start, end)),
            ('a', (start, end)),
            ('s', (start, end)),
        ]))
    }

    fn size(&self) -> usize {
        self.values().fold(1, |acc, (s, e)| acc * (e - s + 1))
    }
}

#[derive(Debug, Clone)]
struct Condition {
    key: Option<char>,
    operator: Option<char>,
    value: Option<usize>,
    action: String,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseConditionError;

impl std::str::FromStr for Condition {
    type Err = ParseConditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((f, action)) = s.split_once(':') {
            let mut f_it = f.chars();
            let key = f_it.next().unwrap();
            let operator = f_it.next().unwrap();
            let value = f_it.as_str().parse::<usize>().unwrap();
            Ok(Self {
                key: Some(key),
                operator: Some(operator),
                value: Some(value),
                action: action.to_string(),
            })
        } else {
            Ok(Self {
                key: None,
                operator: None,
                value: None,
                action: s.to_string(),
            })
        }
    }
}

impl Condition {
    fn run(&self, input: &Part) -> Option<String> {
        if let Some(key) = self.key {
            let op = self.operator.unwrap();
            let value = self.value.unwrap();
            let part_value = input.get(&key).unwrap_or(&0);
            match op {
                '>' => {
                    if part_value > &value {
                        Some(self.action.clone())
                    } else {
                        None
                    }
                }
                '<' => {
                    if part_value < &value {
                        Some(self.action.clone())
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            }
        } else {
            Some(self.action.clone())
        }
    }

    fn run_part_range(&self, input: &PartRange) -> Result<Option<String>, Vec<PartRange>> {
        if let Some(key) = self.key {
            let op = self.operator.unwrap();
            let value = self.value.unwrap();
            let (start, end) = input.get(&key).unwrap_or(&(0, 0));
            match op {
                '>' => {
                    if start <= &value && &value < end {
                        let mut a = input.clone();
                        a.insert(key, (*start, value));

                        let mut b = input.clone();
                        b.insert(key, (value + 1, *end));

                        return Err(vec![a, b]);
                    }
                    if start > &value {
                        Ok(Some(self.action.clone()))
                    } else {
                        Ok(None)
                    }
                }
                '<' => {
                    if start < &value && &value <= end {
                        let mut a = input.clone();
                        a.insert(key, (*start, value - 1));

                        let mut b = input.clone();
                        b.insert(key, (value, *end));
                        return Err(vec![a, b]);
                    }
                    if start < end && end < &value {
                        Ok(Some(self.action.clone()))
                    } else {
                        Ok(None)
                    }
                }

                _ => unreachable!(),
            }
        } else {
            Ok(Some(self.action.clone()))
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseWorkflowError;

impl std::str::FromStr for Workflow {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, conditions_str) = s.split_once('{').unwrap();

        let conditions = conditions_str
            .trim_end_matches('}')
            .split(',')
            .map(|s| s.parse::<Condition>().unwrap())
            .collect();
        Ok(Self {
            name: name.to_string(),
            conditions,
        })
    }
}

#[derive(Debug, Clone)]
struct WorkflowManager {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseWorkflowManagerError;

impl std::str::FromStr for WorkflowManager {
    type Err = ParseWorkflowManagerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (wf_str, parts_str) = s.split_once("\n\n").unwrap();
        let mut workflows = HashMap::new();
        wf_str.lines().for_each(|l| {
            let workflow = l.parse::<Workflow>().unwrap();
            workflows.insert(workflow.name.clone(), workflow);
        });
        let parts = parts_str
            .lines()
            .map(|l| l.parse::<Part>().unwrap())
            .collect();

        Ok(Self { workflows, parts })
    }
}

impl WorkflowManager {
    fn accepted(&self, part: &Part) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();
        loop {
            for condition in &workflow.conditions {
                if let Some(action) = condition.run(part) {
                    match action.as_str() {
                        "A" => {
                            return true;
                        }
                        "R" => {
                            return false;
                        }
                        _ => {
                            workflow = self.workflows.get(&action).unwrap();
                            break;
                        }
                    }
                }
            }
        }
    }

    fn get_total_accepted_value(&self) -> usize {
        self.parts
            .iter()
            .filter(|p| self.accepted(p))
            .map(|p| p.get_total())
            .sum()
    }

    fn get_range(&self, part: &PartRange) -> Vec<PartRange> {
        let mut workflow = self.workflows.get("in").unwrap();
        loop {
            for condition in &workflow.conditions {
                match condition.run_part_range(part) {
                    Ok(Some(action)) => match action.as_str() {
                        "A" => {
                            return vec![part.clone()];
                        }
                        "R" => {
                            return vec![];
                        }
                        _ => {
                            workflow = self.workflows.get(&action).unwrap();
                            break;
                        }
                    },
                    Err(parts) => return parts,
                    _ => (),
                }
            }
        }
    }

    fn find_combinations(&self) -> usize {
        let mut available = vec![PartRange::new_with_range(1, 4000)];
        let mut next = vec![];

        loop {
            for part in available.clone() {
                let mut parts = self.get_range(&part);
                next.append(&mut parts);
            }
            if next == available {
                break;
            }
            available = next;
            next = vec![];
        }
        available.iter().fold(0, |acc, p| acc + p.size())
    }
}

pub fn solution_day_19_01(file_path: String) -> Option<usize> {
    let manager = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .parse::<WorkflowManager>()
        .unwrap();
    Some(manager.get_total_accepted_value())
}

pub fn solution_day_19_02(file_path: String) -> Option<usize> {
    let manager = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .parse::<WorkflowManager>()
        .unwrap();
    Some(manager.find_combinations())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_19_01() {
        let file_path: String = String::from("inputs/2023/day19e.txt");
        let result = solution_day_19_01(file_path).unwrap();
        assert_eq!(result, 19114);
    }

    #[test]
    fn test_day_19_02() {
        let file_path: String = String::from("inputs/2023/day19e.txt");
        let result = solution_day_19_02(file_path).unwrap();
        assert_eq!(result, 167409079868000);
    }

    #[test]
    #[ignore]
    fn output_day_19_01() {
        let file_path: String = String::from("inputs/2023/day19.txt");
        let result = solution_day_19_01(file_path).unwrap();
        assert_eq!(result, 342650);
    }

    #[test]
    #[ignore]
    fn output_day_19_02() {
        let file_path: String = String::from("inputs/2023/day19.txt");
        let result = solution_day_19_02(file_path).unwrap();
        assert_eq!(result, 130303473508222);
    }
}
