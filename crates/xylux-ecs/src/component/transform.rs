//! Define el componente `Transform`, que representa la posición, rotación y
//! escala de una entidad en el espacio 3D.

use crate::component::Component;
use glam::{Quat, Vec3};

/// Componente de Transformación 3D de una entidad.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    // Podríamos añadir escala aquí en el futuro.
    // pub scale: Vec3,
}

impl Component for Transform {}