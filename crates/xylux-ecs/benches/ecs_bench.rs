use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use xylux_ecs::{Transform, Velocity, World, Query};

fn bench_ecs(c: &mut Criterion) {
    let mut world = World::new(10_000);
    world.register_component::<Transform>();
    world.register_component::<Velocity>();

    for i in 0..10_000 {
        let entity = world.spawn_entity();
        world.insert(entity, Transform {
            position: glam::Vec3::new(i as f32, 0.0, 0.0),
            ..Default::default()
        });
        world.insert(entity, Velocity(glam::Vec3::new(0.1, 0.0, 0.0)));
    }

    c.bench_function("query_10k_entities", |b| {
        b.iter(|| {
            // Pasa mutable directamente
            let mut query = Query::<(&Transform, &mut Velocity)>::new(black_box(&mut world));
            for (transform, velocity) in query.iter() {
                black_box(transform);
                black_box(velocity);
            }
        });
    });
}

criterion_group!(benches, bench_ecs);
criterion_main!(benches);
