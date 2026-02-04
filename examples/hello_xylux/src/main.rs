use xylux_render::{Renderer, Camera};
use xylux_ecs::{World, Transform};
use xylux_window::XyluxWindow;
use glam::Vec3;

// --- MAIN ---

fn main() {
    // Crear ventana y gestionar el loop internamente
    let mut xwindow = XyluxWindow::new("Xylux Engine: Hello Xylux", 800, 600);

    // Inicializar renderer
    let mut renderer = Renderer::new(&xwindow);
    
    // Inicializar Cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0), // Target en el origen
        10.0,       // Distancia aumentada
        800.0 / 600.0 
    );
    // Ajuste inicial: Mirando fuertemente hacia abajo (60 grados)
    camera.pitch = -1.0;

    // 1. Generar Grid
    let mut scene_vertices = xylux_render::generate_grid(20.0, 1.0);
    println!("Grid generado con {} vértices.", scene_vertices.len());
    
    // 2. Intentar cargar modelo (Abeja)
    let model_path = "assets/models/abeja.blend";
    let mut model_path_buf = std::path::PathBuf::from(model_path);
    
    // Fix: Check for assets in parent directories if running from crate subfolder
    if !model_path_buf.exists() {
        if std::path::Path::new("../../assets").exists() {
             model_path_buf = std::path::PathBuf::from("../../").join(model_path);
        }
    }
    
    let mut item_names = Vec::new();

    if model_path_buf.exists() {
        println!("Intentando cargar modelo desde {:?}...", model_path_buf.canonicalize().unwrap_or_default());
        
        let extension = model_path_buf.extension().and_then(|e| e.to_str()).unwrap_or("");
        let result = match extension {
            "blend" => xylux_render::load_blend(&model_path_buf),
            _ => xylux_render::load_obj(&model_path_buf).map_err(|e| e.into()),
        };

        match result {
            Ok(meshes) => {
                if meshes.is_empty() {
                    println!("AVISO: El modelo se cargó pero no contiene objetos.");
                } else {
                    println!("¡Modelo cargado con éxito! {} objetos.", meshes.len());
                    for (name, vertices) in meshes {
                        println!(" - Mesh: '{}' ({} vertices)", name, vertices.len());
                        scene_vertices.extend(vertices);
                        item_names.push(name);
                    }
                }
            }
            Err(e) => {
                eprintln!("ERROR CRÍTICO cargando modelo: {}", e);
            }
        }
    } else {
        println!("AVISO: Modelo no encontrado en '{:?}'.", model_path_buf);
    }

    // 3. Generar UI Overlay (Panel)
    // Calcular donde empieza la UI en el buffer
    let ui_start = scene_vertices.len() as u32;

    // Generar panel y añadirlo
    scene_vertices.extend(xylux_render::ui::generate_ui_panel(&item_names));
    
    // Subir todos los vértices (Grid + Modelo + UI)
    renderer.upload_vertices(&scene_vertices);

    
    // Configurar el punto de corte para el renderizado
    renderer.set_ui_start_index(ui_start);

    // Crear mundo ECS y registrar componentes
    let mut world = World::new(1000);
    world.register_component::<Transform>();

    // Ejecutar loop principal usando nuestra abstracción
    xwindow.run_loop(|window, events| {
        for event in events {
            camera.handle_event(event);
        }
        renderer.render(&mut world, window, &camera);
    });

    // Limpiar recursos al salir
    renderer.cleanup();
}
