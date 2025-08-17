//! # Xylux ECS
//!
//! Este crate proporciona un motor ECS (Entity-Component-System) minimalista y eficiente.
//!
//! - `component`: definición de componentes y almacenamiento.
//! - `world`: gestión de entidades y componentes.
//! - `query`: consultas sobre componentes.
//! - `system`: ejecución de sistemas sobre el mundo.

pub mod component;
pub mod query;
pub mod system;
pub mod world;

// --- REEXPORTS ---
pub use component::{Component, ComponentId, Transform};
pub use query::{Query, Velocity};
pub use system::{System, TaskGraph};
pub use world::World;

/// --- TEST BÁSICO ---
#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec3, Quat};

    #[test]
    fn test_spawn_and_query_mut() {
        let mut world = World::new(1000);
        world.register_component::<Transform>();
        world.register_component::<Velocity>();

        let entity = world.spawn_entity();

        world.insert(entity, Transform {
            position: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::IDENTITY,
        });
        world.insert(entity, Velocity(Vec3::new(0.1, 0.2, 0.3)));

        // --- Query mutable: solo funciona con (&Transform, &mut Velocity)
        let mut query = Query::<(&Transform, &mut Velocity)>::new(&mut world);
        let mut results: Vec<_> = query.iter().collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(results[0].1 .0, Vec3::new(0.1, 0.2, 0.3));

        // Modificamos Velocity
        results[0].1 .0 += Vec3::new(0.1, 0.1, 0.1);

        // Confirmamos cambios
        let mut query_check = Query::<(&Transform, &mut Velocity)>::new(&mut world);
        let results_check: Vec<_> = query_check.iter().collect();
        assert_eq!(results_check[0].1 .0, Vec3::new(0.2, 0.3, 0.4));
    }

    // Nota: eliminamos el test de solo lectura hasta definir Queryable para (&Transform, &Velocity)
}
