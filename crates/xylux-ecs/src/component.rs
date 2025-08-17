//! # Módulo de componentes ECS
//!
//! Define la infraestructura base para manejar **componentes** en un motor ECS
//! (Entity Component System). Contiene:
//! - `ComponentId`: Identificador único de tipo de componente.
//! - `Component`: Trait que deben implementar todos los componentes.
//! - `ComponentStorage`: Contenedor genérico de componentes por entidad.
//! - `Transform`: Componente de ejemplo con posición y rotación.

use glam::{Vec3, Quat};
use std::any::{Any, TypeId};
use bitvec::prelude::*;

/// Identificador único para cada tipo de componente.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComponentId(TypeId);

impl ComponentId {
    /// Obtiene el `ComponentId` único para el tipo `T`.
    pub fn of<T: 'static>() -> Self {
        ComponentId(TypeId::of::<T>())
    }
}

/// Trait que deben implementar todos los componentes ECS.
pub trait Component: 'static + Default {
    /// Retorna el identificador único del tipo de componente.
    fn component_id() -> ComponentId
    where
        Self: Sized,
    {
        ComponentId::of::<Self>()
    }
}

/// Almacenamiento genérico para un solo tipo de componente.
pub struct ComponentStorage {
    data: Box<dyn Any>,
    bitmask: BitVec,
}

impl ComponentStorage {
    /// Crea un nuevo almacenamiento para `T` con capacidad `capacity`.
    pub fn new<T: Component>(capacity: usize) -> Self {
        let mut vec: Vec<T> = Vec::with_capacity(capacity);
        vec.resize_with(capacity, Default::default);

        Self {
            data: Box::new(vec),
            bitmask: bitvec![0; capacity],
        }
    }

    /// Inserta un componente `T` en la entidad indicada.
    pub fn insert<T: Component>(&mut self, entity: usize, component: T) {
        let vec = self
            .data
            .downcast_mut::<Vec<T>>()
            .expect("tipo incorrecto en ComponentStorage");

        if entity >= vec.len() {
            vec.resize_with(entity + 1, Default::default);
        }
        vec[entity] = component;

        if entity >= self.bitmask.len() {
            self.bitmask.resize(entity + 1, false);
        }
        self.bitmask.set(entity, true);
    }

    /// Elimina un componente de la entidad indicada.
    pub fn remove(&mut self, entity: usize) {
        if entity < self.bitmask.len() {
            self.bitmask.set(entity, false);
        }
    }

    /// Obtiene referencia inmutable al componente de la entidad.
    pub fn get<T: Component>(&self, entity: usize) -> Option<&T> {
        if !self.has(entity) {
            return None;
        }

        self.data
            .downcast_ref::<Vec<T>>()
            .expect("tipo incorrecto en ComponentStorage")
            .get(entity)
    }

    /// Obtiene referencia mutable al componente de la entidad.
    pub fn get_mut<T: Component>(&mut self, entity: usize) -> Option<&mut T> {
        if !self.has(entity) {
            return None;
        }

        self.data
            .downcast_mut::<Vec<T>>()
            .expect("tipo incorrecto en ComponentStorage")
            .get_mut(entity)
    }

    /// Verifica si la entidad tiene este componente.
    pub fn has(&self, entity: usize) -> bool {
        entity < self.bitmask.len() && self.bitmask[entity]
    }
}

/// Componente de ejemplo: Transformación 3D de una entidad.
#[derive(Clone, Copy, Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Component for Transform {}
