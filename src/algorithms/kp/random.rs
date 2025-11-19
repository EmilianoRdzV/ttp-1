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
    pub fn solve(path: &Path, instance: &Instance) -> (f64, Vec<usize>) {
        let mut items: Vec<(usize, f64, f64)> = vec![];
        for item in instance.items.iter() {
            // Check if the node is in the path
            if path.has_node(item.3 as i32) {
                items.push((item.0, item.1, item.2));
            }
        }

        let max_capacity = instance.capacity_of_knapsack;
        let mut current_weight: f64 = 0.0;
        let mut current_profit: f64 = 0.0;
        let mut selected_items: Vec<usize> = Vec::new();

        // Generate a random solution
        // Randomly select items to put in the knapsack
        let mut rng = thread_rng();
        for _ in 0..items.len() {
            // Randomly select an item
            // Get random item from vector and remove it
            let index = rng.gen_range(0..=(items.len() - 1));

            let (id, weight, profit) = items[index];
            items.remove(index);

            // Check if adding the item exceeds the capacity
            if current_weight + weight <= max_capacity {
                // Add the item to the solution
                current_weight += weight;
                current_profit += profit;
                selected_items.push(id);
            }
        }

        println!(
            "Current capacity: {} Current profit: {} Max capacity: {}",
            current_weight, current_profit, max_capacity
        );

        (current_profit, selected_items)
    }
}
