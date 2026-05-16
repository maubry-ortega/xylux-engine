use crate::component::Component;

#[derive(Clone, Copy, Debug, Default)]
pub struct MeshComponent {
    pub start_index: u32,
    pub count: u32,
}

impl Component for MeshComponent {}
