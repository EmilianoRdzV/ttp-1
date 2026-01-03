/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use std::collections::HashSet;

use crate::models::path::Path;

pub struct TabuSearchTSB;

/**
 * Tabu search algorithm for the TSP problem.
 * Generate neighbors of the current path and select the best one.
 * Keep track of the best solution found so far and avoid visiting the same path twice.
 */

impl TabuSearchTSB {
    pub fn solve(path: &Path) -> Path {
        let mut best_solution = path.clone();
        let mut current_solution = path.clone();
        let mut tabu_list: HashSet<Vec<usize>> = HashSet::new();

        let max_iterations = 100;
        let tabu_tenure = 30;
        let mut iterations = 0;

        while iterations < max_iterations {
            let neighbors = TabuSearchTSB::generate_neighbors(&current_solution);
            let mut best_neighbor = None;
            let mut best_neighbor_length = f64::INFINITY;

            for neighbor in neighbors {
                let neighbor_length = neighbor.length();
                let neighbor_signature = neighbor
                    .nodes
                    .iter()
                    .map(|(i, _, _)| *i)
                    .collect::<Vec<usize>>();
                if neighbor_length < best_neighbor_length
                    && !tabu_list.contains(&neighbor_signature)
                {
                    best_neighbor = Some(neighbor.clone());
                    best_neighbor_length = neighbor_length;
                }
            }

            if let Some(neighbor) = best_neighbor {
                current_solution = neighbor.clone();
                if neighbor.length() < best_solution.length() {
                    best_solution = neighbor.clone();
                }
                let neighbor_signature = neighbor
                    .nodes
                    .iter()
                    .map(|(i, _, _)| *i)
                    .collect::<Vec<usize>>();
                tabu_list.insert(neighbor_signature);
                if tabu_list.len() > tabu_tenure {
                    // This logic was flawed as HashSet iteration order is arbitrary.
                    // For a simple fix, we'll just ignore the FIFO removal or implement a VecDeque if needed.
                    // But to match previous logic (arbitrary removal), we can keep it simple or fix it.
                    // The previous code: tabu_list.iter().next().unwrap().clone(); removed an arbitrary element.
                    // We will do the same for now to maintain behavior, though it acts more like a random eviction.
                    let oldest_tabu = tabu_list.iter().next().unwrap().clone();
                    tabu_list.remove(&oldest_tabu);
                }
            }

            iterations += 1;
        }

        best_solution
    }

    fn generate_neighbors(path: &Path) -> Vec<Path> {
        let mut neighbors = Vec::new();
        let nodes_count = path.nodes.len();
        for i in 1..nodes_count {
            for j in (i + 1)..nodes_count {
                let mut new_nodes = path.nodes.clone();
                new_nodes.swap(i, j);
                neighbors.push(Path::new(new_nodes));
            }
        }
        neighbors
    }
}
