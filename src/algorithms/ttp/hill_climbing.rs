/*
 * TTP Hill Climbing Optimizer (2-opt + Swap)
 */

use crate::models::instance::Instance;
use crate::models::solution::Solution;

pub struct HillClimbingTTP;

impl HillClimbingTTP {
    // Changing signature to return (Tour, PackingPlan)
    pub fn optimize_full(
        instance: &Instance,
        tour: Vec<usize>,
        packing_plan: Vec<usize>,
    ) -> (Vec<usize>, Vec<usize>) {
        let mut best_tour = tour.clone();
        let mut best_packing = packing_plan.clone();

        // Initial evaluation
        let best_sol = Solution::evaluate(instance, &best_tour, &best_packing);
        let mut best_obj = best_sol.ob;

        println!("  > Initial TTP Objective: {:.2}", best_obj);

        let mut improved = true;
        let mut cycle = 0;

        while improved {
            cycle += 1;
            improved = false;
            println!("  > Starting TTP Optimization Cycle #{}", cycle);

            // 2-Opt Phase
            let (tour_2opt, obj_2opt) =
                Self::run_2opt(instance, &best_tour, &best_packing, best_obj);
            if obj_2opt > best_obj {
                best_tour = tour_2opt;
                best_obj = obj_2opt;
                improved = true;
                println!(
                    "    [2-Opt] Improvement found! New Objective: {:.2}",
                    best_obj
                );
            }

            // Swap Phase
            let (tour_swap, obj_swap) =
                Self::run_swap(instance, &best_tour, &best_packing, best_obj);
            if obj_swap > best_obj {
                best_tour = tour_swap;
                best_obj = obj_swap;
                improved = true;
                println!(
                    "    [Swap]  Improvement found! New Objective: {:.2}",
                    best_obj
                );
            }

            // Bit-Flip Phase (Packing Optimization)
            let (pack_flip, obj_flip) =
                Self::run_bit_flip(instance, &best_tour, &best_packing, best_obj);
            if obj_flip > best_obj {
                best_packing = pack_flip;
                best_obj = obj_flip;
                improved = true;
                println!(
                    "    [BitFlip] Improvement found! New Objective: {:.2}",
                    best_obj
                );
            }

            // Item Swap Phase
            let (pack_swap, obj_swap) =
                Self::run_item_swap(instance, &best_tour, &best_packing, best_obj);
            if obj_swap > best_obj {
                best_packing = pack_swap;
                best_obj = obj_swap;
                improved = true;
                println!(
                    "    [ItemSwap] Improvement found! New Objective: {:.2}",
                    best_obj
                );
            }
        }

        (best_tour, best_packing)
    }

    fn run_2opt(
        instance: &Instance,
        current_tour: &Vec<usize>,
        packing_plan: &Vec<usize>,
        current_best_obj: f64,
    ) -> (Vec<usize>, f64) {
        let mut best_tour = current_tour.clone();
        let mut best_obj = current_best_obj;
        let mut improved = true;

        // As standard steepest ascent or first improvement?
        // User code does: loop inner 2-opt, if improved, update and break inner loops to restart?
        // "if novaGanancia > mejorGanancia: ruta = novaRuta ... break"
        // Yes, First Improvement strategy.

        while improved {
            improved = false;
            let len = best_tour.len();
            // 2-opt usually 1 to len-1 (leaving start/end fixed if cycle, or strict cycle logic)
            // Our tour is 0-based indices.
            // If tour is a full cycle representation like [0, 1, 2, ..., 0], we swap inner.
            // If tour is [0, 1, 2], we treat it as cycle.
            // Let's assume best_tour is just the cities. standard 2-opt loops i from 0, j from i+2?
            // User code: for i in 1..len(ruta)-1 ...

            for i in 1..len - 1 {
                for j in i + 1..len {
                    // Reverse segment [i..j] ?? Or i+1..j?
                    // Standard 2-opt: reverse between i and j.
                    // User code: nuevaRuta[i:j+1] = reversed(...)
                    // Let's emulate simple reversal.

                    let mut new_tour = best_tour.clone();
                    // Reverse section
                    if j < new_tour.len() {
                        new_tour[i..=j].reverse();
                    } else {
                        continue;
                    }

                    let sol = Solution::evaluate(instance, &new_tour, packing_plan);
                    if sol.ob > best_obj {
                        best_obj = sol.ob;
                        best_tour = new_tour;
                        improved = true;
                        break; // Restart search (First Improvement)
                    }
                }
                if improved {
                    break;
                }
            }
        }
        (best_tour, best_obj)
    }

    fn run_swap(
        instance: &Instance,
        current_tour: &Vec<usize>,
        packing_plan: &Vec<usize>,
        current_best_obj: f64,
    ) -> (Vec<usize>, f64) {
        let mut best_tour = current_tour.clone();
        let mut best_obj = current_best_obj;
        let mut improved = true;

        while improved {
            improved = false;
            let len = best_tour.len();

            for i in 1..len - 1 {
                for j in i + 1..len {
                    let mut new_tour = best_tour.clone();
                    new_tour.swap(i, j);

                    let sol = Solution::evaluate(instance, &new_tour, packing_plan);
                    if sol.ob > best_obj {
                        best_obj = sol.ob;
                        best_tour = new_tour;
                        improved = true;
                        // println!("      Swap found: {:.2}", best_obj);
                        break;
                    }
                }
                if improved {
                    break;
                }
            }
        }
        (best_tour, best_obj)
    }

    fn run_bit_flip(
        instance: &Instance,
        tour: &Vec<usize>,
        current_packing: &Vec<usize>,
        current_best_obj: f64,
    ) -> (Vec<usize>, f64) {
        let mut best_packing = current_packing.clone();
        let mut best_obj = current_best_obj;
        let mut improved = true;

        // Calculate current weight to ensure we don't violate capacity when flipping 0->1
        // Actually Solution::evaluate handles logic, but doesn't punish capacity violation?
        // Wait, Solution::evaluate logic (based on previous view) doesn't explicitly check capacity limit?
        // Actually, TTP usually enforces capacity.
        // Let's check if evaluate handles it (or we should enforce it).
        // Assuming evaluate handles it (or we should enforce it).
        // Standard BitFlip: iterate all items.
        // If 0->1: Check capacity. If fits, eval.
        // If 1->0: Always allowed. eval.

        // Let's compute current weight first.
        let mut current_weight: f64 = 0.0;
        for (i, &val) in best_packing.iter().enumerate() {
            if val == 1 {
                current_weight += instance.items[i].2;
            }
        }
        let capacity = instance.capacity_of_knapsack;

        while improved {
            improved = false;

            for i in 0..instance.num_items {
                let item_weight = instance.items[i].2;
                let original_val = best_packing[i];

                // Flip
                if original_val == 0 {
                    // Try to pick
                    if current_weight + item_weight <= capacity {
                        best_packing[i] = 1;
                        let sol = Solution::evaluate(instance, tour, &best_packing);
                        if sol.ob > best_obj {
                            best_obj = sol.ob;
                            current_weight += item_weight;
                            improved = true;
                            // println!("      BitFlip (Pick) found: {:.2}", best_obj);
                        } else {
                            // Revert
                            best_packing[i] = 0;
                        }
                    }
                } else {
                    // Try to drop
                    best_packing[i] = 0;
                    let sol = Solution::evaluate(instance, tour, &best_packing);
                    if sol.ob > best_obj {
                        best_obj = sol.ob;
                        current_weight -= item_weight;
                        improved = true;
                        // println!("      BitFlip (Drop) found: {:.2}", best_obj);
                    } else {
                        // Revert
                        best_packing[i] = 1;
                    }
                }
            }
        }
        (best_packing, best_obj)
    }

    fn run_item_swap(
        instance: &Instance,
        tour: &Vec<usize>,
        current_packing: &Vec<usize>,
        current_best_obj: f64,
    ) -> (Vec<usize>, f64) {
        let mut best_packing = current_packing.clone();
        let mut best_obj = current_best_obj;
        let mut improved = true;

        // Calculate current weight
        let mut current_weight: f64 = 0.0;
        for (i, &val) in best_packing.iter().enumerate() {
            if val == 1 {
                current_weight += instance.items[i].2;
            }
        }
        let capacity = instance.capacity_of_knapsack;

        // Collect indices of picked vs unpicked for efficiency
        let mut picked_indices = Vec::new();
        let mut unpicked_indices = Vec::new();
        for (i, &val) in best_packing.iter().enumerate() {
            if val == 1 {
                picked_indices.push(i);
            } else {
                unpicked_indices.push(i);
            }
        }

        while improved {
            improved = false;

            // Try to swap every picked item with every unpicked item
            // This is O(M * N) which can be heavy, but necessary for local search.
            // We use standard "First Improvement".
            'outer: for &pidx in &picked_indices {
                // pidx: picked item index
                for &uidx in &unpicked_indices {
                    // uidx: unpicked item index
                    let w_out = instance.items[pidx].2;
                    let w_in = instance.items[uidx].2;

                    // Check if swap is feasible capacity-wise
                    if current_weight - w_out + w_in <= capacity {
                        // Perform Swap
                        best_packing[pidx] = 0;
                        best_packing[uidx] = 1;

                        let sol = Solution::evaluate(instance, tour, &best_packing);
                        if sol.ob > best_obj {
                            best_obj = sol.ob;
                            current_weight = current_weight - w_out + w_in;

                            // Update our local lists (expensive, but simplistic for now or just rebuild)
                            // Rebuilding lists is slow. But since we 'break' on first improvement,
                            // we just need to restart the loop logic.
                            // We need to reflect the change in lists?
                            // Easier to just break, allowing the outer 'while improved' to rebuild lists if needed.
                            // But building lists inside loop is better.

                            improved = true;
                            break 'outer;
                        } else {
                            // Revert
                            best_packing[pidx] = 1;
                            best_packing[uidx] = 0;
                        }
                    }
                }
            }

            // If improved, we need to refresh indices because best_packing changed
            if improved {
                picked_indices.clear();
                unpicked_indices.clear();
                for (i, &val) in best_packing.iter().enumerate() {
                    if val == 1 {
                        picked_indices.push(i);
                    } else {
                        unpicked_indices.push(i);
                    }
                }
            }
        }
        (best_packing, best_obj)
    }
}
