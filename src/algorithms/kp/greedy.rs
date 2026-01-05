/*
 * Greedy Knapsack Heuristic
 */

use crate::models::instance::Instance;
// We don't necessarily need Path for this specific greedy, but we keep the signature consistent if we want
// or just pass instance.

pub struct GreedyKP;

impl GreedyKP {
    pub fn solve(instance: &Instance, _tour: &Vec<usize>) -> Vec<usize> {
        let capacity = instance.capacity_of_knapsack;
        let mut current_weight = 0.0;
        let mut packing_plan = vec![0; instance.num_items];

        // Simple Greedy: Profit / Weight
        // We ignore the tour for the initial plan (as per user's Python script success).

        struct ItemData {
            idx: usize,
            weight: f64,
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
                    efficiency,
                }
            })
            .collect();

        // Sort descending by EFFICIENCY
        items.sort_by(|a, b| b.efficiency.partial_cmp(&a.efficiency).unwrap());

        for item in items {
            // Fill as much as possible
            if current_weight + item.weight <= capacity {
                current_weight += item.weight;
                packing_plan[item.idx] = 1;
            }
        }

        let picked_count = packing_plan.iter().filter(|&&x| x == 1).count();
        println!("DEBUG: GreedyKP finished.");
        println!(
            "DEBUG: Capacity: {:.2}, Used Weight: {:.2} ({:.2}%)",
            capacity,
            current_weight,
            (current_weight / capacity) * 100.0
        );
        println!(
            "DEBUG: Items picked: {} / {}",
            picked_count, instance.num_items
        );

        packing_plan
    }
}
