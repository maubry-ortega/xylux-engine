use crate::component::{Component, ComponentId, ComponentStorage};
use std::collections::HashMap;

/// Representa una entidad única en el mundo ECS.
/// El par `(id, version)` asegura que no se acceda a entidades recicladas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: usize,
    pub version: u32,
}

/// Representa el mundo ECS que maneja entidades y componentes.
/// Incluye control de versiones para evitar acceder a entidades inválidas.
pub struct World {
    entity_count: usize,
    max_entities: usize,
    components: HashMap<ComponentId, ComponentStorage>,
    entity_versions: Vec<u32>,       // versión por entidad
    free_entities: Vec<usize>,       // IDs libres para reutilizar
}

impl World {
    /// Crea un nuevo mundo con capacidad máxima `max_entities`.
    pub fn new(max_entities: usize) -> Self {
        Self {
            entity_count: 0,
            max_entities,
            components: HashMap::new(),
            entity_versions: vec![0; max_entities],
            free_entities: Vec::new(),
        }
    }

    /// Crea una entidad y devuelve su `Entity` con id y versión.
    pub fn spawn_entity(&mut self) -> Entity {
        let id = if let Some(free) = self.free_entities.pop() {
            free
        } else {
            if self.entity_count >= self.max_entities {
                panic!("Max entities reached");
            }
            let id = self.entity_count;
            self.entity_count += 1;
            id
        };
        let version = self.entity_versions[id];
        Entity { id, version }
    }

    /// Elimina una entidad y aumenta su versión para invalidar referencias antiguas.
    pub fn despawn_entity(&mut self, entity: Entity) {
        // Validar que sigue viva antes de despawn
        if !self.is_alive(entity) {
            return; // o panic!("Entidad inválida"), según lo que prefieras
        }

        self.entity_versions[entity.id] = self.entity_versions[entity.id].wrapping_add(1);
        self.free_entities.push(entity.id);

        // Limpia componentes asociados
        for storage in self.components.values_mut() {
            storage.remove(entity.id);
        }
    }

    /// Registra un tipo de componente para permitir insertarlo en entidades.
    pub fn register_component<T: Component>(&mut self) {
        let id = ComponentId::of::<T>();
        self.components
            .entry(id)
            .or_insert_with(|| ComponentStorage::new::<T>(self.max_entities));
    }

    /// Inserta un componente `T` en una entidad, validando que esté viva.
    pub fn insert<T: Component + Default>(&mut self, entity: Entity, component: T) {
        if !self.is_alive(entity) {
            panic!("Intento de insertar en una entidad inválida");
        }

        let id = ComponentId::of::<T>();
        if let Some(storage) = self.components.get_mut(&id) {
            storage.insert(entity.id, component);
        } else {
            panic!("Componente no registrado: {:?}", id);
        }
    }

    /// Obtiene una referencia inmutable a un componente si la entidad sigue viva.
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        if !self.is_alive(entity) {
            return None;
        }
        self.components
            .get(&ComponentId::of::<T>())
            .and_then(|storage| storage.get(entity.id))
    }

    /// Obtiene una referencia mutable a un componente si la entidad sigue viva.
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.is_alive(entity) {
            return None;
        }
        self.components
            .get_mut(&ComponentId::of::<T>())
            .and_then(|storage| storage.get_mut(entity.id))
    }

    /// Devuelve la versión actual de una entidad según su ID.
    pub fn entity_version(&self, entity_id: usize) -> u32 {
        self.entity_versions[entity_id]
    }

    /// Comprueba si una entidad sigue viva (versión coincide).
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entity_versions[entity.id] == entity.version
    }

    /// Devuelve la cantidad actual de entidades creadas (incluye huecos).
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }
}
