pub mod component;
pub mod query;
pub mod system;
pub mod world;

pub use component::{Component, ComponentId, Transform};
pub use query::{Query, Velocity};   // reexportamos Velocity para que esté disponible
pub use system::{System, TaskGraph};
pub use world::World;

#[test]
fn test_spawn_and_query() {
    let mut world = World::new(1000);
    world.register_component::<Transform>();
    world.register_component::<Velocity>();

    // CORREGIDO: spawn_entity devuelve Entity
    let entity = world.spawn_entity();

    world.insert(entity, Transform {
        position: glam::Vec3::new(1.0, 2.0, 3.0),
        rotation: glam::Quat::IDENTITY,
    });
    world.insert(entity, Velocity(glam::Vec3::new(0.1, 0.2, 0.3)));

    // Query mutable porque iter() pide &mut self
    let mut query = Query::<(&Transform, &mut Velocity)>::new(&mut world);
    let results: Vec<_> = query.iter().collect();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0.position, glam::Vec3::new(1.0, 2.0, 3.0));
}
