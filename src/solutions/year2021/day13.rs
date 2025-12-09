// Advent of Code 2021 - Day 13

use std::fs;

fn get_input(file_path: String) -> (Vec<Vec<usize>>, Vec<Vec<String>>) {
    let content: Vec<String> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();
    let points: Vec<Vec<usize>> = content[0]
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let instructions: Vec<Vec<String>> = content[1]
        .split('\n')
        .map(|x| x.split('=').map(|s| s.to_string()).collect::<Vec<String>>())
        .collect();
    (points, instructions)
}

fn create_page(points: &Vec<Vec<usize>>) -> Vec<Vec<char>> {
    let (mut x_size, mut y_size) = (0, 0);
    for pair in points {
        if pair[0] > x_size {
            x_size = pair[0];
        }
        if pair[1] > y_size {
            y_size = pair[1];
        }
    }
    let mut page: Vec<Vec<char>> = vec![vec!['.'; x_size + 1]; y_size + 1];
    for pair in points {
        page[pair[1]][pair[0]] = '#'
    }
    page
}

fn fold_x(page: &Vec<Vec<char>>, pos: usize) -> Vec<Vec<char>> {
    return page
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(pos);
            let mut left = left.to_vec();
            right[1..]
                .iter()
                .rev()
                .enumerate()
                .for_each(|(index, item)| {
                    if item == &'#' {
                        left[index] = *item;
                    }
                });
            left
        })
        .collect();
}

fn fold_y(page: &Vec<Vec<char>>, pos: usize) -> Vec<Vec<char>> {
    let (top, bottom) = page.split_at(pos);
    let mut top = top.to_vec();
    bottom[1..]
        .iter()
        .rev()
        .enumerate()
        .for_each(|(index, line)| {
            line.iter().enumerate().for_each(|(i, val)| {
                if val == &'#' {
                    top[index][i] = *val;
                }
            })
        });
    top
}

pub fn solution_2021_13_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let (points, instructions) = get_input(filepath);
    let mut page = create_page(&points);
    match instructions[0][0].as_str() {
        "fold along y" => page = fold_y(&page, instructions[0][1].parse::<usize>().unwrap()),
        "fold along x" => page = fold_x(&page, instructions[0][1].parse::<usize>().unwrap()),
        _ => (),
    }
    let count = page.iter().fold(0, |acc, x| {
        acc + x
            .iter()
            .fold(0, |cc, y| if y == &'#' { cc + 1 } else { cc })
    });
    Ok(count)
}

pub fn solution_2021_13_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let (points, instructions) = get_input(filepath);
    let mut page = create_page(&points);
    for instruction in instructions.iter() {
        match instruction[0].as_str() {
            "fold along y" => page = fold_y(&page, instruction[1].parse::<usize>().unwrap()),
            "fold along x" => page = fold_x(&page, instruction[1].parse::<usize>().unwrap()),
            _ => (),
        }
    }
    println!("\nDay 13 Part 2 - Code:");
    for line in &page {
        println!("{}", line.iter().collect::<String>());
    }
    println!();
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_13_01() {
        let result = solution_2021_13_01("inputs/2021/day13e.txt".to_string()).unwrap();
        assert_eq!(result, 17);
    }

    #[test]
    #[ignore]
    fn output_2021_13_01() {
        let result = solution_2021_13_01("inputs/2021/day13.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_13_02() {
        let result = solution_2021_13_02("inputs/2021/day13e.txt".to_string()).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn output_2021_13_02() {
        let result = solution_2021_13_02("inputs/2021/day13.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert_eq!(result, 0);
    }
}
