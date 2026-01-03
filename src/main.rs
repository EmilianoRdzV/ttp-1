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
    models::{instance::Instance, path::Path, solution::Solution},
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
    let mode_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select execution mode")
        .item("Full TTP (TSP + KP)")
        .item("TSP Only")
        .default(0)
        .interact()
        .expect("Failed to select execution mode");

    // Load the selected instance
    let load_items = mode_selection == 0;
    let instance = Instance::load(&format!("./instances/{}", selected_file), load_items)
        .expect("Failed to load instance");

    let algorithm_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an algorithm")
        .item("Brute force TSP")
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
        _ => println!("Invalid selection"),
    }

    if let Some(shortest_path) = shortest_path {
        println!("Shortest path length: {}", shortest_path.length());

        if mode_selection == 0 {
            let solution = RandomKP::solve(&shortest_path, &instance);

            let output_filename = format!("{}_solution.txt", selected_file);
            solution.write_result(&output_filename);
            println!("Solution saved to {}", output_filename);
        }

        let route_ids: Vec<i32> = shortest_path.nodes.iter().map(|(id, _, _)| *id).collect();
        let tsp_output_content = format!("{:?}", route_ids);
        let tsp_output_filename = format!("{}_tsp_route.txt", selected_file);
        fs::write(&tsp_output_filename, tsp_output_content)
            .expect("Failed to write TSP route file");
        println!("TSP route saved to {}", tsp_output_filename);
    } else {
        println!("Failed to find the shortest path");
    }
}
