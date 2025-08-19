use xylux_render::{Renderer};
use xylux_ecs::{World, Transform};
use xylux_window::XyluxWindow;

// --- MAIN ---

fn main() {
    // Crear ventana y gestionar el loop internamente
    let mut xwindow = XyluxWindow::new("Xylux: Hello Triangle", 800, 600);

    // Inicializar renderer
    let mut renderer = Renderer::new(&xwindow);

    // Crear mundo ECS y registrar componentes
    let mut world = World::new(1000);
    world.register_component::<Transform>();

    // Ejecutar loop principal usando nuestra abstracción
    xwindow.run_loop(|window| {
        renderer.render(&mut world, window);
    });

    // Limpiar recursos al salir
    renderer.cleanup();
}
