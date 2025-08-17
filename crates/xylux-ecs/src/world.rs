//! # Módulo de World ECS Profesional
//!
//! Define el **mundo ECS**, responsable de gestionar entidades y componentes.
//! Se centra en **single responsibility**, **rendimiento SoA**, y **paralelismo seguro**.
//!
//! Contiene:
//! - `Entity`: representa una entidad única con control de versiones.
//! - `World`: administra entidades, componentes y acceso seguro a datos.

use crate::component::{Component, ComponentId, ComponentStorage};
use std::collections::HashMap;

/// Representa una entidad única en el ECS.
///
/// `(id, version)` evita accesos a entidades recicladas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: usize,
    pub version: u32,
}

/// Contenedor principal del ECS.
///
/// Gestiona entidades, versiones y componentes usando almacenamiento **SoA**.
pub struct World {
    max_entities: usize,
    entity_count: usize,
    components: HashMap<ComponentId, ComponentStorage>,
    entity_versions: Vec<u32>,
    free_entities: Vec<usize>,
}

impl World {
    /// Crea un nuevo mundo ECS con capacidad máxima `max_entities`.
    pub fn new(max_entities: usize) -> Self {
        Self {
            max_entities,
            entity_count: 0,
            components: HashMap::new(),
            entity_versions: vec![0; max_entities],
            free_entities: Vec::new(),
        }
    }

    /// Genera una nueva entidad.
    ///
    /// Reutiliza IDs libres si los hay, sino incrementa `entity_count`.
    pub fn spawn_entity(&mut self) -> Entity {
        let id = self.free_entities.pop().unwrap_or_else(|| {
            if self.entity_count >= self.max_entities {
                panic!("Max entities reached");
            }
            let id = self.entity_count;
            self.entity_count += 1;
            id
        });

        Entity {
            id,
            version: self.entity_versions[id],
        }
    }

    /// Elimina una entidad y sus componentes.
    ///
    /// Incrementa la versión para invalidar referencias antiguas.
    pub fn despawn_entity(&mut self, entity: Entity) {
        if !self.is_alive(entity) {
            return;
        }

        self.entity_versions[entity.id] = self.entity_versions[entity.id].wrapping_add(1);
        self.free_entities.push(entity.id);

        self.components.values_mut().for_each(|storage| storage.remove(entity.id));
    }

    /// Registra un nuevo tipo de componente en el mundo.
    ///
    /// Crea un `ComponentStorage` dedicado con capacidad `max_entities`.
    pub fn register_component<T: Component>(&mut self) {
        let id = ComponentId::of::<T>();
        self.components
            .entry(id)
            .or_insert_with(|| ComponentStorage::new::<T>(self.max_entities));
    }

    /// Inserta un componente `T` en una entidad específica.
    ///
    /// # Panics
    /// - Si la entidad no está viva.
    /// - Si el componente no ha sido registrado.
    pub fn insert<T: Component + Default>(&mut self, entity: Entity, component: T) {
        if !self.is_alive(entity) {
            panic!("Intento de insertar componente en entidad inválida");
        }

        let id = ComponentId::of::<T>();
        self.components
            .get_mut(&id)
            .expect("Componente no registrado")
            .insert(entity.id, component);
    }

    /// Obtiene una referencia inmutable al componente `T` de una entidad.
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        if !self.is_alive(entity) {
            return None;
        }
        self.components
            .get(&ComponentId::of::<T>())
            .and_then(|storage| storage.get(entity.id))
    }

    /// Obtiene una referencia mutable al componente `T` de una entidad.
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.is_alive(entity) {
            return None;
        }
        self.components
            .get_mut(&ComponentId::of::<T>())
            .and_then(|storage| storage.get_mut(entity.id))
    }

    /// Devuelve la versión actual de una entidad por ID.
    pub fn entity_version(&self, entity_id: usize) -> u32 {
        self.entity_versions[entity_id]
    }

    /// Comprueba si una entidad sigue viva.
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entity_versions[entity.id] == entity.version
    }

    /// Retorna la cantidad total de entidades creadas (incluye huecos reciclados).
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Retorna la capacidad máxima de entidades del mundo.
    pub fn capacity(&self) -> usize {
        self.max_entities
    }
}
