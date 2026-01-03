/*
 * Copyright (c) 2024 Filippo Finke and Walter Sostene Losa
 */

use rand::{thread_rng, Rng};
use std::time::Instant;

use crate::models::{instance::Instance, path::Path, solution::Solution};

pub struct RandomKP;

/**
 * RandomKP is a simple algorithm that generates a random solution for the knapsack problem.
 */
impl RandomKP {
    pub fn solve(path: &Path, instance: &Instance) -> Solution {
        let start_time = Instant::now();
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
        let mut selected_items_map: std::collections::HashMap<i32, Vec<(f64, f64)>> =
            std::collections::HashMap::new();

        // Generate a random solution
        // Randomly select items to put in the knapsack
        let mut rng = thread_rng();
        for _ in 0..items.len() {
            // Randomly select an item
            // Get random item from vector and remove it
            let index = rng.gen_range(0..=(items.len() - 1));

            // FIXED: Correctly destructure (id, profit, weight)
            let (id, profit, weight) = items[index];
            items.remove(index);

            // Check if adding the item exceeds the capacity
            if current_weight + weight <= max_capacity {
                // Add the item to the solution
                current_weight += weight;
                current_profit += profit;
                selected_items.push(id);

                // Track item based on node for time calculation
                if let Some(original_item) = instance.items.iter().find(|&&x| x.0 == id) {
                    let node_id = original_item.3 as i32;
                    selected_items_map
                        .entry(node_id)
                        .or_insert_with(Vec::new)
                        .push((weight, profit));
                }
            }
        }

        // --- Calculate TTP Objective function (Time and Renting Cost) ---

        let max_speed = instance.max_speed;
        let min_speed = instance.min_speed;
        let capacity = instance.capacity_of_knapsack;
        let renting_ratio = instance.renting_ratio;

        let mut current_knapsack_weight_calc = 0.0;
        let mut total_time = 0.0;

        let path_nodes = &path.nodes;
        // Check start node
        let start_node_id = path_nodes[0].0;
        if let Some(items_at_node) = selected_items_map.get(&start_node_id) {
            for (w, _) in items_at_node {
                current_knapsack_weight_calc += w;
            }
        }

        for i in 0..(path_nodes.len() - 1) {
            let (_, x1, y1) = path_nodes[i];
            let (_, x2, y2) = path_nodes[i + 1];

            let dx = (x2 - x1) as f64;
            let dy = (y2 - y1) as f64;
            let distance = (dx * dx + dy * dy).sqrt();

            let mut velocity = max_speed;
            if capacity > 0.0 {
                velocity =
                    max_speed - current_knapsack_weight_calc * (max_speed - min_speed) / capacity;
            }
            if velocity < min_speed {
                velocity = min_speed;
            }

            let time = distance / velocity;
            total_time += time;

            let next_node_id = path_nodes[i + 1].0;
            if let Some(items_at_node) = selected_items_map.get(&next_node_id) {
                for (w, _) in items_at_node {
                    current_knapsack_weight_calc += w;
                }
            }
        }

        // Return trip for TSP
        let (_, x1, y1) = path_nodes[path_nodes.len() - 1];
        let (_, x2, y2) = path_nodes[0];
        let dx = (x2 - x1) as f64;
        let dy = (y2 - y1) as f64;
        let distance = (dx * dx + dy * dy).sqrt();

        let mut velocity = max_speed;
        if capacity > 0.0 {
            velocity =
                max_speed - current_knapsack_weight_calc * (max_speed - min_speed) / capacity;
        }
        let time = distance / velocity;
        total_time += time;

        let final_score = current_profit - (total_time * renting_ratio);
        let duration = start_time.elapsed();

        println!(
            "Total Profit: {:.2} \nTotal Time: {:.2} \nRenting Cost: {:.2} \nFinal TTP Score: {:.2} \n(Capacity: {}/{})",
            current_profit, total_time, total_time * renting_ratio, final_score, current_weight, max_capacity
        );

        selected_items.sort();
        let tour: Vec<usize> = path.nodes.iter().map(|(id, _, _)| *id as usize).collect();

        Solution {
            tsp_tour: tour,
            packing_plan: selected_items,
            fp: current_profit,
            ft: total_time,
            ftraw: total_time, // Assuming raw time is same as calculated time for now
            ob: final_score,
            wend: current_weight,
            wend_used: current_weight,
            computation_time: duration.as_secs_f64(),
        }
    }
}
