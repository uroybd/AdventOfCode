// Advent of Code 2021 - Day 05

use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_comma_separated(data_string: &str) -> Point {
        let pair: Vec<usize> = data_string
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Point {
            x: pair[0],
            y: pair[1],
        }
    }

    fn get_hash_key(&self) -> String {
        format!("{}x{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn from_arrowed_pair(data_string: &str) -> Line {
        let mut pair: Vec<Point> = data_string
            .split(" -> ")
            .map(|x| Point::from_comma_separated(x))
            .collect();

        Line {
            a: pair.remove(0),
            b: pair.remove(0),
        }
    }

    fn create_line_series(&self, allow_diagonal: bool) -> Vec<Point> {
        let mut series: Vec<Point> = vec![];
        if self.a.x == self.b.x {
            let (mut start, mut end) = (self.a.y, self.b.y);
            if start > end {
                end = start;
                start = self.b.y;
            }
            for i in start..end + 1 {
                series.push(Point { x: self.a.x, y: i })
            }
        } else if self.a.y == self.b.y {
            let (mut start, mut end) = (self.a.x, self.b.x);
            if start > end {
                end = start;
                start = self.b.x;
            }
            for i in start..end + 1 {
                series.push(Point { x: i, y: self.a.y })
            }
        } else {
            if !allow_diagonal {
                return series;
            }
            let (mut reverse_x, mut reverse_y, mut start_x, mut end_x, mut start_y, mut end_y) =
                (false, false, self.a.x, self.b.x, self.a.y, self.b.y);

            if start_y > end_y {
                end_y = start_y;
                start_y = self.b.y;
                reverse_y = true;
            }
            if start_x > end_x {
                end_x = start_x;
                start_x = self.b.x;
                reverse_x = true;
            }

            let mut xs: Vec<usize> = (start_x..end_x + 1).collect();
            let mut ys: Vec<usize> = (start_y..end_y + 1).collect();

            if reverse_x {
                xs.reverse();
            }
            if reverse_y {
                ys.reverse();
            }
            for i in 0..ys.len() {
                series.push(Point { x: xs[i], y: ys[i] })
            }
        }
        series
    }
}

pub fn solution_2021_05_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<String> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let mut cloud_map = HashMap::new();
    for line in lines.iter() {
        let line = Line::from_arrowed_pair(line);
        let values = line.create_line_series(false);
        for v in values {
            let key = v.get_hash_key();
            let v = cloud_map.entry(key).or_insert(0);
            *v += 1;
        }
    }
    let count = cloud_map.values().into_iter().filter(|&x| x > &1).count();
    Ok(count as i64)
}

pub fn solution_2021_05_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<String> = fs::read_to_string(filepath)?
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let mut cloud_map = HashMap::new();
    for line in lines.iter() {
        let line = Line::from_arrowed_pair(line);
        let values = line.create_line_series(true);
        for v in values {
            let key = v.get_hash_key();
            let v = cloud_map.entry(key).or_insert(0);
            *v += 1;
        }
    }
    let count = cloud_map.values().into_iter().filter(|&x| x > &1).count();
    Ok(count as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_05_01() {
        let result = solution_2021_05_01("inputs/2021/day05e.txt".to_string()).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    #[ignore]
    fn output_2021_05_01() {
        let result = solution_2021_05_01("inputs/2021/day05.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_05_02() {
        let result = solution_2021_05_02("inputs/2021/day05e.txt".to_string()).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn output_2021_05_02() {
        let result = solution_2021_05_02("inputs/2021/day05.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
