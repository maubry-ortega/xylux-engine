//! # Módulo de componentes ECS
//!
//! Este módulo define la infraestructura base para manejar **componentes** en un motor ECS
//! (Entity Component System).
//!
//! - `ComponentId`: Identificador único para cada tipo de componente.
//! - `Component`: Trait que deben implementar todos los componentes.
//! - `ComponentStorage`: Contenedor genérico para almacenar componentes por entidad.
//! - `Transform`: Componente de ejemplo (posición y rotación).

use glam::{Vec3, Quat};
use std::any::{Any, TypeId};
use bitvec::prelude::*;

/// Identificador único para cada tipo de componente.
/// Internamente usa `TypeId` para distinguir tipos en tiempo de ejecución.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComponentId(TypeId);

impl ComponentId {
    /// Obtiene el `ComponentId` único para el tipo `T`.
    pub fn of<T: 'static>() -> Self {
        ComponentId(TypeId::of::<T>())
    }
}

/// Trait que deben implementar todos los componentes del ECS.
///
/// Generalmente no requiere métodos adicionales, pero proporciona
/// un método estático para obtener el `ComponentId` asociado al tipo.
pub trait Component: 'static {
    /// Obtiene el identificador único del tipo de componente.
    fn component_id() -> ComponentId where Self: Sized {
        ComponentId::of::<Self>()
    }
}

/// Almacenamiento genérico para **un solo tipo de componente**.
///
/// Internamente almacena:
/// - Un `Vec<T>` con los datos del componente.
/// - Un `BitVec` para indicar qué entidades tienen este componente.
///
/// > Nota: este almacenamiento es **SoA (Structure of Arrays)**, optimizado para iteración.
pub struct ComponentStorage {
    data: Vec<Box<dyn Any>>, // siempre un solo Vec<T> dentro
    bitmask: BitVec,         // Indica qué entidades tienen este componente
}

impl ComponentStorage {
    /// Crea un nuevo almacenamiento para un tipo de componente `T`.
    ///
    /// - `capacity`: número máximo de entidades que podrá manejar.
    pub fn new<T: Component>(capacity: usize) -> Self {
        Self {
            data: vec![Box::new(Vec::<T>::with_capacity(capacity)) as Box<dyn Any>],
            bitmask: bitvec![0; capacity], // Inicializa todos los bits en 0
        }
    }

    /// Inserta un componente `T` en la entidad especificada.
    ///
    /// - Si la entidad excede el tamaño actual del `Vec`, este se expande.
    /// - Marca el bit correspondiente como activo.
    pub fn insert<T: Component + Default>(&mut self, entity: usize, component: T) {
        let vec = self.data[0]
            .downcast_mut::<Vec<T>>()
            .expect("Error: tipo incorrecto en ComponentStorage");

        if entity >= vec.len() {
            vec.resize_with(entity + 1, Default::default);
        }
        vec[entity] = component;

        if entity >= self.bitmask.len() {
            self.bitmask.resize(entity + 1, false);
        }
        self.bitmask.set(entity, true);
    }

    /// Elimina un componente de la entidad especificada.
    pub fn remove(&mut self, entity: usize) {
        if entity < self.bitmask.len() {
            self.bitmask.set(entity, false);
            // Nota: No eliminamos físicamente el dato en Vec<T> para evitar mover elementos.
            //       Simplemente lo marcamos como inactivo usando bitmask.
        }
    }

    /// Obtiene una referencia inmutable al componente de la entidad.
    ///
    /// Retorna `None` si la entidad no tiene este componente.
    pub fn get<T: Component>(&self, entity: usize) -> Option<&T> {
        if !self.has(entity) {
            return None;
        }

        self.data[0]
            .downcast_ref::<Vec<T>>()
            .expect("Error: tipo incorrecto en ComponentStorage")
            .get(entity)
    }

    /// Obtiene una referencia mutable al componente de la entidad.
    ///
    /// Retorna `None` si la entidad no tiene este componente.
    pub fn get_mut<T: Component>(&mut self, entity: usize) -> Option<&mut T> {
        if !self.has(entity) {
            return None;
        }

        self.data[0]
            .downcast_mut::<Vec<T>>()
            .expect("Error: tipo incorrecto en ComponentStorage")
            .get_mut(entity)
    }

    /// Verifica si la entidad tiene este componente.
    pub fn has(&self, entity: usize) -> bool {
        entity < self.bitmask.len() && self.bitmask[entity]
    }
}

/// Componente de ejemplo: Transformación 3D.
///
/// Contiene posición y rotación de una entidad.
#[derive(Clone, Copy, Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Component for Transform {}
