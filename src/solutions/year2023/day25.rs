use std::collections::{HashMap, HashSet, VecDeque};

// Advent of Code 2023 - Day 25: Snowverload

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn from_str(input: &str) -> Self {
        let mut nodes = HashSet::new();
        let mut edges: HashMap<String, HashSet<String>> = HashMap::new();

        for line in input.lines() {
            let mut parts = line.splitn(2, ": ");
            let source = parts.next().unwrap().to_string();
            let targets: Vec<String> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            nodes.insert(source.clone());
            for target in targets {
                nodes.insert(target.clone());
                // Undirected graph: add edge in both directions
                edges
                    .entry(source.clone())
                    .or_default()
                    .insert(target.clone());
                edges
                    .entry(target.clone())
                    .or_default()
                    .insert(source.clone());
            }
        }

        Self { nodes, edges }
    }

    /// Find shortest path using BFS
    fn shortest_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(node) = queue.pop_front() {
            if node == end {
                // Reconstruct path
                let mut path = vec![end.to_string()];
                let mut current = end.to_string();
                while let Some(prev) = parent.get(&current) {
                    path.push(prev.clone());
                    current = prev.clone();
                }
                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = self.edges.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        parent.insert(neighbor.clone(), node.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        None
    }

    /// Count edge usage in shortest paths between node pairs
    /// This computes a simplified edge betweenness centrality
    fn count_edge_usage(&self, samples: usize) -> HashMap<(String, String), usize> {
        let mut edge_count: HashMap<(String, String), usize> = HashMap::new();
        let nodes: Vec<_> = self.nodes.iter().collect();

        // Sample paths between first 'samples' nodes
        for i in 0..samples.min(nodes.len()) {
            for j in (i + 1)..samples.min(nodes.len()) {
                if let Some(path) = self.shortest_path(nodes[i], nodes[j]) {
                    // Count each edge in the path
                    for window in path.windows(2) {
                        let mut edge = (window[0].clone(), window[1].clone());
                        // Normalize edge representation (smaller first)
                        if edge.0 > edge.1 {
                            edge = (edge.1, edge.0);
                        }
                        *edge_count.entry(edge).or_default() += 1;
                    }
                }
            }
        }

        edge_count
    }

    /// Remove an edge from the graph
    fn remove_edge(&mut self, a: &str, b: &str) {
        if let Some(neighbors) = self.edges.get_mut(a) {
            neighbors.remove(b);
        }
        if let Some(neighbors) = self.edges.get_mut(b) {
            neighbors.remove(a);
        }
    }

    /// Count nodes in connected component starting from given node using BFS
    fn count_component(&self, start: &str) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start.to_string());

        while let Some(node) = queue.pop_front() {
            if let Some(neighbors) = self.edges.get(node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        visited.len()
    }
}

/// Part 1: Find 3 edges to cut to split graph into 2 components
pub fn solution_2023_25_01(file_path: String) -> anyhow::Result<usize> {
    let mut graph = Graph::from_str(&std::fs::read_to_string(file_path)?);

    let edge_usage = graph.count_edge_usage(50);
    // ]\
    // Get top 3 edges by usage count
    let mut edges_by_usage: Vec<_> = edge_usage.iter().collect();
    edges_by_usage.sort_by_key(|(_, &count)| std::cmp::Reverse(count));

    // Remove the top 3 most-used edges
    for i in 0..3 {
        let ((a, b), _) = edges_by_usage[i];
        graph.remove_edge(a, b);
    }

    // Count the two components
    let start_node = graph.nodes.iter().next().unwrap();
    let component1_size = graph.count_component(start_node);
    let component2_size = graph.nodes.len() - component1_size;

    Ok(component1_size * component2_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_25_01() {
        let file_path: String = String::from("inputs/2023/day25e.txt");
        let result = solution_2023_25_01(file_path).unwrap();
        assert_eq!(result, 54);
    }

    #[test]
    #[ignore]
    fn output_2023_25_01() {
        let file_path: String = String::from("inputs/2023/day25.txt");
        let result = solution_2023_25_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
