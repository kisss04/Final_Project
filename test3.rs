#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_bfs() {
        // Sample adjacency list for testing
        let adjacency_list = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 3],
            vec![1, 2, 4],
            vec![3],
        ];

        // Perform BFS from node 0
        let distances = bfs(&adjacency_list, 0);

        // Verify the distances
        assert_eq!(distances, vec![Some(0), Some(1), Some(1), Some(2), None]);
    }

    #[test]
    fn test_analysis() {
        // Read the dataset
        let contents = fs::read_to_string("email-Eu-core.txt").expect("Failed to read file");

        // Create data structures to store the graph
        let mut adjacency_list: Vec<Vec<usize>> = Vec::new();
        let mut num_outlinks: Vec<usize> = Vec::new();

        // Process each line in the dataset
        for line in contents.lines() {
            if !line.starts_with('#') {
                let parts: Vec<usize> = line
                    .split_whitespace()
                    .map(|s| s.parse().expect("Invalid node ID"))
                    .collect();

                let source_node = parts[0];
                let target_node = parts[1];

                // Update the adjacency list and outlink count vectors
                let max_node_id = source_node.max(target_node);
                if max_node_id >= adjacency_list.len() {
                    adjacency_list.resize(max_node_id + 1, Vec::new());
                    num_outlinks.resize(max_node_id + 1, 0);
                }

                adjacency_list[source_node].push(target_node);
                num_outlinks[source_node] += 1;
            }
        }

        // Calculate the number of nodes and edges
        let num_nodes = adjacency_list.len();
        let num_edges: usize = num_outlinks.iter().sum();

        // Calculate the average outdegree
        let average_outdegree = num_edges as f32 / num_nodes as f32;

        // Calculate the average distance between pairs of vertices
        let mut total_distance = 0;
        let mut pair_count = 0;

        for start_node in 0..num_nodes {
            let distances = bfs(&adjacency_list, start_node);

            for (node, distance) in distances.iter().enumerate() {
                if start_node != node {
                    if let Some(dist) = distance {
                        total_distance += dist;
                        pair_count += 1;
                    }
                }
            }
        }

        let average_distance = total_distance as f32 / pair_count as f32;

        // Verify the analysis results
        assert_eq!(num_nodes, 1005);
        assert_eq!(num_edges, 25571);
        assert_eq!(average_outdegree, 25.46, epsilon = 0.01);
        assert_eq!(average_distance, 3.691, epsilon = 0.01);
    }
}

