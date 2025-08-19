//! Define el identificador único de un tipo de componente.

use std::any::TypeId;

/// Identificador único para cada tipo de componente, basado en `TypeId`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComponentId(TypeId);

impl ComponentId {
    /// Obtiene el `ComponentId` único para el tipo `T`.
    pub fn of<T: 'static>() -> Self {
        ComponentId(TypeId::of::<T>())
    }
}