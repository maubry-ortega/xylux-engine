use xylux_engine::XyluxEngine;

fn main() {
    let mut engine = XyluxEngine::new("Xylux Engine - Integrated Demo");
    
    // Load custom assets from the default directory
    engine.load_assets("assets/models");

    engine.run(|_world, _input, _camera| {
        // User update logic here if any
        // Example: logging when a key is pressed
    });
}
