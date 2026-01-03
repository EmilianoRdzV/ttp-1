/*
 * TTP Hill Climbing Optimizer (2-opt + Swap)
 */

use crate::models::instance::Instance;
use crate::models::solution::Solution;

pub struct HillClimbingTTP;

impl HillClimbingTTP {
    pub fn optimize(
        instance: &Instance,
        tour: Vec<usize>,
        packing_plan: &Vec<usize>,
    ) -> Vec<usize> {
        let mut best_tour = tour.clone();

        // Initial evaluation
        let best_sol = Solution::evaluate(instance, &best_tour, packing_plan);
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
                Self::run_2opt(instance, &best_tour, packing_plan, best_obj);
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
                Self::run_swap(instance, &best_tour, packing_plan, best_obj);
            if obj_swap > best_obj {
                best_tour = tour_swap;
                best_obj = obj_swap;
                improved = true;
                println!(
                    "    [Swap]  Improvement found! New Objective: {:.2}",
                    best_obj
                );
            }

            // Check improvement delta if needed, or loop until local optimum (strict increase above)
        }

        best_tour
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

            for i in 0..len - 1 {
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

            for i in 0..len - 1 {
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
}
