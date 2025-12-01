fn parse_instruction(line: &str) -> (char, i32) {
    let direction = line.chars().next().unwrap();
    let value = line[1..].trim().parse().unwrap();
    (direction, value)
}

/// Get current dial position and number of full rotations made, that is, passed over 0
fn get_dial_position(current_pos: i32, ins: &(char, i32), dial_size: i32) -> (i32, i32) {
    let offset = match ins.0 {
        'L' => -ins.1,
        'R' => ins.1,
        _ => panic!("Invalid direction"),
    };
    let total = current_pos + offset;
    let new_pos = total.rem_euclid(dial_size);
    let full_rotations = if total < 0 {
        let mut val = -total / dial_size;
        if current_pos != 0 {
            val += 1;
        }
        val
    } else {
        total / dial_size
    };
    (new_pos, full_rotations)
}

const DIAL_SIZE: i32 = 100;

pub fn solution_2025_01_01(filepath: String) -> Option<i32> {
    let instructions: Vec<(char, i32)> = std::fs::read_to_string(filepath)
        .expect("Invalid file")
        .trim_end()
        .lines()
        .map(|line| parse_instruction(line))
        .collect();

    let mut current_position = 50;
    let mut zero_count = 0;
    for ins in instructions.iter() {
        (current_position, _) = get_dial_position(current_position, ins, DIAL_SIZE);
        if current_position == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn solution_2025_01_02(filepath: String) -> Option<i32> {
    let instructions: Vec<(char, i32)> = std::fs::read_to_string(filepath)
        .expect("Invalid file")
        .trim_end()
        .lines()
        .map(|line| parse_instruction(line))
        .collect();

    let mut current_position = 50;
    let mut zero_count = 0;
    println!("Instructions: {:?}", instructions);
    println!("Dial Size: {}", DIAL_SIZE);
    println!("Starting Position: {}", current_position);
    for ins in instructions.iter() {
        let (cur_pos, rotations) = get_dial_position(current_position, ins, DIAL_SIZE);
        println!(
            "Instruction: {:?}, New Position: {}, rotations: {}",
            ins, cur_pos, rotations
        );
        current_position = cur_pos;
        zero_count += rotations;
        if current_position == 0 && rotations == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2025_01_01_example() {
        let file_path = String::from("inputs/2025/day01e.txt");
        let result = solution_2025_01_01(file_path).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2025_01_01() {
        let file_path = String::from("inputs/2025/day01.txt");
        let result = solution_2025_01_01(file_path).unwrap();
        println!("Result: {}", result);
        assert_eq!(result, 1168);
    }

    #[test]
    fn test_2025_01_02_example() {
        let file_path = String::from("inputs/2025/day01e.txt");
        let result = solution_2025_01_02(file_path).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2025_01_02() {
        let file_path = String::from("inputs/2025/day01.txt");
        let result = solution_2025_01_02(file_path).unwrap();
        println!("Result: {}", result);
        assert_eq!(result, 7199);
    }
}
