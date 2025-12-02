fn parse_instruction(line: &str) -> i32 {
    let direction = line.chars().next().unwrap();
    let value: i32 = line[1..].trim().parse().unwrap();
    match direction {
        'L' => -value,
        'R' => value,
        _ => panic!("Invalid direction"),
    }
}

/// Get current dial position and number of full rotations made, that is, passed over 0
fn get_dial_position(current_pos: i32, offset: i32, dial_size: i32) -> (i32, i32) {
    let total = current_pos + offset;
    let new_pos = total.rem_euclid(dial_size);
    let mut full_rotations = total.abs() / dial_size;
    if total < 0 && current_pos != 0 {
        full_rotations += 1;
    }
    (new_pos, full_rotations)
}

const DIAL_SIZE: i32 = 100;
const STARTING_POSITION: i32 = 50;

pub fn solution_2025_01_01(filepath: String) -> Result<i32, Box<dyn std::error::Error>> {
    let (_, zero_count) = std::fs::read_to_string(filepath)?
        .trim_end()
        .lines()
        .map(parse_instruction)
        .fold(
            (STARTING_POSITION, 0),
            |(current_position, zero_count), ins| {
                let (new_position, _) = get_dial_position(current_position, ins, DIAL_SIZE);
                let mut zero_count = zero_count;
                if new_position == 0 {
                    zero_count += 1;
                };
                (new_position, zero_count)
            },
        );
    Ok(zero_count)
}

pub fn solution_2025_01_02(filepath: String) -> Result<i32, Box<dyn std::error::Error>> {
    let (_, zero_count) = std::fs::read_to_string(filepath)?
        .trim_end()
        .lines()
        .map(parse_instruction)
        .fold(
            (STARTING_POSITION, 0),
            |(current_position, zero_count), ins| {
                let (new_position, rotations) = get_dial_position(current_position, ins, DIAL_SIZE);
                let mut zero_count = zero_count + rotations;
                if new_position == 0 && rotations == 0 {
                    zero_count += 1;
                }
                (new_position, zero_count)
            },
        );
    Ok(zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2025_01_01_example() {
        let file_path = String::from("inputs/2025/day01e.txt");
        let result = solution_2025_01_01(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2025_01_01() {
        let file_path = String::from("inputs/2025/day01.txt");
        let result = solution_2025_01_01(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        println!("Result: {}", result);
        assert_eq!(result, 1168);
    }

    #[test]
    fn test_2025_01_02_example() {
        let file_path = String::from("inputs/2025/day01e.txt");
        let result = solution_2025_01_02(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2025_01_02() {
        let file_path = String::from("inputs/2025/day01.txt");
        let result = solution_2025_01_02(file_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        println!("Result: {}", result);
        assert_eq!(result, 7199);
    }
}
