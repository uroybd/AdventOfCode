// Advent of Code 2021 - Day 04

use std::fs;

#[derive(Debug)]
struct BoardCell {
    number: u64,
    marked: bool,
    x: usize,
    y: usize,
}

impl BoardCell {
    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug)]
struct Board {
    cells: Vec<BoardCell>,
    won: bool,
}

impl Board {
    fn new(value_string: String) -> Board {
        let mut board_value: Vec<BoardCell> = vec![];
        let rows = value_string.split('\n');
        for (row_idx, row) in rows.into_iter().enumerate() {
            let cols = row.split_whitespace();
            for (col_idx, col) in cols.into_iter().enumerate() {
                if !col.is_empty() {
                    let v = col.parse::<u64>().unwrap();
                    board_value.push(BoardCell {
                        number: v,
                        marked: false,
                        x: row_idx,
                        y: col_idx,
                    });
                }
            }
        }
        Board {
            cells: board_value,
            won: false,
        }
    }

    fn check(&mut self) -> (bool, u64) {
        let (marked, unmarked): (Vec<&BoardCell>, Vec<&BoardCell>) =
            self.cells.iter().partition(|&x| x.marked);

        for i in 0..5 {
            if marked.iter().filter(|&&c| c.x == i).count() == 5 {
                self.won = true;
                let sum = unmarked.iter().fold(0, |acc: u64, &x| acc + x.number);
                return (true, sum);
            }
            if marked.iter().filter(|&&c| c.y == i).count() == 5 {
                self.won = true;
                let sum = unmarked.iter().fold(0, |acc: u64, &x| acc + x.number);
                return (true, sum);
            }
        }
        (false, 0)
    }

    fn play(&mut self, val: u64, check: bool) -> (bool, u64) {
        let mut mark_flag = false;
        for cell in self.cells.iter_mut() {
            if cell.number == val {
                cell.mark();
                mark_flag = true;
            }
        }
        if !check {
            return (false, 0);
        }
        if !mark_flag {
            (false, 0)
        } else {
            self.check()
        }
    }
}

pub fn solution_2021_04_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut data: Vec<String> = fs::read_to_string(filepath)?
        .split("\n\n")
        .map(|d| d.to_string())
        .collect();
    let inputs: Vec<u64> = data
        .remove(0)
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    let mut boards: Vec<Board> = data.iter().map(|v| Board::new(v.to_string())).collect();
    let mut output = 0;
    'outer: for (idx, input) in inputs.iter().enumerate() {
        for board in boards.iter_mut() {
            if !board.won {
                let val = board.play(*input, idx >= 4);
                match val {
                    (true, x) => {
                        output = x * input;
                        break 'outer;
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(output as i64)
}

pub fn solution_2021_04_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let mut data: Vec<String> = fs::read_to_string(filepath)?
        .split("\n\n")
        .map(|d| d.to_string())
        .collect();
    let inputs: Vec<u64> = data
        .remove(0)
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    let mut output = 0;
    let mut boards: Vec<Board> = data.iter().map(|v| Board::new(v.to_string())).collect();
    for (idx, input) in inputs.iter().enumerate() {
        for board in boards.iter_mut() {
            if !board.won {
                let val = board.play(*input, idx >= 4);
                match val {
                    (true, x) => {
                        output = x * input;
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(output as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_04_01() {
        let result = solution_2021_04_01("inputs/2021/day04e.txt".to_string()).unwrap();
        assert_eq!(result, 4512);
    }

    #[test]
    #[ignore]
    fn output_2021_04_01() {
        let result = solution_2021_04_01("inputs/2021/day04.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    fn test_2021_04_02() {
        let result = solution_2021_04_02("inputs/2021/day04e.txt".to_string()).unwrap();
        assert_eq!(result, 1924);
    }

    #[test]
    #[ignore]
    fn output_2021_04_02() {
        let result = solution_2021_04_02("inputs/2021/day04.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
