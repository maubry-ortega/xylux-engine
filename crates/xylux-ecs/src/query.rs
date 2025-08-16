use crate::component::{Component, ComponentId, Transform};
use crate::world::{World, Entity};
use std::marker::PhantomData;

/// --- Componente de ejemplo ---
#[derive(Clone, Copy, Default)]
pub struct Velocity(pub glam::Vec3);
impl Component for Velocity {}

/// --- TRAIT QUERYABLE ---
pub trait Queryable<'w>: Sized {
    fn component_ids() -> Vec<ComponentId>;

    /// Extrae el conjunto de componentes de una entidad si existe y está viva.
    unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self>;
}

/// --- QUERY ---
pub struct Query<'w, T: Queryable<'w>> {
    world: *mut World,
    _lt: PhantomData<&'w mut World>,
    _marker: PhantomData<T>,
}

impl<'w, T: Queryable<'w>> Query<'w, T> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            _lt: PhantomData,
            _marker: PhantomData,
        }
    }

    pub fn iter(&mut self) -> QueryIter<'w, T> {
        let count = unsafe { (*self.world).entity_count() };
        let mut entities = Vec::new();

        for id in 0..count {
            let version = unsafe { (*self.world).entity_version(id) };
            let entity = Entity { id, version };

            // Bloque unsafe explícito aquí
            let has_components = unsafe { <T as Queryable>::fetch(self.world, entity) }.is_some();
            if has_components {
                entities.push(entity);
            }
        }

        QueryIter {
            world: self.world,
            entities,
            index: 0,
            _lt: PhantomData,
            _marker: PhantomData,
        }
    }
}

/// --- QUERY ITER ---
pub struct QueryIter<'w, T: Queryable<'w>> {
    world: *mut World,
    entities: Vec<Entity>,
    index: usize,
    _lt: PhantomData<&'w mut World>,
    _marker: PhantomData<T>,
}

impl<'w, T: Queryable<'w>> Iterator for QueryIter<'w, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.entities.len() {
            return None;
        }
        let entity = self.entities[self.index];
        self.index += 1;

        unsafe { <T as Queryable>::fetch(self.world, entity) }
    }
}

/// --- IMPLEMENTACIONES DE QUERYABLE ---
impl<'w> Queryable<'w> for &'w Transform {
    fn component_ids() -> Vec<ComponentId> {
        vec![ComponentId::of::<Transform>()]
    }

    unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self> {
        if !unsafe { (*world).is_alive(entity) } {
            return None;
        }
        unsafe { (*world).get::<Transform>(entity) }
    }
}

impl<'w> Queryable<'w> for (&'w Transform, &'w mut Velocity) {
    fn component_ids() -> Vec<ComponentId> {
        vec![
            ComponentId::of::<Transform>(),
            ComponentId::of::<Velocity>(),
        ]
    }

    unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self> {
        if !unsafe { (*world).is_alive(entity) } {
            return None;
        }
        Some((
            unsafe { (*world).get::<Transform>(entity)? },
            unsafe { (*world).get_mut::<Velocity>(entity)? },
        ))
    }
}
