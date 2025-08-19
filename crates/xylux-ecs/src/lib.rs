//! # Xylux ECS
//!
//! Este crate proporciona un motor ECS (Entity-Component-System) minimalista, eficiente y
//! diseñado con principios de Data-Oriented Design (DOD).
//!
//! ## Arquitectura y Diseño
//!
//! 1.  **Struct of Arrays (SoA)**: A diferencia de un enfoque de Array of Structs (AoS),
//!     los componentes se almacenan en `Vec`s contiguos por tipo. Por ejemplo, todas
//!     las `Position` están juntas en memoria. Esto mejora drásticamente el rendimiento
//!     de las iteraciones al maximizar el uso de la caché de la CPU.
//! 2.  **Generational Indices para Entidades**: Para manejar de forma segura la creación
//!     y destrucción de entidades, cada `Entity` tiene un `id` y una `version`. Cuando
//!     una entidad se elimina, su `id` puede ser reciclado, pero su `version` se incrementa.
//!     Esto previene el "problema del ABA", donde una referencia antigua podría acceder
//!     accidentalmente a una nueva entidad que reutilizó el mismo `id`.
//! 3.  **Queries Eficientes con Bitmasks**: Las consultas (`Query`) se resuelven calculando
//!     la intersección de `BitVec`s (bitmasks). Cada tipo de componente tiene un bitmask
//!     que indica qué entidades lo poseen. Esto permite filtrar millones de entidades
//!     en microsegundos, antes de acceder a los datos de los componentes.

pub mod component;
pub mod entity;
pub mod query;
pub mod system;
pub mod world;

// --- REEXPORTS ---
pub use component::{Component, ComponentId, Transform, Velocity};
pub use entity::Entity;
pub use query::Query;
pub use system::{System, TaskGraph};
pub use world::World;

/// --- TEST BÁSICO ---
#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Quat, Vec3};

    // Componente de prueba
    #[derive(Default, Debug, PartialEq, Clone, Copy)]
    struct Tag(u32);
    impl Component for Tag {}

    #[test]
    fn test_spawn_and_query_mut() {
        let mut world = World::new(1000);
        world.register_component::<Transform>();
        world.register_component::<Velocity>();

        let entity = world.spawn_entity();

        world.insert(
            entity,
            Transform {
                position: Vec3::new(1.0, 2.0, 3.0),
                rotation: Quat::IDENTITY,
            },
        );
        world.insert(entity, Velocity(Vec3::new(0.1, 0.2, 0.3)));

        // --- Query mutable: (&Transform, &mut Velocity)
        let mut query = Query::<(&Transform, &mut Velocity)>::new(&mut world);
        let mut found = false;
        for (transform, velocity) in query.iter() {
            assert_eq!(transform.position, Vec3::new(1.0, 2.0, 3.0));
            assert_eq!(velocity.0, Vec3::new(0.1, 0.2, 0.3));
            // Modificamos Velocity
            velocity.0 += Vec3::new(0.1, 0.1, 0.1);
            found = true;
        }
        assert!(found, "La query no encontró la entidad");

        // Confirmamos los cambios en una nueva query
        let mut query_check = Query::<(&Transform, &Velocity)>::new(&mut world);
        let (_transform, velocity) = query_check.iter().next().unwrap();
        assert_eq!(velocity.0, Vec3::new(0.2, 0.3, 0.4));
    }

    #[test]
    fn test_despawn_and_recycle() {
        let mut world = World::new(10);
        world.register_component::<Tag>();

        let e1 = world.spawn_entity();
        world.insert(e1, Tag(1));
        let e2 = world.spawn_entity();
        world.insert(e2, Tag(2));

        assert_eq!(Query::<(&Tag,)>::new(&mut world).iter().count(), 2);

        world.despawn_entity(e1);
        assert!(!world.is_alive(e1));

        // La query ya no debería encontrar e1
        let results: Vec<_> = Query::<(&Tag,)>::new(&mut world).iter().collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0 .0, 2);

        // Al crear una nueva entidad, debería reciclar el ID de e1
        let e3 = world.spawn_entity();
        assert_eq!(e1.id, e3.id);
        assert_ne!(e1.version, e3.version);

        world.insert(e3, Tag(3));
        assert_eq!(Query::<(&Tag,)>::new(&mut world).iter().count(), 2);
    }

    #[test]
    fn test_query_with_entity() {
        let mut world = World::new(10);
        world.register_component::<Transform>();

        let e1 = world.spawn_entity();
        world.insert(e1, Transform::default());
        let e2 = world.spawn_entity();
        world.insert(e2, Transform::default());

        let mut query = Query::<(Entity, &Transform)>::new(&mut world);
        let mut results: Vec<(Entity, &Transform)> = query.iter().collect();
        results.sort_by_key(|(e, _)| e.id);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, e1);
        assert_eq!(results[1].0, e2);
    }
}
