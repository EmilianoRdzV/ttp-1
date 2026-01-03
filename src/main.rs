/*
 * Copyright (c) 2024 Filippo Finke
 */

use crate::{
    algorithms::{
        kp::random::RandomKP,
        tsp::{
            brute_force::BruteForceTSP, lin_kernighan::LinKernighanTSP,
            nearest_insertion::NearestInsertionTSP, simulated_annealing::SimulatedAnnealingTSP,
            tabu_search::TabuSearchTSB, two_opt::TwoOpt,
        },
    },
    models::{instance::Instance, path::Path},
};
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;

mod algorithms;
mod models;

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
        .item("Brute force TSP")
        .item("Nearest insertion TSP")
        .item("Two opt TSP")
        .item("Simulated annealing TSP")
        .item("Tabu search TSP")
        .item("Lin-Kernighan TSP")
        .item("TTP Optimizador (Greedy + 2-Opt)")
        .default(0)
        .interact()
        .expect("Failed to select an algorithm");

    let path = Path::new(instance.node_coords.clone());
    println!("Initial path length: {}", path.length());
    let mut shortest_path: Option<Path> = None;
    let mut is_ttp_optimized = false;

    // Default TSP step for initial tour
    match algorithm_selection {
        0 => {
            println!("Brute force TSP");
            shortest_path = Some(BruteForceTSP::solve(&path));
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
        6 => {
            // TTP Optimizer flow
            // 1. Initial TSP (use 2-opt as fast baseline)
            println!("--- TTP Optimizador ---");
            println!("1. Generando ruta inicial (TSP 2-opt)...");
            shortest_path = Some(TwoOpt::solve(&path));
            is_ttp_optimized = true;
        }
        _ => println!("Invalid selection"),
    }

    if let Some(shortest_path_val) = shortest_path {
        println!("TSP Base path length: {}", shortest_path_val.length());

        let packing_plan: Vec<usize>;

        if is_ttp_optimized {
            println!("2. Generando Plan de Recolección (Greedy)...");
            // Use new GreedyKP
            packing_plan = crate::algorithms::kp::greedy::GreedyKP::solve(&instance);
        } else {
            // Fallback for classic TSP modes -> RandomKP as before
            packing_plan = RandomKP::solve(&shortest_path_val, &instance);
        }

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

        let mut solution_tour = tour.clone();

        if is_ttp_optimized {
            println!("3. Optimizando ruta TTP (Hill Climbing)...");
            solution_tour = crate::algorithms::ttp::hill_climbing::HillClimbingTTP::optimize(
                &instance,
                solution_tour,
                &packing_plan,
            );
        }

        let solution =
            crate::models::solution::Solution::evaluate(&instance, &solution_tour, &packing_plan);
        // println!("TTP Solution:\n{}", solution);

        // Find next available file name: [instance_name]_sol_[N].txt
        let base_name = instance.problem_name.clone();
        let mut counter = 1;
        let mut file_path = format!("{}_sol_{}.txt", base_name, counter);
        while std::path::Path::new(&file_path).exists() {
            counter += 1;
            file_path = format!("{}_sol_{}.txt", base_name, counter);
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
