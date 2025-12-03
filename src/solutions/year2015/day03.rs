use std::collections::HashSet;

fn gen_key(pos: &(isize, isize)) -> String {
    format!("{}x{}", pos.0, pos.1)
}

struct Walker(isize, isize);

impl Walker {
    fn new() -> Self {
        Self(0, 0)
    }

    fn key(&self) -> String {
        gen_key(&(self.0, self.1))
    }

    fn walk(&mut self, dir: &char) -> String {
        match dir {
            '>' => self.0 += 1,
            '<' => self.0 -= 1,
            '^' => self.1 += 1,
            'v' => self.1 -= 1,
            _ => unreachable!(),
        };
        self.key()
    }
}

pub fn solution_2015_03_01(filepath: String) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut santa = Walker::new();
    visited.insert(santa.key());

    for dir in std::fs::read_to_string(filepath)
        .expect("Invalid file.")
        .trim_end()
        .chars()
    {
        visited.insert(santa.walk(&dir));
    }
    Some(visited.len())
}

pub fn solution_2015_03_02(filepath: String) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut santas = [Walker::new(), Walker::new()];
    visited.insert(santas[0].key());

    for (i, dir) in std::fs::read_to_string(filepath)
        .expect("Invalid file.")
        .trim_end()
        .chars()
        .enumerate()
    {
        visited.insert(santas[i % 2].walk(&dir));
    }
    Some(visited.len())
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution_2015_03_01() {
        let file_path = String::from("inputs/2015/day03.txt");
        assert_eq!(solution_2015_03_01(file_path), Some(2565));
    }

    #[test]
    fn test_solution_2015_03_02() {
        let file_path = String::from("inputs/2015/day03.txt");
        assert_eq!(solution_2015_03_02(file_path), Some(2639));
    }
}
