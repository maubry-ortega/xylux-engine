use xylux_window::XyluxWindow;
use xylux_render::{Renderer, Vertex};
use xylux_ecs::{World, Transform, MeshComponent};
use xylux_core::{Camera, SceneLoader};
use xylux_assets::AssetManager;
use xylux_input::InputContext;
use glam::Vec3;
use std::path::Path;
use std::fs;

pub struct XyluxEngine {
    pub window: XyluxWindow,
    pub renderer: Renderer,
    pub world: World,
    pub assets: AssetManager,
    pub camera: Camera,
    pub input: InputContext,
    scene_vertices: Vec<Vertex>,
}

impl XyluxEngine {
    pub fn new(title: &str) -> Self {
        let window = XyluxWindow::new(title, 800, 600);
        let renderer = Renderer::new(&window);
        let assets = AssetManager::new();
        
        let mut world = World::new(1000);
        world.register_component::<Transform>();
        world.register_component::<MeshComponent>();
        
        let camera = Camera::new(Vec3::ZERO, 10.0, 800.0 / 600.0);
        let input = InputContext::new();

        // Standard engine initialization: Grid
        let scene_vertices = xylux_render::generate_grid(20.0, 1.0);

        let mut engine = Self {
            window,
            renderer,
            world,
            assets,
            camera,
            input,
            scene_vertices,
        };

        // Ensure initial state is uploaded (Grid + Empty UI)
        engine.sync_renderer(&[]);
        
        engine
    }

    /// Synchronizes internal vertex data with the GPU renderer
    fn sync_renderer(&mut self, loaded_names: &[String]) {
        // UI start index: where the mesh vertices end
        let ui_start = self.scene_vertices.len() as u32;
        
        // Generate UI vertices (even if empty names, it provides the UI pane)
        let ui_vertices = xylux_render::ui::generate_ui_panel(loaded_names);
        
        // Combine temporary for upload (Grid/Meshes + UI)
        let mut all_vertices = self.scene_vertices.clone();
        all_vertices.extend(ui_vertices);

        self.renderer.upload_vertices(&all_vertices);
        self.renderer.set_ui_start_index(ui_start);
    }

    /// Automatically discover models in default location and spawn them
    pub fn preload_assets(&mut self) {
        self.load_assets("assets/models");
    }

    /// Load assets from a specific file or directory
    pub fn load_assets<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        let mut loaded_names = Vec::new();
        let mut to_load = Vec::new();

        if path.is_file() {
            to_load.push(path.to_path_buf());
        } else if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(ext) = entry_path.extension() {
                            if ext == "obj" || ext == "blend" || ext == "gltf" || ext == "glb" {
                                to_load.push(entry_path);
                            }
                        }
                    }
                }
            }
        }

        for model_path in to_load {
             if let Some(path_str) = model_path.to_str() {
                 if let Ok(meshes) = self.assets.load_model(path_str) {
                     for (name, _) in &meshes {
                         loaded_names.push(name.clone());
                     }
                     SceneLoader::spawn_meshes(&mut self.world, meshes, &mut self.scene_vertices);
                 }
             }
        }

        // Sync everything to GPU
        self.sync_renderer(&loaded_names);
    }

    pub fn run<F>(mut self, mut update: F) 
    where 
        F: FnMut(&mut World, &InputContext, &mut Camera) + 'static
    {
        // Ensure assets are preloaded if not done manually
        if self.world.entity_count() == 0 {
            // self.preload_assets(); // Optional: can be called manually
        }

        self.window.run_loop(move |window, events| {
            self.input.reset_per_frame();
            for event in events {
                self.input.process_event(event);
            }
            
            // Allow user to update game state
            update(&mut self.world, &self.input, &mut self.camera);
            
            // Engine updates systems
            self.camera.update(&self.input);
            
            // Render everything
            self.renderer.render(&mut self.world, window, &self.camera);
        });
    }
}
