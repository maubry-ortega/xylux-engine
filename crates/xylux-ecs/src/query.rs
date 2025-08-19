//! # Módulo de Queries ECS
//!
//! Infraestructura para consultas sobre entidades y componentes en un mundo ECS.
//! Permite iterar sobre entidades que cumplen ciertos criterios de componentes.

use crate::component::{Component, ComponentId};
use crate::entity::Entity;
use crate::world::World;
use std::marker::PhantomData;

/// Trait que define qué se puede extraer de una `Query`.
/// Implementado para tuplas de `&T` y `&mut T` donde `T: Component`.
pub trait Queryable<'w>: Sized {
    /// Devuelve los `ComponentId` de los componentes de la query.
    fn component_ids() -> Vec<ComponentId>;

    /// Extrae los componentes de una entidad del mundo.
    ///
    /// # Safety
    /// - El puntero `world` debe ser válido.
    /// - La `entity` debe estar viva y tener todos los componentes requeridos.
    ///   Esta condición la garantiza `QueryIter`.
    unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self>;
}

/// Query sobre entidades que cumplen los requisitos de `T: Queryable`.
pub struct Query<'w, T: Queryable<'w>> {
    world: *mut World,
    _lt: PhantomData<&'w mut World>,
    _marker: PhantomData<T>,
}

impl<'w, T: Queryable<'w>> Query<'w, T> {
    /// Crea una nueva query sobre el mundo.
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            _lt: PhantomData,
            _marker: PhantomData,
        }
    }

    /// Devuelve un iterador sobre los componentes solicitados.
    pub fn iter(&mut self) -> QueryIter<'w, T> {
        let component_ids = T::component_ids();

        // OPTIMIZACIÓN: Intersectamos los bitmasks de los componentes para obtener
        // solo las entidades que tienen TODOS los componentes requeridos.
        // Esto es mucho más eficiente que iterar y comprobar cada entidad.
        if !component_ids.is_empty() {
            let world_ref = unsafe { &*self.world };
            let mut final_mask = if let Some(storage) = world_ref.components.get(&component_ids[0])
            {
                storage.bitmask.clone() // Clonamos el primer bitmask para empezar la intersección.
            } else {
                // Si el primer componente no existe, la query no puede devolver nada.
                return QueryIter::empty(self.world);
            };

            for component_id in component_ids.iter().skip(1) {
                let Some(storage) = world_ref.components.get(component_id) else {
                    // Si falta algún storage de un componente requerido, el resultado es vacío.
                    final_mask.clear();
                    break;
                };
                final_mask &= &storage.bitmask;
            }

            // Movemos el bitmask calculado al iterador para que sea lazy.
            return QueryIter::new(self.world, final_mask);
        }

        // Si no se piden componentes (e.g., Query<(Entity,)>), iteramos sobre todas las entidades vivas.
        let world_ref = unsafe { &*self.world };
        let alive_mask = world_ref.alive_mask().clone();
        QueryIter::new(self.world, alive_mask)
    }
}

/// Iterador sobre entidades y sus componentes.
/// Este iterador es "lazy" y no pre-asigna un vector con todas las entidades coincidentes.
pub struct QueryIter<'w, T: Queryable<'w>> {
    world: *mut World,
    mask: bitvec::vec::BitVec,
    cursor: usize,
    _lt: PhantomData<&'w mut World>,
    _marker: PhantomData<T>,
}

impl<'w, T: Queryable<'w>> QueryIter<'w, T> {
    /// Crea un nuevo iterador a partir de un bitmask de entidades coincidentes.
    fn new(world: *mut World, mask: bitvec::vec::BitVec) -> Self {
        Self {
            world,
            mask,
            cursor: 0,
            _lt: PhantomData,
            _marker: PhantomData,
        }
    }

    /// Crea un iterador vacío.
    fn empty(world: *mut World) -> Self {
        Self {
            world,
            mask: bitvec::vec::BitVec::new(),
            cursor: 0,
            _lt: PhantomData,
            _marker: PhantomData,
        }
    }
}

impl<'w, T: Queryable<'w>> Iterator for QueryIter<'w, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Busca el siguiente bit activado desde la posición actual del cursor.
            let next_one = self.mask[self.cursor..].first_one()?;
            let entity_id = self.cursor + next_one;
            self.cursor = entity_id + 1; // Mueve el cursor para la siguiente búsqueda.

            let entity = Entity {
                id: entity_id,
                version: unsafe { (*self.world).entity_version(entity_id) },
            };

            // Verificamos que la entidad siga viva antes de hacer fetch.
            // Podría haber sido eliminada por una iteración anterior del mismo sistema.
            if unsafe { (*self.world).is_alive(entity) } {
                // SAFETY: El iterador se construye con entidades que tienen todos los
                // componentes necesarios. El puntero al mundo es válido durante la vida del
                // iterador. La API de `Query` con `&'w mut World` previene la creación
                // de múltiples iteradores mutables que podrían invalidar las referencias.
                // La comprobación `is_alive` añade una capa extra de seguridad.
                return unsafe { T::fetch(self.world, entity) };
            }
        }
    }
}

// --- Implementación de Queryable para tuplas ---

/// Trait auxiliar para abstraer sobre `&T` y `&mut T` en las queries.
///
/// # Safety
/// La implementación de este trait es `unsafe` porque debe garantizar que
/// el acceso a los componentes a través de `fetch_param` es válido bajo
/// las reglas de borrowing de Rust, aunque se use un puntero crudo.
pub unsafe trait QueryParam<'w>: Sized {
    type Item;

    /// Añade los `ComponentId` requeridos por este parámetro a la lista.
    /// No hace nada si el parámetro no es un componente (e.g., `Entity`).
    fn add_component_ids(ids: &mut Vec<ComponentId>);

    /// # Safety
    /// El puntero `world` debe ser válido y la `entity` debe estar viva.
    unsafe fn fetch_param(world: *mut World, entity: Entity) -> Option<Self::Item>;
}

unsafe impl<'w, C: Component> QueryParam<'w> for &'w C {
    type Item = &'w C;

    fn add_component_ids(ids: &mut Vec<ComponentId>) {
        ids.push(ComponentId::of::<C>());
    }

    unsafe fn fetch_param(world: *mut World, entity: Entity) -> Option<Self::Item> {
        // SAFETY: The caller of `fetch_param` guarantees that `world` is a valid
        // pointer and that `entity` is alive.
        unsafe { (*world).get(entity) }
    }
}

unsafe impl<'w, C: Component> QueryParam<'w> for &'w mut C {
    type Item = &'w mut C;

    fn add_component_ids(ids: &mut Vec<ComponentId>) {
        ids.push(ComponentId::of::<C>());
    }

    unsafe fn fetch_param(world: *mut World, entity: Entity) -> Option<Self::Item> {
        // SAFETY: The caller of `fetch_param` guarantees that `world` is a valid
        // pointer and that `entity` is alive. It also ensures aliasing rules
        // for mutable access are not violated.
        unsafe { (*world).get_mut(entity) }
    }
}

// Implementación para obtener el `Entity` mismo en una query.
unsafe impl<'w> QueryParam<'w> for Entity {
    type Item = Entity;

    fn add_component_ids(_ids: &mut Vec<ComponentId>) {
        // Entity no es un componente, no añade IDs.
    }

    unsafe fn fetch_param(_world: *mut World, entity: Entity) -> Option<Self::Item> {
        Some(entity)
    }
}

macro_rules! impl_queryable_for_tuple {
    ( $($param:ident),* ) => {
        #[allow(non_snake_case)]
        impl<'w, $($param),*> Queryable<'w> for ($($param,)*)
        where
            $($param: QueryParam<'w, Item = $param> + 'w),*
        {
            fn component_ids() -> Vec<ComponentId> {
                let mut ids = Vec::new();
                $( $param::add_component_ids(&mut ids); )*
                ids
            }

            unsafe fn fetch(world: *mut World, entity: Entity) -> Option<Self> {
                // SAFETY: This function is unsafe and relies on the caller (QueryIter)
                // to provide a valid world pointer and an entity that is alive and
                // has all the required components. The individual `fetch_param` calls
                // are also unsafe.
                unsafe {
                    $(
                        let $param = $param::fetch_param(world, entity)?;
                    )*
                    Some(($($param,)*))
                }
            }
        }
    };
}

impl_queryable_for_tuple!(P1);
impl_queryable_for_tuple!(P1, P2);
impl_queryable_for_tuple!(P1, P2, P3);
impl_queryable_for_tuple!(P1, P2, P3, P4);
// Se pueden añadir más tuplas si es necesario.
