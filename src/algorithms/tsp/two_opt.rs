/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use crate::models::path::Path;

pub struct TwoOpt;

/**
* Swap two nodes in the path and check if the new path is better. Do it for all the nodes in the path.
*/
impl TwoOpt {
    pub fn solve(path: &Path) -> Path {
        let mut best_path = path.clone();

        // Continue until the path can't be improved.
        // Run exactly ONE pass (Single Cycle) as requested for speed on large instances.
        // while improved {

        // Cycle through all the nodes in the path and try to swap them.
        for i in 0..best_path.nodes.len() - 1 {
            for j in i + 1..best_path.nodes.len() {
                let mut new_path = best_path.clone();

                // Swap the nodes.
                new_path.nodes[i..=j].reverse();

                // If the new path is better, update the best path.
                if new_path.length() < best_path.length() {
                    best_path = new_path;
                }
            }
        }
        // }
        best_path
    }
}
