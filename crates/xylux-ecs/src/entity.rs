//! Define la entidad ECS y su identificador generacional.

/// Representa una entidad única en el ECS.
///
/// `(id, version)` evita accesos a entidades recicladas (problema del ABA).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity {
    pub id: usize,
    pub version: u32,
}