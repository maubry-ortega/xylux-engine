use glam::Mat4;
// use crate::vertex::Vertex;

#[derive(Clone)]
pub struct Mesh {
    pub start_index: u32,
    pub count: u32,
}

pub struct SceneNode {
    pub name: String,
    pub local_transform: Mat4,
    pub children: Vec<SceneNode>,
    pub mesh: Option<Mesh>,
}

impl SceneNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            local_transform: Mat4::IDENTITY,
            children: Vec::new(),
            mesh: None,
        }
    }

    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.local_transform = transform;
        self
    }

    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }
}

pub struct RenderObject {
    pub mvp: Mat4,
    pub mesh: Mesh,
}

pub fn collect_renderables(node: &SceneNode, parent_transform: Mat4, camera: &xylux_core::Camera, renderables: &mut Vec<RenderObject>) {
    let global_transform = parent_transform * node.local_transform;

    if let Some(mesh) = &node.mesh {
        // UI uses Identity mvp concept (handled via specific nodes or flags usually, 
        // but here we can just pass P*V*M. For pure UI nodes we might need special handling logic 
        // or just set global transform to encompass the view/proj inverse if needed.
        // For now, we assume standard 3D nodes.
        let mvp = camera.get_mvp(global_transform);
        renderables.push(RenderObject {
            mvp,
            mesh: mesh.clone(),
        });
    }

    for child in &node.children {
        collect_renderables(child, global_transform, camera, renderables);
    }
}
