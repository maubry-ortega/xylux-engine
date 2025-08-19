//! Define el almacenamiento de componentes (SoA).

use super::Component;
use bitvec::prelude::*;
use std::any::Any;

/// Almacenamiento genérico para un solo tipo de componente (SoA).
///
/// Mantiene un `Vec<T>` para los datos y un `BitVec` para rastrear
/// qué entidades poseen el componente, permitiendo iteraciones rápidas.
pub struct ComponentStorage {
    data: Box<dyn Any>,
    pub(crate) bitmask: BitVec,
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
            .expect("Tipo incorrecto en ComponentStorage::insert");

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
            .expect("Tipo incorrecto en ComponentStorage::get")
            .get(entity)
    }

    /// Obtiene referencia mutable al componente de la entidad.
    pub fn get_mut<T: Component>(&mut self, entity: usize) -> Option<&mut T> {
        if !self.has(entity) {
            return None;
        }

        self.data
            .downcast_mut::<Vec<T>>()
            .expect("Tipo incorrecto en ComponentStorage::get_mut")
            .get_mut(entity)
    }

    /// Verifica si la entidad tiene este componente.
    pub fn has(&self, entity: usize) -> bool {
        entity < self.bitmask.len() && self.bitmask[entity]
    }
}