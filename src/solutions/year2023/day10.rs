use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

// Advent of Code 2023 - Day 10

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Terrain {
    kind: char,
    x: usize,
    y: usize,
}

impl Terrain {
    pub fn get_hash_key(&self) -> String {
        format!("{}x{}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GroundMap {
    start: Terrain,
    terrains: HashMap<String, Terrain>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGroundMapError;

impl std::str::FromStr for GroundMap {
    type Err = ParseGroundMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.split_once('\n').unwrap().0.len();
        let mut terrains = HashMap::new();
        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let terrain = Terrain { kind: c, x, y };
                    if c == 'S' {
                        start = Some(terrain.clone());
                    }
                    terrains.insert(terrain.get_hash_key(), terrain);
                }
            }
        }
        Ok(Self {
            start: start.unwrap(),
            terrains,
            width,
            height,
        })
    }
}

const VALID_TOP: [char; 3] = ['|', 'F', '7'];
const VALID_BOTTOM: [char; 3] = ['|', 'L', 'J'];
const VALID_LEFT: [char; 3] = ['-', 'F', 'L'];
const VALID_RIGHT: [char; 3] = ['-', '7', 'J'];

impl GroundMap {
    fn top(&self, current: &Terrain) -> Option<&Terrain> {
        if current.y != 0 {
            if let Some(v) = self
                .terrains
                .get(&format!("{:?}x{:?}", current.x, current.y - 1))
            {
                if VALID_TOP.contains(&v.kind) {
                    return Some(v);
                }
            }
        }
        None
    }

    fn bottom(&self, current: &Terrain) -> Option<&Terrain> {
        if current.y != self.height - 1 {
            if let Some(v) = self
                .terrains
                .get(&format!("{:?}x{:?}", current.x, current.y + 1))
            {
                if VALID_BOTTOM.contains(&v.kind) {
                    return Some(v);
                }
            }
        }
        None
    }

    fn left(&self, current: &Terrain) -> Option<&Terrain> {
        if current.x != 0 {
            if let Some(v) = self
                .terrains
                .get(&format!("{:?}x{:?}", current.x - 1, current.y))
            {
                if VALID_LEFT.contains(&v.kind) {
                    return Some(v);
                }
            }
        }
        None
    }

    fn right(&self, current: &Terrain) -> Option<&Terrain> {
        if current.x != self.width - 1 {
            if let Some(v) = self
                .terrains
                .get(&format!("{:?}x{:?}", current.x + 1, current.y))
            {
                if VALID_RIGHT.contains(&v.kind) {
                    return Some(v);
                }
            }
        }
        None
    }

    fn get_actual_start_type(&self) -> Terrain {
        let start = self.start.clone();
        let possible = [
            self.top(&start),
            self.bottom(&start),
            self.left(&start),
            self.right(&start),
        ]
        .into_iter()
        .map(|x| x.map(|x| x.kind))
        .collect::<Vec<Option<char>>>();

        let kind = match possible.as_slice() {
            [_, _, Some(_), Some(_)] => '-',
            [Some(_), Some(_), _, _] => '|',
            [_, Some(_), _, Some(_)] => 'F',
            [Some(_), _, _, Some(_)] => 'L',
            [_, Some(_), Some(_), _] => '7',
            [Some(_), _, Some(_), _] => 'J',
            _ => unreachable!(),
        };
        Terrain {
            kind,
            x: start.x,
            y: start.y,
        }
    }

    fn accessible(&self, current: &Terrain) -> Vec<&Terrain> {
        let mut possible = vec![];
        match current.kind {
            '|' => {
                if let Some(top) = self.top(current) {
                    possible.push(top)
                }
                if let Some(bottom) = self.bottom(current) {
                    possible.push(bottom)
                }
            }
            '-' => {
                if let Some(left) = self.left(current) {
                    possible.push(left)
                }
                if let Some(right) = self.right(current) {
                    possible.push(right)
                }
            }
            'L' => {
                if let Some(top) = self.top(current) {
                    possible.push(top)
                }
                if let Some(right) = self.right(current) {
                    possible.push(right)
                }
            }
            'J' => {
                if let Some(left) = self.left(current) {
                    possible.push(left)
                }
                if let Some(top) = self.top(current) {
                    possible.push(top)
                }
            }
            '7' => {
                if let Some(left) = self.left(current) {
                    possible.push(left)
                }
                if let Some(bottom) = self.bottom(current) {
                    possible.push(bottom)
                }
            }
            'F' => {
                if let Some(right) = self.right(current) {
                    possible.push(right)
                }
                if let Some(bottom) = self.bottom(current) {
                    possible.push(bottom)
                }
            }
            _ => unreachable!(),
        }
        possible
    }

    fn find_largest_loop(&self) -> Vec<Terrain> {
        let mut current = *self
            .accessible(&self.get_actual_start_type())
            .first()
            .unwrap();

        let mut pipes: Vec<Terrain> = vec![self.start.clone()];
        while current.get_hash_key() != self.start.get_hash_key() {
            let neighbors = self.accessible(current);
            let next;
            if neighbors[0] == pipes.last().unwrap() {
                if neighbors.len() > 1 {
                    next = neighbors[1];
                } else {
                    break;
                }
            } else {
                next = neighbors[0];
            }
            pipes.push(current.clone());
            current = next;
        }
        pipes
    }

    fn search_and_mark(
        &mut self,
        curr: (usize, usize),
        pipes: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        if curr.0 >= self.width || curr.1 >= self.height {
            return vec![];
        }
        if pipes.contains(&curr) {
            return vec![];
        }
        let c_key = format!("{:?}x{:?}", curr.0, curr.1);
        if let Some(v) = self.terrains.get_mut(&c_key) {
            if v.kind == 'X' {
                return vec![];
            }
        }
        let neighbors = vec![
            (curr.0.wrapping_sub(1), curr.1),
            (curr.0 + 1, curr.1),
            (curr.0, curr.1.wrapping_sub(1)),
            (curr.0, curr.1 + 1),
        ];
        self.terrains
            .entry(c_key)
            .and_modify(|t| t.kind = 'X')
            .or_insert(Terrain {
                kind: 'X',
                x: curr.0,
                y: curr.1,
            });
        neighbors
    }

    fn count_enclosed(&self, area: &Vec<Terrain>) -> usize {
        let mut marked: GroundMap = self.clone();
        let mut prev = self.start.clone();
        let mut to_mark = VecDeque::with_capacity(self.width * self.height);
        let pipe_set: HashSet<(usize, usize)> = area.iter().map(|t| (t.x, t.y)).collect();
        for p in area {
            let curr = p.clone();
            match (
                curr.x as isize - prev.x as isize,
                curr.y as isize - prev.y as isize,
            ) {
                (1, 0) => {
                    to_mark.extend([(p.x, p.y + 1), (p.x.wrapping_sub(1), p.y + 1)]);
                }
                (0, 1) => {
                    to_mark.extend([
                        (p.x.wrapping_sub(1), p.y.wrapping_sub(1)),
                        (p.x.wrapping_sub(1), p.y),
                    ]);
                }
                (-1, 0) => {
                    to_mark.extend([(p.x, p.y.wrapping_sub(1)), (p.x + 1, p.y.wrapping_sub(1))]);
                }
                (0, -1) => {
                    to_mark.extend([(p.x + 1, p.y), (p.x + 1, p.y + 1)]);
                }
                _ => {}
            }
            prev = curr.clone();
        }

        while let Some(p) = to_mark.pop_front() {
            to_mark.extend(marked.search_and_mark(p, &pipe_set));
        }
        let mut marked_count = marked.terrains.values().filter(|t| t.kind == 'X').count();

        if let Some(v) = marked.terrains.get("0x0") {
            if v.kind == 'X' {
                let total = self.width * self.height;
                marked_count = total - marked_count - pipe_set.len();
            }
        }
        marked_count
    }
}

pub fn solution_day_10_01(file_path: String) -> Option<usize> {
    let ground_map = fs::read_to_string(file_path)
        .expect("Invalid File")
        .parse::<GroundMap>()
        .unwrap();
    let pipes = ground_map.find_largest_loop();
    let val = pipes.len();
    Some((val / 2) + (val % 2))
}

pub fn solution_day_10_02(file_path: String) -> Option<usize> {
    let ground_map = fs::read_to_string(file_path)
        .expect("Invalid File")
        .parse::<GroundMap>()
        .unwrap();
    let pipes = ground_map.find_largest_loop();
    Some(ground_map.count_enclosed(&pipes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_01() {
        let file_path: String = String::from("inputs/2023/day10e.txt");
        let result = solution_day_10_01(file_path).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_day_10_02() {
        let file_path: String = String::from("inputs/2023/day10e2.txt");
        let result = solution_day_10_02(file_path).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    #[ignore]
    fn output_day_10_01() {
        let file_path: String = String::from("inputs/2023/day10.txt");
        let result = solution_day_10_01(file_path).unwrap();
        assert_eq!(result, 6860);
    }

    #[test]
    #[ignore]
    fn output_day_10_02() {
        let file_path: String = String::from("inputs/2023/day10.txt");
        let result = solution_day_10_02(file_path).unwrap();
        assert_eq!(result, 343);
    }
}
