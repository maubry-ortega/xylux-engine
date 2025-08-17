//! # Módulo de Queries ECS
//!
//! Infraestructura para consultas sobre entidades y componentes en un mundo ECS.
//! Permite iterar sobre entidades que cumplen ciertos criterios de componentes.

use crate::component::{Component, ComponentId, Transform};
use crate::world::{World, Entity};
use std::marker::PhantomData;

/// Componente de ejemplo: Velocidad de una entidad
#[derive(Clone, Copy, Default)]
pub struct Velocity(pub glam::Vec3);

impl Component for Velocity {}

/// Trait que define qué componentes se pueden extraer de una entidad.
pub trait Queryable<'w>: Sized {
    fn component_ids() -> Vec<ComponentId>;

    /// Extrae los componentes de una entidad del mundo.
    ///
    /// # Safety
    /// Requiere un puntero válido a `World` y que la entidad sea válida.
    unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self>;
}

/// Query sobre entidades que cumplen los requisitos de `T: Queryable`.
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
        let mut entities = Vec::with_capacity(count);

        for id in 0..count {
            let entity = Entity {
                id,
                version: unsafe { (*self.world).entity_version(id) },
            };

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

/// Iterador sobre entidades y sus componentes.
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

/// Implementación de Queryable para referencia inmutable de Transform
impl<'w> Queryable<'w> for &'w Transform {
    fn component_ids() -> Vec<ComponentId> {
        vec![ComponentId::of::<Transform>()]
    }

    unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self> {
        if !(unsafe { (*world).is_alive(entity) }) {
            return None;
        }
        unsafe { (*world).get::<Transform>(entity) }
    }
}

/// Implementación de Queryable para referencia a Transform y mutable a Velocity
impl<'w> Queryable<'w> for (&'w Transform, &'w mut Velocity) {
    fn component_ids() -> Vec<ComponentId> {
        vec![ComponentId::of::<Transform>(), ComponentId::of::<Velocity>()]
    }

    unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self> {
        if !(unsafe { (*world).is_alive(entity) }) {
            return None;
        }

        Some((
            unsafe { (*world).get::<Transform>(entity)? },
            unsafe { (*world).get_mut::<Velocity>(entity)? },
        ))
    }
}
