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
        // Simple output format matching previous main.rs behavior:
        // Line 1: TSP Tour (list of node IDs)
        // Line 2: Packed Items (list of item IDs)
        // We ensure tsp_tour uses 1-based indexing if that's what's expected,
        // but main.rs previously successfully used 0-based or whatever was in the vector.
        // The previous main.rs output was: format!("{:?}\n{:?}", route_ids, items)
        // So we will replicate that but using fields.
        format!("{:?}\n{:?}\n", self.tsp_tour, self.packing_plan)
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
}
