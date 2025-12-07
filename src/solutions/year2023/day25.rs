use std::collections::HashMap;

// Advent of Code 2023 - Day 25
// An undirectional graph
struct WireDiagram {
    wires: HashMap<String, HashMap<String, bool>>,
}

impl WireDiagram {
    fn from_str(input: &str) -> Self {
        let mut diagram = WireDiagram {
            wires: HashMap::new(),
        };
        for line in input.lines() {
            let mut parts = line.splitn(2, ": ");
            let wire_name = parts.next().unwrap().to_string();
            parts.next().unwrap().split(" ").for_each(|conn| {
                diagram
                    .wires
                    .entry(wire_name.clone())
                    .or_default()
                    .insert(conn.to_string(), true);
                diagram
                    .wires
                    .entry(conn.to_string())
                    .or_default()
                    .insert(wire_name.clone(), true);
            });
        }
        diagram
    }

    fn remove_connection(&mut self, a: &str, b: &str) {
        self.wires.get_mut(a).unwrap().remove(b);
        self.wires.get_mut(b).unwrap().remove(a);
    }
}

pub fn solution_2023_25_01(file_path: String) -> anyhow::Result<usize> {
    anyhow::bail!("Yet to be implemented")
}

pub fn solution_2023_25_02(file_path: String) -> anyhow::Result<usize> {
    anyhow::bail!("Yet to be implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_25_01() {
        let file_path: String = String::from("inputs/2023/day25e.txt");
        let result = solution_2023_25_01(file_path).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_2023_25_02() {
        let file_path: String = String::from("inputs/2023/day25e.txt");
        let result = solution_2023_25_02(file_path).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn output_2023_25_01() {
        let file_path: String = String::from("inputs/2023/day25.txt");
        let result = solution_2023_25_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_2023_25_02() {
        let file_path: String = String::from("inputs/2023/day25.txt");
        let result = solution_2023_25_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
