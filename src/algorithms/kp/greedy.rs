/*
 * Greedy Knapsack Heuristic
 */

use crate::models::instance::Instance;
// We don't necessarily need Path for this specific greedy, but we keep the signature consistent if we want
// or just pass instance.

pub struct GreedyKP;

impl GreedyKP {
    pub fn solve(instance: &Instance) -> Vec<usize> {
        let capacity = instance.capacity_of_knapsack;
        let mut current_weight = 0.0;
        let mut packing_plan = vec![0; instance.num_items];

        // Create a list of items with their efficiency: (index, weight, profit, efficiency)
        // instance.items: (index, profit, weight, assigned_node)
        // We want to sort by Profit/Weight descending.

        struct ItemData {
            idx: usize,
            weight: f64,
            _profit: f64,
            efficiency: f64,
        }

        let mut items: Vec<ItemData> = instance
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let profit = item.1;
                let weight = item.2;
                let efficiency = if weight > 0.0 {
                    profit / weight
                } else {
                    f64::INFINITY
                };
                ItemData {
                    idx: i,
                    weight,
                    _profit: profit,
                    efficiency,
                }
            })
            .collect();

        // Sort descending by efficiency
        items.sort_by(|a, b| b.efficiency.partial_cmp(&a.efficiency).unwrap());

        for item in items {
            if current_weight + item.weight <= capacity {
                current_weight += item.weight;
                packing_plan[item.idx] = 1;
            }
        }

        packing_plan
    }
}
