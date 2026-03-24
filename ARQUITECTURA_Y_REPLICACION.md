# Documentación de Arquitectura TTP - Guía de Replicación

## TABLA DE CONTENIDOS
1. [Resumen Ejecutivo](#resumen-ejecutivo)
2. [Estructura del Proyecto](#estructura-del-proyecto)
3. [Cómo se Leen las Instancias](#cómo-se-leen-las-instancias)
4. [Módulos Principales](#módulos-principales)
5. [Flujo Principal de Ejecución](#flujo-principal-de-ejecución)
6. [Instrucciones para Replicar en Otro Repositorio](#instrucciones-para-replicar)

---
**
## Resumen Ejecutivo

El proyecto **TTP (Traveling Thief Problem)** es un optimizador que resuelve un problema combinatorio integrado que mezcla:
- **TSP**: Encontrar la mejor ruta para visitar ciudades
- **KP**: Decidir qué items cargar en una mochila
- **Integración**: Velocidad de viaje depende del peso cargado

**Tecnología:** Rust (seguridad de memoria, rendimiento)
**Objetivo:** Maximizar ganancia de items menos penalización por tiempo de viaje

---

## Estructura del Proyecto

```
ttp-1/
├── src/
│   ├── main.rs                  # Programa principal interactivo
│   ├── lib.rs                   # Exponedor de módulos públicos
│   ├── models/
│   │   ├── mod.rs               # Exportaciones
│   │   ├── instance.rs          # Carga y parsea instancias TTP ⭐
│   │   ├── path.rs              # Estructura Path (ruta física)
│   │   └── solution.rs          # Evaluación de soluciones
│   ├── algorithms/
│   │   ├── tsp/
│   │   │   ├── nearest_neighbor.rs    # Greedy O(n²), mejor para 33k+
│   │   │   ├── two_opt.rs             # Local search
│   │   │   ├── nearest_insertion.rs   # Construcción gradual
│   │   │   ├── simulated_annealing.rs # Metaheurística
│   │   │   ├── tabu_search.rs         # Metaheurística
│   │   │   └── lin_kernighan.rs       # Avanzado
│   │   ├── kp/
│   │   │   ├── greedy.rs              # Por eficiencia (profit/weight)
│   │   │   └── random.rs              # Aleatorio
│   │   └── ttp/
│   │       └── hill_climbing.rs       # Optimizador integrado TTP
│   └── bin/
│       ├── improver.rs          # Mejora soluciones existentes
│       └── verifier.rs          # Verifica soluciones
├── instances/                   # Archivos de entrada
│   ├── a280_n279_bounded-strongly-corr_01.ttp
│   ├── a280_n2790_uncorr_10.ttp
│   ├── fnl4461_n4460_bounded-strongly-corr_01.ttp
│   ├── fnl4461_n44600_uncorr_10.ttp
│   └── pla33810_n338090_uncorr_10.ttp
├── Cargo.toml                   # Configuración Rust
└── ARQUITECTURA_Y_REPLICACION.md # Este archivo
```

---

## Cómo se Leen las Instancias

### 1. UBICACIÓN Y FORMATO DEL ARCHIVO

**Ubicación:** `/instances/*.ttp`

**Formato de Ejemplo (a280_n279_bounded-strongly-corr_01.ttp):**

```
PROBLEM NAME: a280-TTP
KNAPSACK DATA TYPE: bounded strongly corr
DIMENSION: 280
NUMBER OF ITEMS: 279
CAPACITY OF KNAPSACK: 25936
MIN SPEED: 0.1
MAX SPEED: 1
RENTING RATIO: 5.61
EDGE_WEIGHT_TYPE: CEIL_2D

NODE_COORD_SECTION
1 288 149
2 288 129
3 288 109
...
280 288 149

ITEMS SECTION
1 500 100 1
2 600 120 5
3 450 80 10
...
279 400 85 45
```

### 2. ESTRUCTURA DE DATOS DE INSTANCIA (Rust)

```rust
pub struct Instance {
    pub problem_name: String,              // "a280-TTP"
    pub dimension: usize,                  // 280 ciudades
    pub num_items: usize,                  // 279 items
    pub capacity_of_knapsack: usize,       // 25936 kg (máximo peso)
    pub min_speed: f64,                    // 0.1 (km/h)
    pub max_speed: f64,                    // 1.0 (km/h)
    pub renting_ratio: f64,                // 5.61 (penalización/tiempo)
    pub node_coords: Vec<(usize, f64, f64)>, // [(id, x, y), ...]
    pub items: Vec<(usize, i32, usize, usize)>, // [(id, profit, weight, node_id), ...]
}
```

### 3. PROCESO DE CARGA (Flujo en instance.rs)

```
Archivo .ttp
    ↓
Instance::load(file_path)
    ↓
Abre archivo
    ↓
Lee línea por línea
    ↓
├─ Extrae metadatos:
│  ├─ "PROBLEM NAME:" → problem_name
│  ├─ "DIMENSION:" → dimension
│  ├─ "NUMBER OF ITEMS:" → num_items
│  ├─ "CAPACITY OF KNAPSACK:" → capacity_of_knapsack
│  ├─ "MIN SPEED:" → min_speed
│  ├─ "MAX SPEED:" → max_speed
│  └─ "RENTING RATIO:" → renting_ratio
│
├─ Detecta "NODE_COORD_SECTION":
│  └─ Para cada línea numérica: parsea (id, x, y) → almacena en node_coords
│
└─ Detecta "ITEMS SECTION":
   └─ Para cada línea: parsea (id, profit, weight, node) → almacena en items

    ↓
Retorna Instance completa
```

### 4. CÓDIGO CLAVE DE LECTURA (en instance.rs)

El método principal es `pub fn load(file_path: &str) -> Result<Instance, Box<dyn Error>>`:

**Pseudocódigo:**
```rust
fn load(file_path: &str) -> Result<Instance, Error> {
    let file = open_file(file_path)
    let mut instance = Instance { /* initialized empty */ }
    let mut section = None  // Qué sección estamos leyendo

    for line in file.lines() {
        let trimmed = line.trim()

        // 1. DETECTAR SECCIONES
        if trimmed.contains("PROBLEM NAME:") {
            instance.problem_name = extract_after_colon(trimmed)
        } else if trimmed.contains("DIMENSION:") {
            instance.dimension = extract_and_parse_i64(trimmed)
        } else if trimmed.contains("NUMBER OF ITEMS:") {
            instance.num_items = extract_and_parse_i64(trimmed)
        } else if trimmed.contains("CAPACITY OF KNAPSACK:") {
            instance.capacity_of_knapsack = extract_and_parse_i64(trimmed)
        } else if trimmed.contains("MIN SPEED:") {
            instance.min_speed = extract_and_parse_f64(trimmed)
        } else if trimmed.contains("MAX SPEED:") {
            instance.max_speed = extract_and_parse_f64(trimmed)
        } else if trimmed.contains("RENTING RATIO:") {
            instance.renting_ratio = extract_and_parse_f64(trimmed)
        }

        // 2. CAMBIAR SECCIÓN
        else if trimmed == "NODE_COORD_SECTION" {
            section = Some("NODE_COORD")
        } else if trimmed == "ITEMS SECTION" {
            section = Some("ITEMS")
        }

        // 3. PARSEAR CONTENIDO DE SECCIÓN
        else if let Some(current_section) = section {
            if trimmed.is_empty() || trimmed.starts_with("%") {
                continue  // Ignorar líneas vacías y comentarios
            }

            if current_section == "NODE_COORD" {
                // Formato: "1 288 149"
                let parts: Vec<&str> = trimmed.split_whitespace().collect()
                if parts.len() >= 3 {
                    let id = parts[0].parse::<usize>()?
                    let x = parts[1].parse::<f64>()?
                    let y = parts[2].parse::<f64>()?
                    instance.node_coords.push((id, x, y))
                }
            } else if current_section == "ITEMS" {
                // Formato: "1 500 100 1"
                let parts: Vec<&str> = trimmed.split_whitespace().collect()
                if parts.len() >= 4 {
                    let id = parts[0].parse::<usize>()?
                    let profit = parts[1].parse::<i32>()?
                    let weight = parts[2].parse::<usize>()?
                    let node = parts[3].parse::<usize>()?
                    instance.items.push((id, profit, weight, node))
                }
            }
        }
    }

    Ok(instance)
}
```

### 5. VALIDACIÓN DESPUÉS DE CARGAR

Después de leer el archivo, se valida:
```
- Número de nodos == dimension
- Número de items == num_items
- Cada item tiene node_id dentro de rango [1, dimension]
- capacity_of_knapsack > 0
- min_speed <= max_speed
- renting_ratio > 0
```

### 6. EJEMPLO DE EJECUCIÓN DE CARGA

```
Input: load("./instances/a280_n279_bounded-strongly-corr_01.ttp")

Output:
Instance {
    problem_name: "a280-TTP",
    dimension: 280,
    num_items: 279,
    capacity_of_knapsack: 25936,
    min_speed: 0.1,
    max_speed: 1.0,
    renting_ratio: 5.61,
    node_coords: [
        (1, 288.0, 149.0),
        (2, 288.0, 129.0),
        (3, 288.0, 109.0),
        ...,
        (280, 288.0, 149.0)
    ],
    items: [
        (1, 500, 100, 1),
        (2, 600, 120, 5),
        (3, 450, 80, 10),
        ...,
        (279, 400, 85, 45)
    ]
}
```

---

## Módulos Principales

### A. Instance (src/models/instance.rs)

**Responsabilidad:** Carga y parsea archivos .ttp

**Funciones públicas:**
- `load(file_path: &str) → Result<Instance, Error>`: Carga archivo completo
- Acceso a miembros públicos para lectura

**Uso en main.rs:**
```rust
let instance = Instance::load("./instances/a280_n279_bounded-strongly-corr_01.ttp")?;
println!("Instancia cargada: {} con {} nodos y {} items",
         instance.problem_name,
         instance.dimension,
         instance.num_items);
```

### B. Path (src/models/path.rs)

**Responsabilidad:** Representa una ruta física de ciudades

**Estructura:**
```rust
pub struct Path {
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}
```

**Funciones:**
- `new(coords: Vec<(usize, f64, f64)>) → Path`: Crea Path desde coordenadas
- `length() → f64`: Calcula distancia total del tour (euclidiana con CEIL)
- `has_node(id: usize) → bool`: Verifica si contiene un nodo

**Uso:**
```rust
let path = Path::new(instance.node_coords);
let total_distance = path.length();
```

### C. Solution (src/models/solution.rs)

**Responsabilidad:** Evalúa soluciones TTP completas

**Estructura:**
```rust
pub struct Solution {
    pub tsp_tour: Vec<usize>,        // 0-based indices
    pub packing_plan: Vec<usize>,    // 0=no coge, 1=coge item
    pub effective_packing_plan: Vec<usize>, // Validado
    pub fp: f64,                     // Ganancia total (profit)
    pub ft: f64,                     // Tiempo total
    pub ob: f64,                     // Objetivo = fp - ft*renting_ratio
    pub computation_time: f64,       // Segundos
}
```

**Función clave:**
```rust
pub fn evaluate(instance: &Instance,
                tsp_tour: &[usize],
                packing_plan: &[usize]) → Solution {
    // 1. Simula el viaje del ladrón
    // 2. En cada ciudad i, recoge items asignados a i si packing_plan[j] == 1
    // 3. Calcula peso actual y velocidad: v = v_max - (w/cap)*(v_max-v_min)
    // 4. Suma tiempos de viaje: tiempo += distancia / velocidad
    // 5. Calcula objetivo final
}
```

**Uso:**
```rust
let solution = Solution::evaluate(&instance, &tour, &packing);
println!("Objetivo: {}", solution.ob);
solution.write_result("output.txt")?;
```

### D. TSP Algorithms (src/algorithms/tsp/*.rs)

**Patrón de interfaz:**
```rust
pub struct AlgorithmName;

impl AlgorithmName {
    pub fn solve(path: &Path) -> Path {
        // Retorna Path optimizado
    }
}
```

**Algoritmos disponibles:**

1. **NearestNeighborTSP** - O(n²)
   - Greedy: siempre va al nodo más cercano no visitado
   - Mejor para instancias 33k+ nodos
   - ~0.5-2 segundos para 33k nodos

2. **TwoOptTSP** - Local Search
   - Una pasada única mejorando aristas
   - Rápido para instancias medianas

3. **NearestInsertionTSP** - Construcción
   - Insertando nodos gradualmente
   - Experimental

4. **SimulatedAnnealingTSP** - Metaheurística
   - Temperature: 1000, cooling_rate: 0.0003
   - Acepta peores soluciones con probabilidad

5. **TabuSearchTSP** - Metaheurística
   - max_iterations: 100, tabu_tenure: 30
   - Evita ciclos con list de tabú

6. **LinKernighanTSP** - Avanzado
   - Multi-edge moves

**Uso:**
```rust
let optimized_path = NearestNeighborTSP::solve(&path);
```

### E. KP Algorithms (src/algorithms/kp/*.rs)

**GreedyKP:**
```rust
pub fn solve(instance: &Instance, tsp_tour: &[usize]) -> Vec<usize> {
    // 1. Ordena items por eficiencia = profit / weight (descendente)
    // 2. Para cada item en orden, si cabe en capacidad, lo coge
    // 3. Retorna Vec<usize> con 0s y 1s
}
```

**Uso:**
```rust
let packing = GreedyKP::solve(&instance, &tour);
// packing[i] = 1 si coge item i, 0 si no
```

### F. TTP Optimizer (src/algorithms/ttp/hill_climbing.rs)

**Función principal:**
```rust
pub fn optimize_full(instance: &Instance,
                     mut tsp_tour: Vec<usize>,
                     mut packing: Vec<usize>)
                     → (Vec<usize>, Vec<usize>) {
    // Ejecuta 4 tipos de mejoras en ciclos:
    // 1. run_2opt(): Invierte segmentos de ruta
    // 2. run_swap(): Intercambia posiciones de ciudades
    // 3. run_bit_flip(): Añade/quita items
    // 4. run_item_swap(): Intercambia items

    // Usa "First Improvement" - acepta primer movimiento que mejora
}
```

**Nota:** Actualmente desactivado en main.rs pero usado en improver.rs

---

## Flujo Principal de Ejecución

### Paso 1: Interfaz de Selección
```rust
// main.rs lee archivos en ./instances/
let files = fs::read_dir("./instances")?
                .filter_map(|e| e.ok())
                .map(|e| e.file_name())
                .collect::<Vec<_>>();

let selection = Select::new()
    .with_prompt("Selecciona una instancia:")
    .items(&files)
    .interact()?;

let selected_file = &files[selection];
```

### Paso 2: Cargar Instancia
```rust
let instance = Instance::load(&format!("./instances/{:?}", selected_file))?;
println!("Instancia: {}", instance.problem_name);
println!("  Nodos: {}", instance.dimension);
println!("  Items: {}", instance.num_items);
println!("  Capacidad: {} kg", instance.capacity_of_knapsack);
```

### Paso 3: Crear Ruta Inicial
```rust
let path = Path::new(instance.node_coords.clone());
let initial_length = path.length();
println!("Ruta inicial total: {:.2} km", initial_length);
```

### Paso 4: Seleccionar Algoritmo TSP
```rust
let algo_selection = Select::new()
    .with_prompt("Selecciona algoritmo TSP:")
    .items(&[
        "Nearest Neighbor (Fast Greedy - Best for 33k+)",
        "Nearest insertion TSP",
        "Two opt TSP",
        "Simulated annealing TSP",
        "Tabu search TSP",
        "Lin-Kernighan TSP"
    ])
    .interact()?;
```

### Paso 5: Ejecutar TSP Solver
```rust
let shortest_path = match algo_selection {
    0 => NearestNeighborTSP::solve(&path),
    1 => NearestInsertionTSP::solve(&path),
    2 => TwoOptTSP::solve(&path),
    3 => SimulatedAnnealingTSP::solve(&path),
    4 => TabuSearchTSP::solve(&path),
    5 => LinKernighanTSP::solve(&path),
    _ => path,
};

println!("Ruta optimizada: {:.2} km", shortest_path.length());
```

### Paso 6: Convertir Path a Tour (0-based)
```rust
let mut tour: Vec<usize> = shortest_path.nodes
    .iter()
    .map(|n| n.id - 1)  // Convierte de 1-based a 0-based
    .collect();

// Asegura comenzar en depot (nodo 0)
if let Some(pos_0) = tour.iter().position(|&x| x == 0) {
    tour.rotate_left(pos_0);
}
```

### Paso 7: Ejecutar knapsack
```rust
let packing_plan = GreedyKP::solve(&instance, &tour);
println!("Items seleccionados: {}", packing_plan.iter().sum::<usize>());
```

### Paso 8: Evaluar Solución
```rust
let solution = Solution::evaluate(&instance, &tour, &packing_plan);
println!("Ganancia: {}", solution.fp);
println!("Tiempo: {:.2} horas", solution.ft);
println!("Objetivo: {}", solution.ob);
println!("Tiempo cálculo: {:.6} segundos", solution.computation_time);
```

### Paso 9: Generar Nombre y Guardar
```rust
let filename = format!("{}_{}_{:.6}.txt",
    instance.problem_name,
    instance.num_items,
    solution.ob);

solution.write_result(&filename)?;
println!("Resultado guardado en: {}", filename);
```

---

## Instrucciones para Replicar en Otro Repositorio

### OPCIÓN A: Replicación Manual Paso a Paso

#### 1. CREAR PROYECTO BASE

```bash
# En la carpeta destino
cargo new ttp-replica
cd ttp-replica

# Estructura inicial
mkdir -p src/{models,algorithms/{tsp,kp,ttp},bin}
mkdir instances
```

#### 2. ARCHIVO Cargo.toml

**Copiar dependencias del original:**
```toml
[package]
name = "ttp-replica"
version = "0.1.0"
edition = "2021"

[dependencies]
dialoguer = "0.11"
itertools = "0.12"
rand = "0.8"

[[bin]]
name = "improver"
path = "src/bin/improver.rs"

[[bin]]
name = "verifier"
path = "src/bin/verifier.rs"
```

#### 3. IMPLEMENTAR En ORDEN:

**3.1 src/models/instance.rs** (⭐ PRIMERO)
- Copiar estructura `Instance`
- Implementar `Instance::load(file_path)` parseando:
  1. Líneas con metadata (extraer después de ':')
  2. Sección NODE_COORD_SECTION (parsear id, x, y)
  3. Sección ITEMS SECTION (parsear id, profit, weight, node)

**3.2 src/models/path.rs**
- Estructura Node: id, x, y
- Estructura Path: Vec<Node>
- Funciones:
  - `new()`: Crear desde Vec<(id, x, y)>
  - `length()`: Distancia euclidiana con CEIL
  - `has_node()`

**3.3 src/models/solution.rs**
- Estructura Solution con 8 campos
- Función `evaluate()`: Simular viaje, calcular velocidad dinámica
- Función `write_result()`: Guardar en archivo formato específico

**3.4 src/models/mod.rs**
- Exportar: `pub mod instance, path, solution;`
- Exportar públicamente: las estructuras principales

**3.5 src/algorithms/tsp/nearest_neighbor.rs** (⭐ SEGUNDO)
- Algoritmo greedy O(n²)
- Función `solve(path: &Path) -> Path`

**3.6 Otros TSP** (en orden de prioridad):
1. `two_opt.rs` (rápido)
2. `simulated_annealing.rs` (metaheurística)
3. `tabu_search.rs` (metaheurística)
4. Resto

**3.7 src/algorithms/kp/greedy.rs** (⭐ TERCERO)
- Ordena por profit/weight
- Llena mochila según orden

**3.8 src/algorithms/ttp/hill_climbing.rs** (⭐ CUARTO - Opcional)
- Optimizador integrado con 4 movimientos
- Puede dejarse para después

**3.9 src/lib.rs**
```rust
pub mod models;
pub mod algorithms;

pub use models::{Instance, Path, Solution};
pub use algorithms::{
    tsp::NearestNeighborTSP,
    kp::GreedyKP,
    ttp::HillClimbingTTP,
};
```

**3.10 src/main.rs** (⭐ ÚLTIMO)
- Loop interactivo con `select!` de dialoguer
- Cargar instancia
- Crear Path inicial
- Menú de algoritmo TSP
- Ejecutar KP
- Evaluar y guardar

---

### OPCIÓN B: Replicación AUTOMÁTICA (Recomendada)

Usa el siguiente prompt para que otro modelo replique:

```
Necesito replicar la arquitectura de un proyecto Rust de optimización TTP
(Traveling Thief Problem) en un nuevo repositorio.

El proyecto original está en: /Users/emirdz/repos/ttp-1

MI TAREA:
1. Crear una copia NOT EXACTA pero FUNCTIONAL de la arquitectura
2. Implementar el sistema de lectura de instancias idéntico
3. Mantener la misma estructura modular
4. Probar que funciona con instancias reales

INSTRUCCIONES PASO A PASO:

### ORDEN DE IMPLEMENTACIÓN:

1. **PRIMERO - Sistema de carga de instancias:**
   - Crear src/models/instance.rs
   - Estructura Instance con campos: problem_name, dimension, num_items,
     capacity_of_knapsack, min_speed, max_speed, renting_ratio,
     node_coords: Vec<(usize, f64, f64)>, items: Vec<(usize, i32, usize, usize)>
   - Implementar Instance::load(file_path) que:
     a) Abre archivo .ttp
     b) Lee línea por línea detectando secciones por keywords
     c) Extrae metadata con split(':')
     d) Parsea NODE_COORD_SECTION como (id, x, y)
     e) Parsea ITEMS SECTION como (id, profit, weight, node)
     f) Retorna Instance completa

2. **SEGUNDO - Representación de rutas:**
   - Crear src/models/path.rs
   - Estructura Path con nodes: Vec<Node> donde Node = {id, x, y}
   - Función new(coords) crea Path
   - Función length() calcula distancia total euclidiana con CEIL

3. **TERCERO - Evaluación de soluciones:**
   - Crear src/models/solution.rs
   - Estructura Solution con: tsp_tour, packing_plan, fp, ft, ob, computation_time
   - Función evaluate() simula viaje:
     * Para cada ciudad en tour, recoge items del packing_plan
     * Calcula peso y velocidad: v = vmax - (w/cap)*(vmax-vmin)
     * Suma tiempos: tiempo += distancia/velocidad
     * Calcula ob = fp - ft*renting_ratio

4. **CUARTO - Algoritmo TSP principal:**
   - Crear src/algorithms/tsp/nearest_neighbor.rs
   - Implementar NearestNeighborTSP::solve() greedy O(n²)
   - Para instancias 33k+ nodos

5. **QUINTO - Algoritmo Knapsack:**
   - Crear src/algorithms/kp/greedy.rs
   - Implementar GreedyKP::solve() ordenando por profit/weight

6. **SEXTO - main.rs:**
   - Loop interactivo con dialoguer
   - Carga instancia → crea Path → ejecuta TSP → ejecuta KP → evalúa → guarda

### ARCHIVOS PARA COPIAR DIRECTAMENTE:
- Todas las demás instancias de algoritmos TSP del original
- Estructura del lib.rs
- Estructura del Cargo.toml

### TESTING:
- Probara con instancia a280_n279
- Verificar que genera archivo de salida correcto
```

---

### OPCIÓN C: Fusionar Repositorios (Git)

```bash
# En nuevo repositorio
cd /ruta/nuevo-repo

# Agregar remoto original
git remote add original https://github.com/usuario/ttp-1.git
git fetch original

# Traer rama específica
git checkout -b replica original/feat/pruebas

# O copiar archivos específicos
git checkout original/feat/pruebas -- src/models/
git checkout original/feat/pruebas -- src/algorithms/
git checkout original/feat/pruebas -- instances/
```

---

## Checklist de Replicación

- [ ] Crear src/models/instance.rs con lectura de .ttp
- [ ] Crear src/models/path.rs con Path y cálculo de distancia
- [ ] Crear src/models/solution.rs con evaluación TTP
- [ ] Crear src/models/mod.rs exportando módulos
- [ ] Crear src/algorithms/tsp/nearest_neighbor.rs
- [ ] Crear src/algorithms/tsp/dos_opt.rs (mínimo)
- [ ] Crear src/algorithms/kp/greedy.rs
- [ ] Crear src/algorithms/mod.rs exportando
- [ ] Crear src/lib.rs exportando lib completa
- [ ] Crear src/main.rs con flujo interactivo
- [ ] Crear Cargo.toml con dependencias
- [ ] Copiar carpeta instances/ con archivos .ttp
- [ ] Testar: `cargo run` con instancia pequeña
- [ ] Testar: Verificar archivo de salida generado

---

## Comandos Útiles para Testing

```bash
# Compilar
cargo build --release

# Ejecutar
cargo run

# Ejecutar con profiling
time cargo run --release

# Ejecutar herramienta verifier
cargo run --bin verifier

# Ver estructura
tree -L 3 src/
```

---

## Notas Finales

1. **Clave para replicación:** Empieza por `instance.rs` - es lo más crítico
2. **Testing:** Usa instancia a280_n279 (pequeña, ~5 segundos de ejecución)
3. **Escalabilidad:** Nearest Neighbor es clave para instancias 33k+
4. **Modularidad:** Cada algoritmo debe ser independiente
5. **Flujo:** TSP → KP → TTP (en ese orden, cada uno sobre resultado anterior)

