//! # Módulo de Sistemas ECS
//!
//! Define cómo ejecutar sistemas sobre el mundo ECS.
//! Incluye:
//! - `System`: encapsula una función que opera sobre el mundo.
//! - `TaskGraph`: organiza sistemas con dependencias y permite ejecución segura.
//! - Ejemplo: `move_system`, que actualiza posición según Velocity.

use crate::query::{Query, Velocity};
use crate::world::World;
use crate::component::Transform;
use std::collections::{HashMap, HashSet};

/// --- SYSTEM ---
/// Representa un sistema ECS ejecutable.
pub struct System {
    func: Box<dyn FnMut(&mut World) + Send + Sync>,
}

impl System {
    /// Crea un nuevo sistema a partir de una función.
    pub fn new<F: FnMut(&mut World) + Send + Sync + 'static>(func: F) -> Self {
        Self { func: Box::new(func) }
    }

    /// Ejecuta el sistema sobre el mundo.
    pub fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
}

/// --- TASK GRAPH ---
/// Grafo de tareas/sistemas con dependencias.
pub struct TaskGraph {
    systems: HashMap<String, (System, Vec<String>)>, // nombre -> (sistema, dependencias)
    execution_order: Vec<String>,                    // orden topológico calculado
}

impl TaskGraph {
    /// Crea un grafo vacío.
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            execution_order: Vec::new(),
        }
    }

    /// Añade un sistema al grafo.
    pub fn add_system(&mut self, name: String, dependencies: Vec<String>, system: System) {
        self.systems.insert(name, (system, dependencies));
    }

    /// Calcula el orden topológico de ejecución según dependencias.
    fn compute_execution_order(&mut self) {
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();
        self.execution_order.clear();

        fn visit(
            node: &str,
            systems: &HashMap<String, (System, Vec<String>)>,
            visited: &mut HashSet<String>,
            temp_mark: &mut HashSet<String>,
            order: &mut Vec<String>,
        ) {
            if visited.contains(node) {
                return;
            }
            if temp_mark.contains(node) {
                panic!("Ciclo detectado en TaskGraph en '{}'", node);
            }
            temp_mark.insert(node.to_string());

            if let Some((_, deps)) = systems.get(node) {
                for dep in deps {
                    visit(dep, systems, visited, temp_mark, order);
                }
            }

            temp_mark.remove(node);
            visited.insert(node.to_string());
            order.push(node.to_string());
        }

        for node in self.systems.keys() {
            visit(node, &self.systems, &mut visited, &mut temp_mark, &mut self.execution_order);
        }
    }

    /// Ejecuta todos los sistemas en orden topológico.
    pub fn run(&mut self, world: &mut World) {
        self.compute_execution_order();
        for name in &self.execution_order {
            if let Some((system, _)) = self.systems.get_mut(name) {
                system.run(world);
            }
        }
    }
}

/// --- EJEMPLO DE SISTEMA ---
/// Sistema que mueve entidades según su Velocity.
pub fn move_system() -> System {
    System::new(|world: &mut World| {
        let mut query = Query::<(&Transform, &mut Velocity)>::new(world);

        for (transform, velocity) in query.iter() {
            // Acceso seguro mediante puntero crudo solo donde es estrictamente necesario
            let transform_ptr = transform as *const Transform as *mut Transform;
            unsafe {
                (*transform_ptr).position += velocity.0;
            }
        }
    })
}
