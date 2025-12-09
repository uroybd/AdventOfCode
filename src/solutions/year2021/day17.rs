// Advent of Code 2021 - Day 17

struct StepVelocity {
    value: usize,
    to_add: usize,
}

impl Iterator for StepVelocity {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += self.to_add;
        self.to_add += 1;
        Some(self.value)
    }
}

impl StepVelocity {
    fn new() -> Self {
        return StepVelocity {
            value: 0,
            to_add: 1,
        };
    }
}

fn get_x_limit(start: usize, end: usize) -> (usize, usize) {
    let mut sv = StepVelocity::new();
    let mut res: Vec<usize> = vec![];
    let (mut first, mut count) = (0, 0);
    while sv.value < end {
        count += 1;
        let v = sv.next().unwrap();
        if v >= start && v <= end {
            res.push(v);
            if first == 0 {
                first = count;
            }
        }
    }
    res.push(sv.next().unwrap());
    (first, *res.last().unwrap())
}

struct Probe {
    position: (isize, isize),
    velocity: (isize, isize),
}

struct Target {
    x: (isize, isize),
    y: (isize, isize),
}

impl Probe {
    fn step(&mut self) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        self.velocity.1 -= 1;
        if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        } else if self.velocity.0 < 0 {
            self.velocity.0 += 1;
        }
    }

    fn new(position: (isize, isize), velocity: (isize, isize)) -> Probe {
        Probe { velocity, position }
    }

    fn on_target(&self, target: &Target) -> bool {
        self.position.0 >= target.x.0
            && self.position.0 <= target.x.1
            && self.position.1 >= target.y.0
            && self.position.1 <= target.y.1
    }

    fn will_be_on_target(&mut self, target: &Target) -> (bool, isize) {
        let (mut highest_y, mut on_target) = (self.position.1, false);
        while self.position.0 <= target.x.1 || self.position.1 >= target.y.1 {
            self.step();
            if self.position.1 > highest_y {
                highest_y = self.position.1
            }
            if self.on_target(target) {
                on_target = true;
                break;
            }
            if self.velocity.0 == 0 && self.position.1 < target.y.1 {
                break;
            }
        }
        return (on_target, highest_y);
    }
}

fn solve(x: (isize, isize), y: (isize, isize)) -> (isize, usize) {
    let target = Target { x, y };
    let mut highest = 0;
    let x_limit = get_x_limit(x.0 as usize, x.1 as usize);
    let mut count = 0;
    for i in x_limit.0..x_limit.1 {
        for j in 0..x_limit.1 {
            if (i, j) != (0, 0) {
                let mut probe = Probe::new((0, 0), (i as isize, j as isize));
                let (on_target, y_highest) = probe.will_be_on_target(&target);
                if on_target {
                    if y_highest > highest {
                        highest = y_highest
                    }
                    count += 1
                }
                if j != 0 {
                    let mut probe = Probe::new((0, 0), (i as isize, -(j as isize)));
                    let (on_target, y_highest) = probe.will_be_on_target(&target);
                    if on_target {
                        if y_highest > highest {
                            highest = y_highest
                        }
                        count += 1
                    }
                }
            }
        }
    }
    (highest, count)
}

pub fn solution_2021_17_01(_filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let (highest, _) = solve((253, 280), (-73, -46));
    Ok(highest as i64)
}

pub fn solution_2021_17_02(_filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let (_, count) = solve((253, 280), (-73, -46));
    Ok(count as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2021_17_01() {
        let result = solution_2021_17_01("inputs/2021/day17e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_17_01() {
        let result = solution_2021_17_01("inputs/2021/day17.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_17_02() {
        let result = solution_2021_17_02("inputs/2021/day17e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_17_02() {
        let result = solution_2021_17_02("inputs/2021/day17.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
