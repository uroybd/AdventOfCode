// Advent of Code 2025 - Day 08

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Coordinate(usize, usize, usize);

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> usize {
        // Use squared Euclidean distance
        self.0.abs_diff(other.0).pow(2)
            + self.1.abs_diff(other.1).pow(2)
            + self.2.abs_diff(other.2).pow(2)
    }

    fn from_str(s: &str) -> Option<Coordinate> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return None;
        }
        let x = parts[0].parse::<usize>().ok()?;
        let y = parts[1].parse::<usize>().ok()?;
        let z = parts[2].parse::<usize>().ok()?;
        Some(Coordinate(x, y, z))
    }
}

struct Circuit {
    coords: Vec<Coordinate>,
    distance_matrix: HashMap<(Coordinate, Coordinate), usize>,
    connections: HashMap<Coordinate, Vec<Coordinate>>,
}

impl Circuit {
    fn from_str(inp: &str) -> Circuit {
        let mut coords = Vec::new();
        for line in inp.lines() {
            coords.push(Coordinate::from_str(line.trim()).unwrap());
        }
        coords.sort();
        let distance_matrix = HashMap::new();
        let connections = HashMap::new(); // Placeholder for actual connection logic
        Circuit {
            coords,
            distance_matrix,
            connections,
        }
    }
    fn build_distance_matrix(&mut self) {
        let mut matrix = HashMap::new();
        let len = self.coords.len();
        for i in 0..len {
            let curr_coord = &self.coords[i];
            for j in i + 1..len {
                let other_coord = &self.coords[j];
                let dist = curr_coord.distance(other_coord);
                matrix.insert((curr_coord.clone(), other_coord.clone()), dist);
            }
        }
        self.distance_matrix = matrix
    }

    /// Connect two coordinates with an edge.
    /// Only creates the connection if the nodes are not already in the same connected component.
    /// Returns true if connection was made, false if already in same component.
    /// This implements the union-find logic for Kruskal's algorithm - prevents cycles.
    fn connect(&mut self, c1: &Coordinate, c2: &Coordinate) -> bool {
        // Check if they're in the same connected component (directly or indirectly connected)
        if self.are_connected(c1, c2) {
            return false; // Already in same component
        }

        // Add bidirectional connection
        self.connections
            .entry(c1.clone())
            .or_default()
            .push(c2.clone());
        self.connections
            .entry(c2.clone())
            .or_default()
            .push(c1.clone());

        true
    }

    /// Check if two nodes are in the same connected component (reachable from each other).
    /// Uses BFS to determine connectivity.
    fn are_connected(&self, c1: &Coordinate, c2: &Coordinate) -> bool {
        if c1 == c2 {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(c1.clone());
        visited.insert(c1.clone());

        while let Some(current) = queue.pop_front() {
            if current == *c2 {
                return true; // Found a path
            }

            if let Some(neighbors) = self.connections.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        false // No path found
    }

    /// Returns a vector of vectors, where each inner vector is a chain of connected nodes.
    /// Only includes chains with at least 2 nodes (i.e., has at least one connection).
    fn get_all_subgraphs(&self) -> Vec<Vec<Coordinate>> {
        let mut result = Vec::new();
        let mut global_visited = HashSet::new();

        // Iterate over all nodes
        for coord in &self.coords {
            // Skip if we've already visited this node in a previous chain
            if global_visited.contains(coord) {
                continue;
            }

            // Skip nodes with no connections
            if let Some(neighbors) = self.connections.get(coord) {
                if neighbors.is_empty() {
                    continue;
                }
            } else {
                continue;
            }

            // Find the connected component starting from this node
            let mut component = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            let mut visited = HashSet::new();

            queue.push_back(coord.clone());
            visited.insert(coord.clone());

            while let Some(current) = queue.pop_front() {
                component.push(current.clone());
                global_visited.insert(current.clone());

                if let Some(neighbors) = self.connections.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }

            // Only include if it's a chain (has connections)
            if component.len() > 1 {
                result.push(component);
            }
        }

        result
    }
}

pub fn solution_2025_08_01(
    file_path: String,
    connection_sample_size: usize,
) -> anyhow::Result<usize> {
    let mut circuit = Circuit::from_str(&std::fs::read_to_string(file_path)?);
    circuit.build_distance_matrix();
    let binding = circuit.distance_matrix.clone();
    let mut matrix = binding.iter().collect::<Vec<_>>();
    // Sort by distance
    matrix.sort_by_key(|k| k.1);
    for e in matrix[0..connection_sample_size].iter() {
        let c = e.0;
        circuit.connect(&c.0, &c.1);
    }

    let mut circuit_lengths: Vec<usize> = circuit
        .get_all_subgraphs()
        .iter()
        .map(|c| c.len())
        .collect();

    // Add single-node circuits (nodes with no connections)
    for coord in &circuit.coords {
        if !circuit.connections.contains_key(coord) {
            circuit_lengths.push(1);
        }
    }

    circuit_lengths.sort();
    circuit_lengths.reverse();
    Ok(circuit_lengths.iter().take(3).product())
}

pub fn solution_2025_08_02(file_path: String) -> anyhow::Result<usize> {
    let mut circuit = Circuit::from_str(&std::fs::read_to_string(file_path)?);
    circuit.build_distance_matrix();

    // Get all edges sorted by distance
    let binding = circuit.distance_matrix.clone();
    let mut matrix = binding.iter().collect::<Vec<_>>();
    matrix.sort_by_key(|k| k.1);

    let mut last_connection: Option<(Coordinate, Coordinate)> = None;
    let total_nodes = circuit.coords.len();
    let mut num_components = total_nodes; // Initially each node is its own component

    // Keep connecting until we have just one component
    for e in matrix.iter() {
        let c = e.0;
        if circuit.connect(&c.0, &c.1) {
            // Successful connection - two components merged into one
            num_components -= 1;
            last_connection = Some((c.0.clone(), c.1.clone()));
            if num_components == 1 {
                break;
            }
        }
    }

    // Return the product of X coordinates of the last connection
    if let Some((c1, c2)) = last_connection {
        Ok(c1.0 * c2.0)
    } else {
        anyhow::bail!("No connections were made")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2025_08_01() {
        let file_path: String = String::from("inputs/2025/day08e.txt");
        let result = solution_2025_08_01(file_path, 10).unwrap();
        assert_eq!(result, 40);
    }

    #[test]
    fn test_2025_08_02() {
        let file_path: String = String::from("inputs/2025/day08e.txt");
        let result = solution_2025_08_02(file_path).unwrap();
        assert_eq!(result, 25272);
    }

    #[test]
    #[ignore]
    fn output_2025_08_01() {
        let file_path: String = String::from("inputs/2025/day08.txt");
        let result = solution_2025_08_01(file_path, 1000);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_2025_08_02() {
        let file_path: String = String::from("inputs/2025/day08.txt");
        let result = solution_2025_08_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
