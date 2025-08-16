use crate::query::{Query, Velocity};
use crate::world::World;
use crate::component::Transform;

/// Representa un sistema ECS encapsulado como función.
pub struct System {
    func: Box<dyn FnMut(&mut World) + Send + Sync>,
}

impl System {
    pub fn new<F: FnMut(&mut World) + Send + Sync + 'static>(func: F) -> Self {
        Self {
            func: Box::new(func),
        }
    }

    pub fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
}

/// Grafo de tareas/sistemas con dependencias
pub struct TaskGraph {
    systems: Vec<(String, System, Vec<String>)>, // (nombre, sistema, dependencias)
}

impl TaskGraph {
    pub fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn add_system(&mut self, name: String, dependencies: Vec<String>, system: System) {
        self.systems.push((name, system, dependencies));
    }

    /// Ejecuta todos los sistemas secuencialmente por ahora
    pub fn run(&mut self, world: &mut World) {
        for (_, system, _) in self.systems.iter_mut() {
            system.run(world);
        }
    }
}

/// --- Sistema de ejemplo: mueve entidades según su Velocity ---
pub fn move_system() -> System {
    System::new(|world: &mut World| {
        // Creamos un query para obtener Transform y Velocity
        let mut query = Query::<(&Transform, &mut Velocity)>::new(world);

        for (transform, velocity) in query.iter() {
            // Aquí sí podemos actualizar la posición
            // Necesitamos un bloque unsafe porque Query nos da referencias crudas detrás de escenas
            unsafe {
                let pos = &mut (*(transform as *const Transform as *mut Transform)).position;
                *pos += velocity.0;
            }
        }
    })
}
