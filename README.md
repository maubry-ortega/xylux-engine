# Xylux Engine

Xylux is a modular, high-performance 3D engine built in Rust using Vulkan. It focuses on simplicity through its facade API while maintaining a powerful modular architecture.

## Project Structure

The project is divided into several specialized crates:

- **`xylux-engine`**: High-level facade for easy engine usage.
- **`xylux-window`**: Window management and event handling (winit).
- **`xylux-render`**: Vulkan-based renderer with automatic vertex management.
- **`xylux-ecs`**: Entity Component System for efficient game logic.
- **`xylux-core`**: Core types (Vertex, Camera, SceneLoader).
- **`xylux-assets`**: Asset management and model loading (supports `.obj`, `.blend`, `.gltf`).
- **`xylux-input`**: Input handling and context management.

## Quick Start (Hello Xylux)

The easiest way to start is using the `xylux-engine` facade.

```rust
use xylux_engine::XyluxEngine;

fn main() {
    let mut engine = XyluxEngine::new("My Game");
    
    // Load all models in a folder
    engine.load_assets("assets/models");
    
    // Or load a specific model
    // engine.load_assets("assets/models/abeja.blend");

    engine.run(|world, input, camera| {
        // Your game logic here
        // world: ECS world
        // input: Input context
        // camera: Scene camera
    });
}
```

## Running Examples

To run the main demo:
```bash
cargo run -p hello_xylux
```

## Documentation

- [Engine Manual](ENGINE_MANUAL.md): Detailed API reference and usage guide.
- [Walkthrough](.gemini/antigravity/brain/c2ff06b7-fe99-49d2-bc7f-0a4ca1547053/walkthrough.md): Evolution of the engine's development.