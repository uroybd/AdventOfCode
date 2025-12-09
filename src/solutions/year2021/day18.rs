// Advent of Code 2021 - Day 18

use serde::{Deserialize, Serialize};
use std::fs;
use std::{fmt::Display, ops::Add};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum ListItem {
    Number(usize),
    Nested(Box<ListItem>, Box<ListItem>),
}

#[derive(Debug)]
enum ExplodeCarry {
    Consumed,
    Lhs(usize),
    Rhs(usize),
}

impl Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListItem::Number(n) => write!(f, "{}", n),
            ListItem::Nested(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

impl ListItem {
    fn unwrap_one(&self) -> usize {
        if let Self::Number(n) = self {
            return *n;
        }
        panic!("Expected Regular Number");
    }

    fn unwrap_pair(&self) -> (usize, usize) {
        if let Self::Nested(l, r) = self {
            if matches!(**l, Self::Number(_)) && matches!(**r, Self::Number(_)) {
                return (l.unwrap_one(), r.unwrap_one());
            }
        }

        panic!("Expected a pair of regular numbers");
    }

    fn magnitude(&self) -> usize {
        match self {
            ListItem::Number(n) => *n,
            ListItem::Nested(l, r) => {
                if matches!(**l, ListItem::Number(_)) && matches!(**r, ListItem::Number(_)) {
                    let (l, r) = self.unwrap_pair();

                    3 * l + 2 * r
                } else {
                    let l = l.magnitude();
                    let r = r.magnitude();

                    3 * l + 2 * r
                }
            }
        }
    }

    fn explode(
        &mut self,
        stable: &mut bool,
        lvl: usize,
    ) -> (Option<ExplodeCarry>, Option<ExplodeCarry>) {
        if !*stable || matches!(self, Self::Number(_)) {
            return (None, None);
        }

        if lvl == 4 {
            let (l, r) = self.unwrap_pair();

            let l_exp = ExplodeCarry::Lhs(l);
            let r_exp = ExplodeCarry::Rhs(r);

            *self = Self::Number(0);
            *stable = false;

            return (Some(l_exp), Some(r_exp));
        }

        if lvl < 4 {
            if let Self::Nested(l, r) = self {
                if let (Some(l_ec), Some(r_ec)) = l.explode(stable, lvl + 1) {
                    let r_ec = r.carry(&r_ec);

                    return (Some(l_ec), Some(r_ec));
                } else if let (Some(l_exp), Some(r_exp)) = r.explode(stable, lvl + 1) {
                    let l_exp = l.carry(&l_exp);

                    return (Some(l_exp), Some(r_exp));
                }
            }
        }

        (None, None)
    }

    fn carry(&mut self, carried: &ExplodeCarry) -> ExplodeCarry {
        match carried {
            ExplodeCarry::Consumed => ExplodeCarry::Consumed,
            ExplodeCarry::Lhs(c) => match self {
                ListItem::Number(n) => {
                    *n += c;

                    ExplodeCarry::Consumed
                }
                ListItem::Nested(_, r) => r.carry(carried),
            },
            ExplodeCarry::Rhs(c) => match self {
                ListItem::Number(n) => {
                    *n += c;

                    ExplodeCarry::Consumed
                }
                ListItem::Nested(l, _) => l.carry(carried),
            },
        }
    }

    fn split(&mut self, stable: &mut bool) {
        if !*stable {
            return;
        }

        match self {
            ListItem::Number(n) => {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = ((*n as f32) / 2f32).ceil() as usize;

                    *self = ListItem::Nested(
                        Box::new(ListItem::Number(l)),
                        Box::new(ListItem::Number(r)),
                    );
                    *stable = false;
                }
            }
            ListItem::Nested(l, r) => {
                l.split(stable);
                r.split(stable);
            }
        }
    }
}

impl Add for ListItem {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_pair = Self::Nested(Box::new(self), Box::new(rhs));

        loop {
            let mut stable = true;

            new_pair.explode(&mut stable, 0);

            if !stable {
                continue;
            }

            new_pair.split(&mut stable);

            if stable {
                break;
            }
        }

        new_pair
    }
}

pub fn solution_2021_18_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut numbers: Vec<ListItem> = fs::read_to_string(filepath)?
        .lines()
        .filter_map(|x| serde_json::from_str::<ListItem>(x).ok())
        .collect();

    let initial = numbers.remove(0);
    let res = numbers.into_iter().fold(initial, |acc, cur| acc + cur);
    Ok(res.magnitude() as i64)
}

pub fn solution_2021_18_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let numbers: Vec<ListItem> = fs::read_to_string(filepath)?
        .lines()
        .filter_map(|x| serde_json::from_str::<ListItem>(x).ok())
        .collect();

    let mut results = Vec::with_capacity(numbers.len() * numbers.len());

    for a in 0..numbers.len() {
        for b in 0..numbers.len() {
            if a == b {
                continue;
            }

            results.push((numbers[a].clone() + numbers[b].clone()).magnitude());
        }
    }

    Ok(*results.iter().max().unwrap() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_18_01() {
        let result = solution_2021_18_01("inputs/2021/day18e.txt".to_string()).unwrap();
        assert_eq!(result, 4140);
    }

    #[test]
    #[ignore]
    fn output_2021_18_01() {
        let result = solution_2021_18_01("inputs/2021/day18.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_18_02() {
        let result = solution_2021_18_02("inputs/2021/day18e.txt".to_string()).unwrap();
        assert_eq!(result, 3993);
    }

    #[test]
    #[ignore]
    fn output_2021_18_02() {
        let result = solution_2021_18_02("inputs/2021/day18.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
