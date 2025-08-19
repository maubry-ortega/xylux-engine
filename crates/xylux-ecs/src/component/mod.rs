//! # Módulo de Componentes
//!
//! Define la infraestructura y la biblioteca de componentes del ECS.
//!
//! ## Estructura
//!
//! - **`mod.rs`**: Define el trait `Component` y re-exporta los elementos públicos.
//! - **`id.rs`**: Define `ComponentId`.
//! - **`storage.rs`**: Define `ComponentStorage` para el almacenamiento SoA.
//! - **`library/`**: Contiene componentes concretos y reutilizables.

pub mod id;
pub mod library;
pub mod storage;

pub use id::ComponentId;
pub use library::{Transform, Velocity};
pub use storage::ComponentStorage;

/// Trait que deben implementar todos los componentes ECS.
///
/// El bound `Default` es esencial para inicializar el almacenamiento de
/// componentes de manera eficiente.
pub trait Component: 'static + Default {
    /// Retorna el identificador único del tipo de componente.
    fn component_id() -> ComponentId where Self: Sized {
        ComponentId::of::<Self>()
    }
}

