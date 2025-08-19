//! Define el componente `Velocity`, que representa la velocidad lineal
//! de una entidad.

use crate::component::Component;
use glam::Vec3;

/// Componente de Velocidad de una entidad.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Velocity(pub Vec3);

impl Component for Velocity {}