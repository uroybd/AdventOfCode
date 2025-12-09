// Advent of Code 2021 - Day 20

use std::fs;

fn add_padding(data: &mut Vec<Vec<char>>, bit: char) {
    for row in data.iter_mut() {
        row.insert(0, bit);
        row.push(bit);
    }
    data.insert(0, vec![bit; data[0].len()]);
    data.push(vec![bit; data[0].len()]);
}

fn get_value(data: &Vec<Vec<char>>, x: isize, y: isize, bg: char) -> usize {
    let x_length = data[0].len() as isize;
    let y_length = data.len() as isize;
    (y - 1..y + 2)
        .flat_map(|i| {
            (x - 1..x + 2).map(move |j| {
                if i < 0 || i >= y_length || j < 0 || j >= x_length {
                    return bg;
                } else {
                    return data[i as usize][j as usize];
                };
            })
        })
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, bit)| {
            let bit_num = bit.to_digit(10).unwrap();
            acc + (bit_num as usize * usize::pow(2, idx as u32))
        })
}

fn enhance(data: &mut Vec<Vec<char>>, lookup: &Vec<char>, pad: char) -> Vec<Vec<char>> {
    let mut val: Vec<Vec<char>> = data.clone();
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            val[y][x] = lookup[get_value(data, x as isize, y as isize, pad)];
        }
    }
    val
}

fn count(data: &Vec<Vec<char>>) -> usize {
    data.iter().flatten().filter(|&x| x == &'1').count()
}

fn solve(filepath: String, limit: usize) -> usize {
    let data: Vec<String> = fs::read_to_string(filepath)
        .expect("Invalid File")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();
    let lookup: Vec<char> = data[0]
        .chars()
        .map(|c| match c {
            '#' => '1',
            _ => '0',
        })
        .collect();
    let mut image: Vec<Vec<char>> = data[1]
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    '#' => '1',
                    _ => '0',
                })
                .collect()
        })
        .collect();

    let mut pad = '0';
    let should_flip = lookup[0] == '1' && lookup[lookup.len() - 1] == '0';
    for _ in 0..limit {
        add_padding(&mut image, pad);
        image = enhance(&mut image, &lookup, pad);
        if should_flip {
            if pad == '0' {
                pad = '1'
            } else {
                pad = '0'
            }
        }
    }
    count(&image)
}

pub fn solution_2021_20_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(solve(filepath, 2) as i64)
}

pub fn solution_2021_20_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(solve(filepath, 50) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2021_20_01() {
        let result = solution_2021_20_01("inputs/2021/day20e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_20_01() {
        let result = solution_2021_20_01("inputs/2021/day20.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_20_02() {
        let result = solution_2021_20_02("inputs/2021/day20e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_20_02() {
        let result = solution_2021_20_02("inputs/2021/day20.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
