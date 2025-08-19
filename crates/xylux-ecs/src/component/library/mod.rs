//! # Biblioteca de Componentes
//!
//! Contiene componentes concretos y reutilizables para el motor.
//! Esta separación mantiene el núcleo del ECS agnóstico a los tipos
//! de componentes específicos del juego o motor.

pub mod transform;
pub mod velocity;

pub use transform::Transform;
pub use velocity::Velocity;