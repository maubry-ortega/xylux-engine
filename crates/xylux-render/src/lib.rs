pub mod vulkan;
pub mod pipeline;
pub mod renderer;
pub mod vertex;
pub mod grid;
pub mod ui;
pub mod scene;
pub mod text;

pub use xylux_ecs::MeshComponent;
pub use xylux_core::Camera;

pub use renderer::Renderer;
pub use vulkan::context::VulkanContext;
pub use vertex::Vertex;
pub use grid::generate_grid;
pub use ui::generate_ui_panel;
pub use text::generate_text_mesh;
pub use scene::{SceneNode, Mesh};

#[cfg(test)]
mod tests {
    use super::*;
    use xylux_ecs::{World, Transform};
    use xylux_window::XyluxWindow;

    #[test]
    #[ignore] // Ignorado por CI; requiere GPU
    fn test_renderer_init_production() {
        println!("=== Inicializando ventana ===");
        let xwindow = XyluxWindow::new("Render Test", 800, 600);

        println!("=== Inicializando renderer ===");
        let mut renderer = Renderer::new(&xwindow);
        
        println!("=== Inicializando cámara ===");
        let camera = Camera::new(glam::Vec3::ZERO, 10.0, 1.33);

        println!("=== Creando mundo ECS ===");
        let mut world = World::new(1000);
        world.register_component::<Transform>();

        println!("=== Renderizando frame de prueba ===");
        renderer.render(&mut world, &xwindow, &camera);

        println!("=== Esperando a que la GPU termine ===");
        unsafe { renderer.context.device.device_wait_idle().unwrap(); }

        println!("=== Limpiando recursos del renderer ===");
        // Renderer drops automatically
        // renderer.cleanup(); 

        println!("=== Test completado correctamente ===");
    }
    
    #[test]
    fn test_ecs_mesh_component_registration() {
        let mut world = World::new(100);
        world.register_component::<Transform>();
        world.register_component::<MeshComponent>();
        
        let entity = world.spawn_entity();
        world.insert(entity, Transform::default());
        world.insert(entity, MeshComponent { start_index: 0, count: 100 });
        
        let mut query = xylux_ecs::Query::<(&Transform, &MeshComponent)>::new(&mut world);
        let count = query.iter().count();
        
        assert_eq!(count, 1, "Should find 1 entity with Transform and MeshComponent");
    }
}