use std::fs;
use ttp::algorithms::ttp::hill_climbing::HillClimbingTTP;
use ttp::models::instance::Instance;
use ttp::models::solution::Solution;

fn main() {
    println!("--- TTP Improver (Warm Start) ---");

    // Hardcoded paths (User preference)
    // Adjust these to target the file you want to improve
    let instance_path = "instances/fnl4461_n4460_bounded-strongly-corr_01.ttp";
    let input_solution_path = "fnl4461-TTP_4460_sol_2.txt"; // The solution to improve

    // Generate output filename based on input
    let output_solution_path = format!(
        "{}_improved.txt",
        input_solution_path.trim_end_matches(".txt")
    );

    println!("Instance: {}", instance_path);
    println!("Input Solution: {}", input_solution_path);
    println!("Output Target: {}", output_solution_path);

    // 1. Load Instance
    let instance = Instance::load(instance_path).expect("Failed to load instance");

    // 2. Load Input Solution
    let content = fs::read_to_string(input_solution_path).expect("Failed to read solution file");
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        panic!("Solution file must have at least 2 lines (Tour, PackingPlan)");
    }

    // 3. Parse Tour [1, 2, ...] -> Vec<usize> (0-based)
    let tour_line = lines[0].trim();
    let tour_line = tour_line.trim_start_matches('[').trim_end_matches(']');
    let tour_1based: Vec<usize> = tour_line
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    // Convert to 0-based
    let mut tour_0based: Vec<usize> = tour_1based.iter().map(|&x| x - 1).collect();

    // Ensure it starts at 0 (Depot) for Optimization
    if let Some(pos_0) = tour_0based.iter().position(|&x| x == 0) {
        tour_0based.rotate_left(pos_0);
    }

    // 4. Parse Packing Plan [1, 2, ...] (Indices) -> Vec<usize> (Binary 0/1)
    let pack_line = lines[1].trim();
    let pack_line = pack_line.trim_start_matches('[').trim_end_matches(']');

    let picked_items_1based: Vec<usize> = if pack_line.is_empty() {
        Vec::new()
    } else {
        pack_line
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    };

    let mut packing_mask = vec![0; instance.num_items];
    for &idx in &picked_items_1based {
        if idx > 0 && idx <= instance.num_items {
            packing_mask[idx - 1] = 1;
        }
    }

    println!(
        "Loaded State -> Tour Len: {}, Items Picked: {}",
        tour_0based.len(),
        picked_items_1based.len()
    );

    // Initial Eval
    let initial_sol = Solution::evaluate(&instance, &tour_0based, &packing_mask);
    println!("Starting Objective: {:.4}", initial_sol.ob);

    // 5. Run Optimization
    println!("Starting Optimization (Hill Climbing + Swap + BitFlip + ItemSwap)...");
    let (new_tour, new_packing) =
        HillClimbingTTP::optimize_full(&instance, tour_0based, packing_mask);

    // 6. Evaluate and Save Result
    let final_sol = Solution::evaluate(&instance, &new_tour, &new_packing);
    println!("Final Objective: {:.4}", final_sol.ob);

    final_sol.write_result(&output_solution_path);
    println!("Saved improved solution to: {}", output_solution_path);
}
