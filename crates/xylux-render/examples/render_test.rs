use xylux_render::Renderer;
use xylux_ecs::{World, Transform};
use xylux_window::XyluxWindow;

fn main() {
    // Crear ventana
    let mut xwindow = XyluxWindow::new("Render Test", 800, 600);

    // Crear renderer
    let mut renderer = Renderer::new(&xwindow);

    // Crear mundo ECS
    let mut world = World::new(1000);
    world.register_component::<Transform>();

    println!("=== Inicializando cámara (Dummy) ===");
    let camera = xylux_render::Camera::new(glam::Vec3::ZERO, 10.0, 1.33);

    println!("=== Iniciando loop de renderizado ===");
    xwindow.run_loop(move |window, _events| {
        // En un test real, haríamos algo aquí
        renderer.render(&mut world, window, &camera);
    });

    // Limpiar recursos al salir
}
