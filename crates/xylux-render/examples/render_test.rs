// main.rs
use xylux_render::{Renderer, XyluxWindow};
use xylux_ecs::{World, Transform};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

fn main() {
    // Inicializar ventana
    let xwindow = XyluxWindow::new("Render Test", 800, 600);

    // Inicializar renderer
    let mut renderer = Renderer::new(&xwindow.window);

    // Crear mundo ECS
    let mut world = World::new(1000);
    world.register_component::<Transform>();

    // Event loop principal
    let mut event_pump = xwindow.sdl.event_pump().unwrap();
    'running: loop {
        // Procesar eventos
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Renderizar frame
        renderer.render(&mut world);

        // Pequeño delay para no saturar la GPU/CPU
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    // Limpiar recursos al salir
    renderer.cleanup();
}
