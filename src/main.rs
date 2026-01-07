/*
 * Copyright (c) 2024 Filippo Finke
 */

use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use ttp::{
    algorithms::tsp::{
        lin_kernighan::LinKernighanTSP, nearest_insertion::NearestInsertionTSP,
        nearest_neighbor::NearestNeighborTSP, simulated_annealing::SimulatedAnnealingTSP,
        tabu_search::TabuSearchTSB, two_opt::TwoOpt,
    },
    models::{instance::Instance, path::Path},
};

fn main() {
    // List all the files in the instances folder
    let files = fs::read_dir("./instances")
        .expect("Failed to read instances folder")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    Some(file_name.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // Ask the user to select one file from the list
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an instance file")
        .default(0)
        .items(&files)
        .interact()
        .expect("Failed to select an instance file");

    // Get the selected file path
    let selected_file = &files[selection];

    println!("Selected file: {}", selected_file);

    // Load the selected instance
    let instance =
        Instance::load(&format!("./instances/{}", selected_file)).expect("Failed to load instance");

    println!("{}\n", instance);

    let algorithm_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an algorithm")
        .item("Nearest Neighbor (Fast Greedy - Best for 33k+)")
        .item("Nearest insertion TSP")
        .item("Two opt TSP")
        .item("Simulated annealing TSP")
        .item("Tabu search TSP")
        .item("Lin-Kernighan TSP")
        .default(0)
        .interact()
        .expect("Failed to select an algorithm");

    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let mut shortest_path: Option<Path> = None;
    // Always optimize for TTP
    let is_ttp_optimized = true;

    // Default TSP step for initial tour
    match algorithm_selection {
        0 => {
            println!("Nearest Neighbor TSP (Scalable)");
            shortest_path = Some(NearestNeighborTSP::solve(&path));
        }
        1 => {
            println!("Nearest insertion TSP");
            shortest_path = Some(NearestInsertionTSP::solve(&path));
        }
        2 => {
            println!("Two opt TSP");
            shortest_path = Some(TwoOpt::solve(&path));
        }
        3 => {
            println!("Simulated annealing TSP");
            shortest_path = Some(SimulatedAnnealingTSP::solve(&path));
        }
        4 => {
            println!("Tabu search TSP");
            shortest_path = Some(TabuSearchTSB::solve(&path));
        }
        5 => {
            println!("Lin-Kernighan TSP");
            shortest_path = Some(LinKernighanTSP::solve(&path));
        }
        _ => println!("Invalid selection"),
    }

    if let Some(shortest_path_val) = shortest_path {
        println!("TSP Base path length: {}", shortest_path_val.length());

        // Convert Path nodes (which are (id, x, y)) to tour (vec of 0-based indices)
        // instance.nodes IDs are 1-based usually.
        let mut tour: Vec<usize> = Vec::new();
        for node in &shortest_path_val.nodes {
            if node.0 > 0 {
                tour.push(node.0 - 1);
            } else {
                tour.push(0);
            }
        }

        // Always use GreedyKP for TTP
        println!("2. Generando Plan de Recolección (Greedy Cost-Aware)...");
        let packing_plan = ttp::algorithms::kp::greedy::GreedyKP::solve(&instance, &tour);

        let mut solution_tour = tour.clone();
        let final_packing_plan = packing_plan.clone();

        // CRITICAL FIX: TTP Simulation MUST start at Depot (Node 0/1).
        // Rotate solution_tour so it starts with 0 BEFORE optimization.
        // This ensures the optimizer works on the correct configuration.
        if let Some(pos_0) = solution_tour.iter().position(|&x| x == 0) {
            solution_tour.rotate_left(pos_0);
        }

        if is_ttp_optimized {
            println!("3. TTP Optimization Skipped (Manual Impover Mode). Saving base solution...");
            /*
            println!("3. Optimizando ruta TTP (Hill Climbing & Bit-Flip & ItemSwap)...");
            let (opt_tour, opt_packing) =
                ttp::algorithms::ttp::hill_climbing::HillClimbingTTP::optimize_full(
                    &instance,
                    solution_tour,
                    final_packing_plan,
                );
            solution_tour = opt_tour;
            final_packing_plan = opt_packing;
            */
        }

        // Rotation already done before.
        // Ensure it stays 0-started? The optimizer logic (next step) will guarantee it.

        let solution = ttp::models::solution::Solution::evaluate(
            &instance,
            &solution_tour,
            &final_packing_plan,
        );
        // println!("TTP Solution:\n{}", solution);

        // Find next available file name: [instance_name]_[nItems]_sol_[N].txt
        let base_name = instance.problem_name.clone();
        let n_items = instance.num_items;
        let mut counter = 1;
        let mut file_path = format!("{}_{}_sol_{}.txt", base_name, n_items, counter);
        while std::path::Path::new(&file_path).exists() {
            counter += 1;
            file_path = format!("{}_{}_sol_{}.txt", base_name, n_items, counter);
        }

        // Write result to file
        solution.write_result(&file_path);

        println!("Solution written to: {}", file_path);
        println!("--------------------------------------------------");
        println!("TTP Result:");
        println!("Objective (Ganancia Final): {:.4}", solution.ob);
        println!("Total Profit (Valor Total): {:.4}", solution.fp);
        println!("Total Time (Tiempo Total) : {:.4}", solution.ft);
        println!("--------------------------------------------------");
    } else {
        println!("Failed to find the shortest path");
    }
}
