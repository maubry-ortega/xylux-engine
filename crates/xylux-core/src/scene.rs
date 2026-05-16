use xylux_ecs::{World, Transform, MeshComponent};
use crate::Vertex;
use glam::Vec3;

pub struct SceneLoader;

impl SceneLoader {
    pub fn spawn_meshes(
        world: &mut World,
        meshes: Vec<(String, Vec<Vertex>)>,
        scene_vertices: &mut Vec<Vertex>,
    ) {
        for (name, vertices) in meshes {
            println!("Spawning entity for mesh: '{}' ({} vertices)", name, vertices.len());
            
            let start_index = scene_vertices.len() as u32;
            let count = vertices.len() as u32;
            
            scene_vertices.extend(vertices);
            
            let entity = world.spawn_entity();
            
            world.insert(entity, Transform {
                position: Vec3::ZERO,
                rotation: glam::Quat::IDENTITY,
                scale: Vec3::ONE,
            });
            
            world.insert(entity, MeshComponent {
                start_index,
                count,
            });
        }
    }
}
