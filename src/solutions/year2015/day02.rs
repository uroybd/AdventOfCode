fn parse_dimensions(s: &str) -> Vec<usize> {
    let mut dims: Vec<usize> = s
        .splitn(3, 'x')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    dims.sort();
    dims
}

fn calculate_paper(vals: &Vec<usize>) -> usize {
    (3 * vals[0] * vals[1]) + (2 * vals[1] * vals[2]) + (2 * vals[2] * vals[0])
}

fn calculate_ribbon(vals: &Vec<usize>) -> usize {
    (2 * vals[0]) + (2 * vals[1]) + (vals[0] * vals[1] * vals[2])
}

pub fn solution_2015_02_01(filepath: String) -> Option<usize> {
    Some(
        std::fs::read_to_string(filepath)
            .expect("Invalid file.")
            .lines()
            .fold(0, |acc, l| acc + calculate_paper(&parse_dimensions(l))),
    )
}

pub fn solution_2015_02_02(filepath: String) -> Option<usize> {
    Some(
        std::fs::read_to_string(filepath)
            .expect("Invalid file.")
            .lines()
            .fold(0, |acc, l| acc + calculate_ribbon(&parse_dimensions(l))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_calculation() {
        assert_eq!(calculate_paper(&vec![2, 3, 4]), 58);
        assert_eq!(calculate_paper(&vec![1, 1, 10]), 43);
    }

    #[test]
    fn test_ribbon_calculation() {
        assert_eq!(calculate_ribbon(&vec![2, 3, 4]), 34);
        assert_eq!(calculate_ribbon(&vec![1, 1, 10]), 14);
    }

    #[test]
    fn test_2015_02_01() {
        let file_path = String::from("inputs/2015/day02.txt");
        let result = solution_2015_02_01(file_path).unwrap();
        assert_eq!(result, 1586300);
    }

    #[test]
    fn test_2015_02_02() {
        let file_path = String::from("inputs/2015/day02.txt");
        let result = solution_2015_02_02(file_path).unwrap();
        assert_eq!(result, 3737498);
    }
}
