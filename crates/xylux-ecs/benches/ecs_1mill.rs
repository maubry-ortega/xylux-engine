use criterion::{criterion_group, criterion_main, Criterion};
use glam::Vec3;
use std::hint::black_box; // <-- Usamos la versión recomendada de Rust estándar
use xylux_ecs::{Component, Query, Transform, Velocity, World};

#[derive(Default, Clone, Copy)]
struct CompA;
impl Component for CompA {}

#[derive(Default, Clone, Copy)]
struct CompB;
impl Component for CompB {}

const ENTITY_COUNT: usize = 1_000_000;

fn setup_world() -> World {
    let mut world = World::new(ENTITY_COUNT + 100);
    world.register_component::<Transform>();
    world.register_component::<Velocity>();
    world.register_component::<CompA>();
    world.register_component::<CompB>();

    for i in 0..ENTITY_COUNT {
        let entity = world.spawn_entity();
        world.insert(entity, Transform::default());
        if i % 2 == 0 {
            world.insert(entity, Velocity(Vec3::ZERO));
        }
        if i % 3 == 0 {
            world.insert(entity, CompA);
        }
        if i % 5 == 0 {
            world.insert(entity, CompB);
        }
    }
    world
}

fn query_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Queries 1M Entities");

    // Query con 1 componente
    group.bench_function("Query 1 Component (Transform)", |b| {
        let mut world = setup_world();
        b.iter(|| {
            let mut query = Query::<(&Transform,)>::new(&mut world);
            for transform in query.iter() {
                black_box(transform); // <- std::hint::black_box
            }
        })
    });

    // Query con 2 componentes
    group.bench_function("Query 2 Components (Transform, Velocity)", |b| {
        let mut world = setup_world();
        b.iter(|| {
            let mut query = Query::<(&Transform, &Velocity)>::new(&mut world);
            for (t, v) in query.iter() {
                black_box((t, v)); // <- std::hint::black_box
            }
        })
    });

    // Query con 3 componentes
    group.bench_function("Query 3 Components (Transform, CompA, CompB)", |b| {
        let mut world = setup_world();
        b.iter(|| {
            let mut query = Query::<(&Transform, &CompA, &CompB)>::new(&mut world);
            for (t, a, b) in query.iter() {
                black_box((t, a, b)); // <- std::hint::black_box
            }
        })
    });

    group.finish();
}

criterion_group!(benches, query_benchmark);
criterion_main!(benches);
