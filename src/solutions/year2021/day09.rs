// Advent of Code 2021 - Day 09

use std::collections::HashMap;
use std::fs;

fn get_adjacencies(
    row: usize,
    col: usize,
    row_size: usize,
    col_size: usize,
    data: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let mut return_data: Vec<usize> = vec![];
    if col >= 1 {
        return_data.push(data[row][col - 1]);
    }
    if col + 1 < col_size {
        return_data.push(data[row][col + 1]);
    }
    if row >= 1 {
        return_data.push(data[row - 1][col]);
    }
    if row + 1 < row_size {
        return_data.push(data[row + 1][col]);
    }
    return_data
}

fn is_lowest(val: &usize, adjacents: &Vec<usize>) -> bool {
    adjacents.iter().all(|x| x > val)
}

pub fn solution_2021_09_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<Vec<usize>> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let row_size = data.len();
    let col_size = data[0].len();
    let risks: usize = data.iter().enumerate().fold(0, |acc, (row, row_data)| {
        acc + row_data
            .iter()
            .enumerate()
            .fold(0, |internal_acc, (col, val)| {
                let adj = get_adjacencies(row, col, row_size, col_size, &data);
                let is_lowest = is_lowest(val, &adj);
                if is_lowest {
                    internal_acc + val + 1
                } else {
                    internal_acc
                }
            })
    });
    Ok(risks as i64)
}

fn get_basin(
    row: usize,
    col: usize,
    row_size: usize,
    col_size: usize,
    data: &Vec<Vec<usize>>,
    visit_map: &mut HashMap<String, bool>,
) -> Vec<usize> {
    let key = format!("{}x{}", row, col);
    let visited = visit_map.entry(key.to_string()).or_insert(false);

    if *visited {
        return vec![];
    } else {
        let d = data[row][col];
        let e = visit_map.entry(key).or_insert(true);
        *e = true;
        if d == 9 {
            return vec![];
        } else {
            let mut return_data: Vec<usize> = vec![d];

            if col >= 1 && data[row][col - 1] < 9 {
                return_data.extend(get_basin(row, col - 1, row_size, col_size, data, visit_map));
            }
            if col + 1 < col_size && data[row][col + 1] < 9 {
                return_data.extend(get_basin(row, col + 1, row_size, col_size, data, visit_map));
            }
            if row >= 1 && data[row - 1][col] < 9 {
                return_data.extend(get_basin(row - 1, col, row_size, col_size, data, visit_map));
            }
            if row + 1 < row_size && data[row + 1][col] < 9 {
                return_data.extend(get_basin(row + 1, col, row_size, col_size, data, visit_map));
            }

            return_data
        }
    }
}

pub fn solution_2021_09_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let data: Vec<Vec<usize>> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let row_size = data.len();
    let col_size = data[0].len();
    let mut visit_map: HashMap<String, bool> = HashMap::new();
    let mut basins: Vec<usize> = vec![];
    for (row, row_data) in data.iter().enumerate() {
        for (col, _) in row_data.iter().enumerate() {
            let basin = get_basin(row, col, row_size, col_size, &data, &mut visit_map);
            basins.push(basin.len());
        }
    }
    basins.sort_by(|a, b| b.cmp(a));
    Ok((basins[0] * basins[1] * basins[2]) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_09_01() {
        let result = solution_2021_09_01("inputs/2021/day09e.txt".to_string()).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    #[ignore]
    fn output_2021_09_01() {
        let result = solution_2021_09_01("inputs/2021/day09.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_09_02() {
        let result = solution_2021_09_02("inputs/2021/day09e.txt".to_string()).unwrap();
        assert_eq!(result, 1134);
    }

    #[test]
    #[ignore]
    fn output_2021_09_02() {
        let result = solution_2021_09_02("inputs/2021/day09.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
