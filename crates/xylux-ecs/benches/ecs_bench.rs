use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use xylux_ecs::{Transform, Velocity, World, Query};

/// Benchmark del ECS usando 10.000 entidades con Transform y Velocity.
fn bench_ecs(c: &mut Criterion) {
    // Creamos el mundo con capacidad para 10_000 entidades
    let mut world = World::new(10_000);
    world.register_component::<Transform>();
    world.register_component::<Velocity>();

    // Spawn de 10.000 entidades y asignación de componentes
    for i in 0..10_000 {
        let entity = world.spawn_entity();
        world.insert(entity, Transform {
            position: glam::Vec3::new(i as f32, 0.0, 0.0),
            ..Default::default()
        });
        world.insert(entity, Velocity(glam::Vec3::new(0.1, 0.0, 0.0)));
    }

    // Benchmark principal
    c.bench_function("query_10k_entities", |b| {
        b.iter(|| {
            // Query mutable, compatible con la implementación de Queryable
            let mut query = Query::<(&Transform, &mut Velocity)>::new(black_box(&mut world));

            for (transform, velocity) in query.iter() {
                // Usamos black_box para evitar optimizaciones que eliminen código
                black_box(transform);
                black_box(velocity);
            }
        });
    });
}

// Configuración de Criterion
criterion_group!(benches, bench_ecs);
criterion_main!(benches);
