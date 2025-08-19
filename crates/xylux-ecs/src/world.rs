//! # Módulo World
//!
//! Define el `World`, el contenedor principal del ECS que gestiona
//! todas las entidades, componentes y sus ciclos de vida.

use crate::component::{Component, ComponentId, ComponentStorage};
use crate::entity::Entity;
use bitvec::prelude::*;
use std::collections::HashMap;

/// Contenedor principal del ECS.
///
/// Gestiona entidades, versiones y componentes usando almacenamiento **SoA**.
pub struct World {
    max_entities: usize,
    entity_count: usize,
    pub(crate) components: HashMap<ComponentId, ComponentStorage>,
    entity_versions: Vec<u32>,
    free_entities: Vec<usize>,
    alive_mask: BitVec,
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
            alive_mask: bitvec![0; max_entities],
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

        self.alive_mask.set(id, true);

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
        self.alive_mask.set(entity.id, false);

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
    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
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

    /// Comprueba si una entidad tiene un componente específico por ID de componente.
    pub fn has_component(&self, entity: Entity, component_id: ComponentId) -> bool {
        if !self.is_alive(entity) {
            return false;
        }
        self.components
            .get(&component_id)
            .map_or(false, |storage| storage.has(entity.id))
    }

    /// Devuelve una colección de entidades que tienen un componente específico.
    ///
    /// Internamente, itera sobre el `bitmask` del `ComponentStorage` para encontrar
    /// eficientemente todas las entidades con el componente.
    pub fn entities_with_component(&self, component_id: ComponentId) -> Option<Vec<Entity>> {
        self.components.get(&component_id).map(|storage| {
            storage.bitmask.iter_ones()
                .map(|id| Entity { id, version: self.entity_versions[id] })
                .collect()
        })
    }

    /// Devuelve un bitmask de todas las entidades vivas.
    pub(crate) fn alive_mask(&self) -> &BitVec {
        &self.alive_mask
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
