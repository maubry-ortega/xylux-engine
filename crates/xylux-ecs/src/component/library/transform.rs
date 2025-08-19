//! Define el componente `Transform`, que representa la posición y rotación
//! de una entidad en el espacio 3D.

use crate::component::Component;
use glam::{Quat, Vec3};

/// Componente de Transformación 3D de una entidad.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Component for Transform {}