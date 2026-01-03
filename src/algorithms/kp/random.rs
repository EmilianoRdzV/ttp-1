/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use rand::{thread_rng, Rng};

use crate::models::{instance::Instance, path::Path};

pub struct RandomKP;

/**
 * RandomKP is a simple algorithm that generates a random solution for the knapsack problem.
 */
impl RandomKP {
    pub fn solve(path: &Path, instance: &Instance) -> Vec<usize> {
        // Collect available items with their original 0-based index
        // Instance items are (id, profit, weight, node_id)
        // We assume we can map 1:1 if we just track the index in the instance.items vector
        // item 0 in instance.items -> index 0 for packing plan

        let mut available_items: Vec<(usize, f64, f64)> = vec![];
        for (idx, item) in instance.items.iter().enumerate() {
            // Check if the node is in the path
            // item.3 is the assigned node ID
            if path.has_node(item.3) {
                available_items.push((idx, item.1, item.2));
            }
        }

        let max_capacity = instance.capacity_of_knapsack;
        let mut current_weight: f64 = 0.0;
        let mut packing_plan = vec![0; instance.num_items];
        let mut _current_profit: f64 = 0.0; // Keep track just for debugging/logging if needed

        // Generate a random solution
        let mut rng = thread_rng();

        // Shuffle available items to pick randomly without remove overhead or repeated gen_range logic
        // Or just pick loops. The previous logic was: iteratively pick random from remaining.

        while !available_items.is_empty() {
            let index_in_available = rng.gen_range(0..available_items.len());
            let (original_idx, profit, weight) = available_items[index_in_available];

            // Remove from available so we don't pick again
            available_items.swap_remove(index_in_available);

            if current_weight + weight <= max_capacity {
                current_weight += weight;
                _current_profit += profit;

                // Mark as picked
                if original_idx < packing_plan.len() {
                    packing_plan[original_idx] = 1;
                }
            }
        }

        // println!(
        //    "RandomKP -> Weight: {}, Profit: {}",
        //    current_weight, _current_profit
        // );

        packing_plan
    }
}
