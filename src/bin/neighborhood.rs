use std::fs;
use std::time::Instant;
use ttp::models::instance::Instance;
use ttp::models::solution::Solution;

fn main() {
    println!("--- TTP Neighborhood Search (Item Window Shift per Node) ---");

    // rutas
    let instance_path = "instances/a280_n279_bounded-strongly-corr_01.ttp";
    let input_solution_path = "a280-TTP_279_-10516.546711.txt"; // Solución base a mejorar
                                                                // -------------------------

    println!("Instancia : {}", instance_path);
    println!("Solución  : {}", input_solution_path);

    let instance = Instance::load(instance_path).expect("No se pudo cargar la instancia");

    let content =
        fs::read_to_string(input_solution_path).expect("No se pudo leer el archivo de solución");
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        panic!("El archivo de solución debe tener al menos 2 líneas (Tour, PackingPlan).");
    }

    let tour_line = lines[0]
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']');
    let tour_1based: Vec<usize> = tour_line
        .split(',')
        .map(|s| s.trim().parse::<usize>().expect("Token de tour inválido"))
        .collect();
    let mut tour: Vec<usize> = tour_1based.iter().map(|&x| x - 1).collect();

    if let Some(pos) = tour.iter().position(|&x| x == 0) {
        tour.rotate_left(pos);
    }

    // packing plan [idx1,idx2,...] -> máscara binaria
    let pack_line = lines[1]
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']');
    let picked_1based: Vec<usize> = if pack_line.is_empty() {
        Vec::new()
    } else {
        pack_line
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<usize>()
                    .expect("Token de packing inválido")
            })
            .collect()
    };

    let mut base_mask = vec![0usize; instance.num_items];
    for &idx in &picked_1based {
        if idx > 0 && idx <= instance.num_items {
            base_mask[idx - 1] = 1;
        }
    }

    let base_sol = Solution::evaluate(&instance, &tour, &base_mask);
    println!(
        "Ganancia base: {:.6}  (items seleccionados: {})",
        base_sol.ob,
        picked_1based.len()
    );

    let mut node_items: Vec<Vec<usize>> = vec![Vec::new(); instance.dimension + 1];
    for &(idx, _p, _w, node) in &instance.items {
        if node > 0 && node <= instance.dimension {
            node_items[node - 1].push(idx - 1);
        }
    }

    //    Para cada nodo que tenga más de 1 ítem, rotamos la ventana de ítems
    //    seleccionados (manteniendo el mismo número de ítems tomados).
    //    Ejemplo: nodo con 5 ítems, tomamos 3 -> 01110
    //    Vecindades: 11100, 00111, 10011, 01001

    let start = Instant::now();
    let mut mejor_ob = base_sol.ob;
    let mut mejor_mask = base_mask.clone();
    let mut mejor_vecindad: usize = 0; // número de vecindad que fue la mejor
    let mut vecindad_global: usize = 0; // contador global de vecindades generadas

    for node_idx in 0..instance.dimension {
        let items = &node_items[node_idx];
        let n = items.len();
        if n <= 1 {
            continue; // nada que rotar con 0 o 1 ítem
        }

        // Selección actual de este nodo (0=no tomado, 1=tomado)
        let seleccion_actual: Vec<usize> = items.iter().map(|&i| base_mask[i]).collect();
        let num_tomados: usize = seleccion_actual.iter().sum();

        if num_tomados == 0 || num_tomados == n {
            continue; // todas tomadas o ninguna -> no hay rotaciones distintas
        }

        // Generar todas las rotaciones cíclicas de la selección (excluyendo la original)
        for rot in 1..n {
            vecindad_global += 1;
            let mut nueva_seleccion = seleccion_actual.clone();
            nueva_seleccion.rotate_right(rot);

            // Construir máscara candidata
            let mut candidata = base_mask.clone();
            for (k, &item_idx) in items.iter().enumerate() {
                candidata[item_idx] = nueva_seleccion[k];
            }

            // Verificar que el peso total no exceda la capacidad (evaluación rápida)
            let peso_total: f64 = candidata
                .iter()
                .enumerate()
                .filter(|(_, &v)| v == 1)
                .map(|(i, _)| instance.items[i].2)
                .sum();

            if peso_total > instance.capacity_of_knapsack {
                continue; // vecindad inviable
            }

            let sol = Solution::evaluate(&instance, &tour, &candidata);

            if sol.ob > mejor_ob {
                mejor_ob = sol.ob;
                mejor_mask = candidata;
                mejor_vecindad = vecindad_global;
            }
        }
    }

    let duracion = start.elapsed();
    println!(
        "Vecindades evaluadas: {}  |  Tiempo: {:.3}s",
        vecindad_global,
        duracion.as_secs_f64()
    );

    if mejor_ob > base_sol.ob {
        println!(
            "¡Mejora encontrada!  Ganancia: {:.6}  (vecindad #{})",
            mejor_ob, mejor_vecindad
        );

        // Construir y guardar la solución mejorada
        let mut sol_mejorada = Solution::evaluate(&instance, &tour, &mejor_mask);
        sol_mejorada.computation_time = duracion.as_secs_f64();

        // Nombre: <instancia_base>_vecindad_<N>_<ganancia>.txt
        // Obtener prefijo TTP del archivo de solución si sigue el patrón conocido
        // e.g. "a280-TTP_279_..." -> "a280-TTP_279"
        let solution_base = {
            let sin_ext = input_solution_path.trim_end_matches(".txt");
            // Buscar el último '_' para encontrar el prefijo antes de la ganancia o "sol_X"
            // Intentamos conservar el bloque "a280-TTP_279"
            if let Some(pos) = sin_ext.rfind('_') {
                // Si lo anterior también termina en dígito agrupado, quizas sea mejor solo tomar
                // hasta el segundo '_' desde el final
                let prefix = &sin_ext[..pos];
                if let Some(pos2) = prefix.rfind('_') {
                    let mid = &prefix[..pos2];
                    // Si mid termina en número (TTP_279) úsalo como base
                    if mid.chars().last().map_or(false, |c| c.is_ascii_digit()) {
                        mid.to_string()
                    } else {
                        prefix.to_string()
                    }
                } else {
                    prefix.to_string()
                }
            } else {
                sin_ext.to_string()
            }
        };

        let ganancia_str = format!("{:.6}", mejor_ob);
        let nombre_salida = format!(
            "{}_vecindad_{}_{}.txt",
            solution_base, mejor_vecindad, ganancia_str
        );

        sol_mejorada.write_result(&nombre_salida);
        println!("Solución guardada en: {}", nombre_salida);
    } else {
        println!(
            "No se encontró mejora. La solución base ({:.6}) sigue siendo óptima en esta vecindad.",
            base_sol.ob
        );
    }
}
