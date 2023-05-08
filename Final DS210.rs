use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
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

    // Print the analysis results
    println!("Number of Nodes: {}", num_nodes);
    println!("Number of Edges: {}", num_edges);
    println!("Average Outdegree: {:.2}", average_outdegree);

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

    println!("Average Distance of Vertices: {:.2}", average_distance);
// BFS selected node 
let start_node = 0;
let distances = bfs(&adjacency_list, start_node);

// Print the shortest path distances from the start node
for (node, distance) in distances.iter().enumerate() {
    println!("Shortest Path Distance from Node {} to Node {}: {:?}", start_node, node, distance);
}

}


// BFS starting from the given node
fn bfs(adjacency_list: &[Vec<usize>], start_node: usize) -> Vec<Option<usize>> {
    let mut distances: Vec<Option<usize>> = vec![None; adjacency_list.len()];
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();

    // Enqueue the start node and mark it as visited with a distance of 0
    queue.push_back(start_node);
    visited.insert(start_node);
    distances[start_node] = Some(0);

    // Perform BFS
    while let Some(node) = queue.pop_front() {
        for &neighbor in &adjacency_list[node] {
            if !visited.contains(&neighbor) {
                // Enqueues, marks visited, updates distance
                queue.push_back(neighbor);
                visited.insert(neighbor);
                distances[neighbor] = Some(distances[node].unwrap() + 1);
            }
        }
    }

    distances
}
