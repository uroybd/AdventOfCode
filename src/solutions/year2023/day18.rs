use std::fs;

// Advent of Code 2023 - Day 18

struct Instruction {
    dir: u8,
    len: usize,
    c: usize,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.splitn(3, ' ');
        let dir = match s.next().unwrap() {
            "R" => 0,
            "D" => 1,
            "L" => 2,
            "U" => 3,
            _ => unreachable!(),
        };
        let len = s.next().unwrap().parse::<usize>().unwrap();
        let c = usize::from_str_radix(
            s.next()
                .unwrap()
                .trim_matches(|s| s == '(' || s == ')' || s == '#'),
            16,
        )
        .unwrap();

        Ok(Self { dir, len, c })
    }
}

fn get_end(start: (isize, isize), dir: u8, len: usize) -> (isize, isize) {
    match dir {
        0 => (start.0.wrapping_add_unsigned(len), start.1),
        1 => (start.0, start.1.wrapping_add_unsigned(len)),
        2 => (start.0.wrapping_sub_unsigned(len), start.1),
        3 => (start.0, start.1.wrapping_sub_unsigned(len)),
        _ => unreachable!(),
    }
}

fn shoelace_area(
    instructions: &[Instruction],
    vector_func: fn(&Instruction) -> (u8, usize),
) -> usize {
    let mut perimeter = 0;
    let mut sum = 0;
    let mut prev = (0, 0);
    for i in instructions {
        let (dir, len) = vector_func(i);

        let next = get_end(prev, dir, len);
        sum += (prev.1 + next.1) * (prev.0 - next.0);
        perimeter += len;
        prev = next;
    }
    perimeter.wrapping_add_signed(sum) / 2 + 1
}
pub fn solution_2023_18_01(file_path: String) -> Option<usize> {
    let plan: Vec<Instruction> = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();
    Some(shoelace_area(&plan, |i| (i.dir, i.len)))
}

pub fn solution_2023_18_02(file_path: String) -> Option<usize> {
    let plan: Vec<Instruction> = fs::read_to_string(file_path)
        .expect("Invalid Input File.")
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();
    Some(shoelace_area(&plan, |i| ((i.c & 3) as u8, (i.c >> 4))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_18_01() {
        let file_path: String = String::from("inputs/2023/day18e.txt");
        let result = solution_2023_18_01(file_path).unwrap();
        assert_eq!(result, 62);
    }

    #[test]
    fn test_2023_18_02() {
        let file_path: String = String::from("inputs/2023/day18e.txt");
        let result = solution_2023_18_02(file_path).unwrap();
        assert_eq!(result, 952408144115);
    }

    #[test]
    #[ignore]
    fn output_day_18_01() {
        let file_path: String = String::from("inputs/2023/day18.txt");
        let result = solution_2023_18_01(file_path).unwrap();
        assert_eq!(result, 36807);
    }

    #[test]
    #[ignore]
    fn output_day_18_02() {
        let file_path: String = String::from("inputs/2023/day18.txt");
        let result = solution_2023_18_02(file_path).unwrap();
        assert_eq!(result, 48797603984357);
    }
}
