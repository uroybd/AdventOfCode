// Advent of Code 2021 - Day 25

use std::fs;

fn move_eastward(data: &mut Vec<Vec<char>>) -> bool {
    let y_len = data.len();
    let x_len = data[0].len();
    let mut moved = false;
    for y in 0..y_len {
        let mut x = 0;
        let mut occupied = vec![];
        while x < x_len {
            match data[y][x] {
                '>' => {
                    occupied.push(x);
                    let mut x_r = x + 1;
                    if x == x_len - 1 {
                        x_r = 0;
                    }
                    if data[y][x_r] == '.' && !occupied.iter().any(|&x| x == x_r) {
                        data[y][x] = '.';
                        data[y][x_r] = '>';
                        moved = true;
                        x += 1;
                    }
                }
                'v' => occupied.push(x),
                _ => (),
            }
            x += 1;
        }
    }
    moved
}

fn move_southward(data: &mut Vec<Vec<char>>) -> bool {
    let y_len = data.len();
    let x_len = data[0].len();
    let mut moved = false;
    for x in 0..x_len {
        let mut y = 0;
        let mut occupied = vec![];
        while y < y_len {
            match data[y][x] {
                'v' => {
                    occupied.push(y);
                    let mut y_r = y + 1;
                    if y == y_len - 1 {
                        y_r = 0;
                    }
                    if data[y_r][x] == '.' && !occupied.iter().any(|&y| y == y_r) {
                        data[y][x] = '.';
                        data[y_r][x] = 'v';
                        moved = true;
                        y += 1;
                    }
                }
                '>' => occupied.push(y),
                _ => (),
            }
            y += 1;
        }
    }
    moved
}

fn move_cucumbers(data: &mut Vec<Vec<char>>) -> bool {
    let m1 = move_eastward(data);
    let m2 = move_southward(data);
    return m1 || m2;
}

pub fn solution_2021_25_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut input: Vec<Vec<char>> = fs::read_to_string(filepath)?
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut steps = 1;
    loop {
        let moved = move_cucumbers(&mut input);
        if moved {
            steps += 1;
        } else {
            break;
        }
    }
    Ok(steps)
}

pub fn solution_2021_25_02(_filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    // Day 25 part 2 is traditionally a freebie after completing all other days
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_25_01() {
        let result = solution_2021_25_01("inputs/2021/day25e.txt".to_string()).unwrap();
        assert_eq!(result, 58);
    }

    #[test]
    #[ignore]
    fn output_2021_25_01() {
        let result = solution_2021_25_01("inputs/2021/day25.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_25_02() {
        let result = solution_2021_25_02("inputs/2021/day25e.txt".to_string()).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn output_2021_25_02() {
        let result = solution_2021_25_02("inputs/2021/day25.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert_eq!(result, 0);
    }
}
