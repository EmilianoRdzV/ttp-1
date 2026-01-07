use std::fs;
use ttp::models::instance::Instance;
use ttp::models::solution::Solution;

fn main() {
    // Hardcoded paths for verification
    let instance_path = "instances/fnl4461_n4460_bounded-strongly-corr_01.ttp";
    let solution_path = "fnl4461-TTP_4460_sol_2.txt";

    println!("--- TTP Verifier ---");
    println!("Instance: {}", instance_path);
    println!("Solution: {}", solution_path);

    let instance = Instance::load(instance_path).expect("Failed to load instance");
    println!(
        "Instance loaded: {} items, capacity {:.2}",
        instance.num_items, instance.capacity_of_knapsack
    );

    let content = fs::read_to_string(solution_path).expect("Failed to read solution file");
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        panic!("Solution file must have at least 2 lines (Tour, PackingPlan)");
    }

    // 3. Parse Tour (Line 1: [1, 2, 3...])
    let tour_line = lines[0].trim();
    let tour_line = tour_line.trim_start_matches('[').trim_end_matches(']');
    // Split by comma (handling potential spaces)
    let tour_1based: Vec<usize> = tour_line
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    // Convert tour to 0-based for internal evaluate
    // File has [1, 2, ...]. Internal wants [0, 1, ...].
    // Note: evaluate expects 0-based indices.
    let tour_0based: Vec<usize> = tour_1based.iter().map(|&x| x - 1).collect();

    // 4. Parse Packing Plan (Line 2: [10, 20...])
    let pack_line = lines[1].trim();
    let pack_line = pack_line.trim_start_matches('[').trim_end_matches(']');
    // If empty "[]", checking
    let picked_items_1based: Vec<usize> = if pack_line.is_empty() {
        Vec::new()
    } else {
        pack_line
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    };

    // Convert to binary packing plan (0/1) for internal evaluate
    let mut packing_mask = vec![0; instance.num_items];
    for &idx in &picked_items_1based {
        // idx is 1-based item index.
        if idx > 0 && idx <= instance.num_items {
            packing_mask[idx - 1] = 1;
        } else {
            println!("Warning: Picked item index {} out of range", idx);
        }
    }

    println!(
        "Parsed Solution: Tour Len {}, Picked Items {}",
        tour_0based.len(),
        picked_items_1based.len()
    );

    let solution = Solution::evaluate(&instance, &tour_0based, &packing_mask);

    println!("--------------------------------------------------");
    println!("VERIFICATION RESULTS:");
    println!("Objective GF: {:.4}", solution.ob);
    println!("Profit VT: {:.4}", solution.fp);
    println!("Time TT: {:.4}", solution.ft);
    println!("Weight End: {:.2}", solution.wend);
}
