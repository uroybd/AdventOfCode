// Advent of Code 2023 - Day 21
use std::collections::HashSet;


#[derive(Debug, Clone)]
struct GardenMap {
    map: Vec<Vec<bool>>,
    start: (usize, usize),
    height: usize,
    width: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGardenMapError;

impl std::str::FromStr for GardenMap {
    type Err = ParseGardenMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(c != '#');
                if c == 'S' {
                    start = (x, y);
                }
            }
            map.push(row);
        }
        Ok(GardenMap {
            map: map.clone(),
            start,
            height: map.len(),
            width: map[0].len(),
        })
    }
}

impl GardenMap {
    fn find_possible_moves(&self, pos: (isize, isize)) -> Vec<(isize, isize)> {
        let mut moves = Vec::new();
        let (x, y) = pos;
        if let Some(true) = self.at((x - 1, y)) {
            moves.push((x - 1, y));
        }
        if let Some(true) = self.at((x + 1, y)) {
            moves.push((x + 1, y));
        }
        if let Some(true) = self.at((x, y - 1)) {
            moves.push((x, y - 1));
        }
        if let Some(true) = self.at((x, y + 1)) {
            moves.push((x, y + 1));
        }
        moves
    }

    fn round(&self, pos: (isize, isize)) -> (isize, isize) {
        let (x, y) = pos;
        (
            x.rem_euclid(self.width as isize),
            y.rem_euclid(self.height as isize),
        )
    }

    fn at(&self, pos: (isize, isize)) -> Option<bool> {
        let (x, y) = self.round(pos);
        self.map[y as usize][x as usize].into()
    }

    fn possible_move_after_steps(&self, steps: usize) -> HashSet<(isize, isize)> {
        let mut moves = HashSet::from([(self.start.0 as isize, self.start.1 as isize)]);
        for _ in 0..steps {
            let new_moves =
                HashSet::from_iter(moves.iter().flat_map(|x| self.find_possible_moves(*x)));

            moves = new_moves;
        }
        moves
    }

    // fn possible_move_after_steps_in_infinite_wrap(&self, steps: usize) -> f64 {
    //     let b = Matrix3x1::from_iterator((0..3).map(|i| {
    //         self.possible_move_after_steps((i * self.width) + steps % self.width)
    //             .len() as f64
    //     }));
    //     let a: Matrix3<f64> = Matrix3::from([[0., 0., 1.], [1., 1., 1.], [4., 2., 1.]]);
    //     let decomp = a.lu();
    //     let x = decomp.solve(&b).unwrap();
    //     println!("{:?}", x);
    //     x[0].abs() * (steps as f64).powi(2) + x[1].abs() * steps as f64 + x[2].abs()
    // }

    fn possible_move_after_steps_in_infinite_wrap(&self, steps: usize) -> usize {
        let mut history = vec![];
        for c in 1..=steps {
            if c % self.height == self.height / 2 {
                history.push(self.possible_move_after_steps(c).len());
                if let &[y0, y1, y2] = &history[..] {
                    let x = steps / self.height;
                    return (x * x * (y0 + y2 - 2 * y1) + x * (4 * y1 - 3 * y0 - y2) + 2 * y0) / 2;
                }
            }
        }
        0
    }
}

pub fn solution_2023_21_01(file_path: String, moves: usize) -> Option<usize> {
    let map = std::fs::read_to_string(file_path)
        .unwrap()
        .parse::<GardenMap>()
        .unwrap();
    Some(map.possible_move_after_steps(moves).len())
}

pub fn solution_2023_21_02(file_path: String, moves: usize) -> Option<usize> {
    let map = std::fs::read_to_string(file_path)
        .unwrap()
        .parse::<GardenMap>()
        .unwrap();
    Some(map.possible_move_after_steps_in_infinite_wrap(moves))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_21_01() {
        let file_path: String = String::from("inputs/2023/day21e.txt");
        let result = solution_2023_21_01(file_path, 6).unwrap();
        assert_eq!(result, 16);
    }

    #[test]
    fn test_2023_21_02() {
        let file_path: String = String::from("inputs/2023/day21e.txt");
        let result = solution_2023_21_02(file_path, 26501365).unwrap();
        assert_eq!(result, 528192899606863);
    }

    #[test]
    #[ignore]
    fn output_day_21_01() {
        let file_path: String = String::from("inputs/2023/day21.txt");
        let result = solution_2023_21_01(file_path, 64).unwrap();
        assert_eq!(result, 3853);
    }

    #[test]
    #[ignore]
    fn output_day_21_02() {
        let file_path: String = String::from("inputs/2023/day21.txt");
        let result = solution_2023_21_02(file_path, 26501365).unwrap();
        assert_eq!(result, 639051580070841);
    }
}
