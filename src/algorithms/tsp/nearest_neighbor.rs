use crate::models::path::Path;

pub struct NearestNeighborTSP;

impl NearestNeighborTSP {
    pub fn solve(path: &Path) -> Path {
        let n = path.nodes.len();
        if n == 0 {
            return Path::new(vec![]);
        }

        let mut visited = vec![false; n];
        let mut route = Vec::with_capacity(n);

        // Start from node 0 (assuming it's the depot or first node)
        let mut current_idx = 0;
        route.push(path.nodes[0]);
        visited[0] = true;

        for _ in 1..n {
            let mut best_dist = f64::INFINITY;
            let mut next_idx = 0;

            // Find nearest unvisited neighbor
            // For 33k nodes, this linear scan is acceptable (33k * 33k ~= 1B ops)
            // In release mode, this takes ~0.5 - 2 seconds.
            // let (id1, x1, y1) = path.nodes[current_idx];
            let (_, x1, y1) = path.nodes[current_idx];

            for j in 0..n {
                if !visited[j] {
                    let (_, x2, y2) = path.nodes[j];
                    let dx = x1 - x2;
                    let dy = y1 - y2;
                    let dist_sq = dx * dx + dy * dy; // Avoid sqrt for comparison

                    if dist_sq < best_dist {
                        best_dist = dist_sq;
                        next_idx = j;
                    }
                }
            }

            visited[next_idx] = true;
            route.push(path.nodes[next_idx]);
            current_idx = next_idx;
        }

        Path::new(route)
    }
}
