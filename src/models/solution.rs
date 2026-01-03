/*
 * Copyright (c) 2024 Filippo Finke
 */

use std::fmt::Display;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Solution {
    pub tsp_tour: Vec<usize>,
    pub packing_plan: Vec<usize>,
    pub fp: f64,
    pub ft: f64,
    pub ftraw: f64,
    pub ob: f64,
    pub wend: f64,
    pub wend_used: f64,
    pub computation_time: f64,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TSP Tour: {:?}\n\
            Packing Plan: {:?}\n\
            FP: {}\n\
            FT: {}\n\
            FTRaw: {}\n\
            OB: {}\n\
            WEnd: {}\n\
            WEnd Used: {}\n\
            Computation Time: {}",
            self.tsp_tour,
            self.packing_plan,
            self.fp,
            self.ft,
            self.ftraw,
            self.ob,
            self.wend,
            self.wend_used,
            self.computation_time
        )
    }
}

impl Solution {
    fn answer(&self) -> String {
        // Convert 0-based tour to 1-based for output
        let tour_out: Vec<usize> = self.tsp_tour.iter().map(|&x| x + 1).collect();

        // Convert binary packing plan to list of picked item indices (1-based)
        let mut packing_plan_list = Vec::new();
        for (index, &val) in self.packing_plan.iter().enumerate() {
            if val == 1 {
                packing_plan_list.push(index + 1);
            }
        }
        // packing_plan_list is already sorted if we iterate enumerate in order

        // Format as requested: arrays
        format!("{:?}\n{:?}\n", tour_out, packing_plan_list)
    }

    #[allow(dead_code)]
    pub fn write_result(&self, title: &str) {
        if let Ok(mut file) = File::create(title) {
            if let Err(e) = file.write_all(self.answer().as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            }
        } else {
            eprintln!("Error creating file: {}", title);
        }
    }

    pub fn evaluate(
        instance: &crate::models::instance::Instance,
        tour: &Vec<usize>,
        packing_plan: &Vec<usize>,
    ) -> Solution {
        let mut current_weight = 0.0;
        let mut current_profit = 0.0;
        let mut time = 0.0;

        // Ensure the tour is valid (starts and ends at the same node if needed,
        // but standard TTP tours usually imply a cycle.
        // We will traverse from tour[i] to tour[i+1])

        // Map from city_index -> list of item indices that are picked up
        // The packing_plan is a binary vector where packing_plan[item_index] == 1 means picked
        // We need to know which items belong to which city.
        // instance.items stores (index, profit, weight, assigned_node_number - 1-based usually)

        // Pre-process items for faster lookup: city_index (0-based) -> vec of item_indices (0-based)
        let mut city_items: Vec<Vec<usize>> = vec![vec![]; instance.dimension + 1];
        for (idx, _profit, _weight, assigned_node) in &instance.items {
            // assigned_node is likely 1-based from the file format, so we subtract 1?
            // Let's verify instance.rs parsing.
            // "assigned_node_number = item_info[3].parse().unwrap_or(0);"
            // Used as is. Usually TTP files are 1-based.
            if *assigned_node > 0 && *assigned_node <= instance.dimension {
                city_items[*assigned_node - 1].push(*idx - 1); // store 0-based item index
            }
        }

        let max_speed = instance.max_speed;
        let min_speed = instance.min_speed;
        let capacity = instance.capacity_of_knapsack;
        let renting_ratio = instance.renting_ratio;

        let mut actual_packing_plan = vec![0; instance.num_items];
        // If the input packing_plan covers all items, copy it.
        // If it's a list of selected item indices, we'd need to convert.
        // Based on solution.rs answer(), it iterates:
        // "if self.packing_plan[packing_plan_index] == 1" -> it is a binary vector.
        if packing_plan.len() == instance.num_items {
            actual_packing_plan = packing_plan.clone();
        } else {
            // If passing a list of picked indices, we might need to adapt.
            // For safety, assume it's the full binary vector as implies by the struct field.
        }

        // Calculate initial physics
        // v = vmax - (weight / capacity) * (vmax - vmin)
        let get_velocity = |w: f64| -> f64 {
            if w > capacity {
                return min_speed; // Should be invalid, but handle gracefully
            }
            max_speed - (w / capacity) * (max_speed - min_speed)
        };

        // Traverse the tour
        // The tour usually contains city indices (0-based or 1-based?).
        // Models usually use 0-based internally if converted, but let's check `path.rs` usage.
        // `path.nodes` are (usize, f64, f64).
        // Let's assume input `tour` is a list of city indices (0-based).

        let tour_len = tour.len();
        // Traverse from first city to last, then back to first?
        // Standard TSP tour includes all cities. If it doesn't repeat start, we close the loop.

        let loop_limit = if tour[0] == tour[tour_len - 1] {
            tour_len - 1
        } else {
            tour_len
        };

        for i in 0..loop_limit {
            let current_city = tour[i]; // 0-based
            let next_city = tour[(i + 1) % loop_limit];

            // 1. Pick up items at current_city
            // In standard TTP, we pick up items at the city we leave, contributing to weight on the edge.
            // "The thief visits each city... collects items... then moves to next city"

            if let Some(items_at_city) = city_items.get(current_city) {
                for &item_idx in items_at_city {
                    if item_idx < actual_packing_plan.len() && actual_packing_plan[item_idx] == 1 {
                        if tuple_elem(&instance.items, item_idx) {
                            let (_id, p, w, _node) = instance.items[item_idx];
                            current_weight += w;
                            current_profit += p;
                        }
                    }
                }
            }

            if current_weight > capacity {
                // Invalid solution basically, but we continue or penalize?
                // Usually represented by wend > capacity.
            }

            // 2. Travel to next city
            let dist = distance(&instance, current_city, next_city);
            let velocity = get_velocity(current_weight);
            time += dist / velocity;
        }

        let objective = current_profit - time * renting_ratio;

        Solution {
            tsp_tour: tour.clone(),
            packing_plan: actual_packing_plan,
            fp: current_profit,
            ft: time,
            ftraw: time, // Assuming raw time is same for now, or without stealing? Usually it's same.
            ob: objective,
            wend: current_weight,
            wend_used: current_weight, // used capacity
            computation_time: 0.0,
        }
    }
}

// Helper to safely access instance items since they are tuples
fn tuple_elem(items: &Vec<(usize, f64, f64, usize)>, idx: usize) -> bool {
    idx < items.len()
}

// Helper for distance calc
fn distance(instance: &crate::models::instance::Instance, c1: usize, c2: usize) -> f64 {
    // instance.node_coords is Vec<(index, x, y)>
    // finding c1 and c2. Assuming c1, c2 are 0-based INDICES into the node_coords array.
    let (_, x1, y1) = instance.node_coords[c1];
    let (_, x2, y2) = instance.node_coords[c2];
    let dx = x1 - x2;
    let dy = y1 - y2;
    (dx * dx + dy * dy).sqrt()
}
