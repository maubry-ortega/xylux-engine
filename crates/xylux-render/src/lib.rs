pub mod vulkan;
pub mod pipeline;
pub mod renderer;
pub mod window;

// Reexportar los tipos principales que usarás desde afuera
pub use renderer::Renderer;
pub use vulkan::context::VulkanContext;
pub use window::XyluxWindow;

#[cfg(test)]
mod tests {
    use super::*;
    use xylux_ecs::{World, Transform};

    #[test]
    #[ignore] // Ignorado por CI; requiere GPU
    fn test_renderer_init_production() {
        println!("=== Inicializando ventana ===");
        let xwindow = XyluxWindow::new("Render Test", 800, 600);

        println!("=== Inicializando renderer ===");
        // No necesitamos `mut` si solo vamos a llamar métodos que toman &self
        let renderer = Renderer::new(&xwindow.window);

        println!("=== Creando mundo ECS ===");
        let mut world = World::new(1000);
        world.register_component::<Transform>();

        println!("=== Renderizando frame de prueba ===");
        // Render seguro, pasamos mutable world
        renderer.render(&mut world);

        println!("=== Esperando a que la GPU termine ===");
        renderer.device_wait_idle();

        println!("=== Limpiando recursos del renderer ===");
        renderer.cleanup();

        println!("=== Test completado correctamente ===");
        // xwindow se destruye al salir del scope, después de cleanup
    }
}
