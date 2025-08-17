use xylux_render::Renderer;
use xylux_ecs::{World, Transform};

fn main() {
    // Inicializamos SDL3 solo dentro del test
    let sdl_context = sdl3::init().expect("Failed to init SDL3");
    let video_subsystem = sdl_context.video().expect("Failed to get video subsystem");

    // Creamos la ventana
    let window = video_subsystem
        .window("Render Test", 800, 600)
        .vulkan()
        .resizable()
        .build()
        .expect("Failed to create window");

    // Llamamos al constructor existente de Renderer
    let mut renderer = Renderer::new(&window);

    let mut world = World::new(1000);
    world.register_component::<Transform>();

    renderer.render(&mut world);
    renderer.cleanup();
}
