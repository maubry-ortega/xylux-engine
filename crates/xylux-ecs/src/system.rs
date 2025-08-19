//! # Módulo de Sistemas ECS
//!
//! Define cómo ejecutar sistemas sobre el mundo ECS.
//! Incluye:
//! - `System`: encapsula una función que opera sobre el mundo.
//! - `TaskGraph`: organiza sistemas con dependencias y permite ejecución segura.
//! - Ejemplo: `move_system`, que actualiza posición según Velocity.

use crate::component::{Transform, Velocity};
use crate::query::Query;
use crate::world::World;
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
        self.systems.insert(name.clone(), (system, dependencies));
        // Es más eficiente recalcular el orden aquí que en cada `run()`.
        self.compute_execution_order();
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
        // Para evitar problemas con el borrow checker al iterar y mutar `self.systems`
        // a la vez, movemos temporalmente los sistemas fuera de la estructura.
        let mut systems = std::mem::take(&mut self.systems);
        for name in &self.execution_order {
            if let Some((system, _)) = systems.get_mut(name) {
                system.run(world);
            }
        }
        self.systems = systems;
    }
}

/// --- EJEMPLO DE SISTEMA ---
/// Sistema que mueve entidades según su Velocity.
pub fn move_system() -> System {
    // Este sistema demuestra una query con acceso mutable a un componente
    // y de solo lectura a otro.
    System::new(|world: &mut World| {
        for (transform, velocity) in Query::<(&mut Transform, &Velocity)>::new(world).iter() {
            transform.position += velocity.0;
        }
    })
}
