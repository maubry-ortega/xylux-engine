pub mod vulkan;
pub mod pipeline;
pub mod renderer;

pub use renderer::Renderer;
pub use vulkan::context::VulkanContext;

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

        println!("=== Creando mundo ECS ===");
        let mut world = World::new(1000);
        world.register_component::<Transform>();

        println!("=== Renderizando frame de prueba ===");
        renderer.render(&mut world, &xwindow);

        println!("=== Esperando a que la GPU termine ===");
        renderer.device_wait_idle();

        println!("=== Limpiando recursos del renderer ===");
        renderer.cleanup();

        println!("=== Test completado correctamente ===");
    }
}