# Xylux Engine Manual

This manual provides a detailed reference for the `XyluxEngine` facade API and the underlying modular systems.

## The `XyluxEngine` Facade

The `XyluxEngine` struct is the main entry point for most applications. It encapsulates all core systems into a single interface.

### Initialization

```rust
let engine = XyluxEngine::new("Window Title");
```
`new()` initializes the window, Vulkan renderer, ECS world, asset manager, and camera. It also creates a default grid in the scene.

### Asset Loading

The engine supports loading models and their meshes into the world and automatic synchronization with the renderer.

#### `load_assets(path: &str)`
Loads assets from a specific file path or scans an entire directory.
- **Supported extensions**: `.obj`, `.blend`, `.gltf`, `.glb`.
- **Directory loading**: Automatically scans and loads all compatible files.

```rust
// Scan directory
engine.load_assets("assets/models");

// Load specific file
engine.load_assets("assets/models/character.obj");
```

#### `preload_assets()`
A convenience method that calls `load_assets("assets/models")`.

### The Game Loop

#### `run<F>(engine, callback: F)`
Starts the main engine loop. The callback is executed every frame and provides access to the engine's core components.

```rust
engine.run(|world, input, camera| {
    // world: &mut World (ECS)
    // input: &InputContext
    // camera: &mut Camera
});
```

## Underlying Systems

For advanced users, the facade provides public access to its internal systems.

### ECS (`world`)
Xylux uses a custom Entity Component System.
- **Register Components**: Done automatically for `Transform` and `MeshComponent` in `XyluxEngine::new()`.
- **Query Entities**: Use `world.query::<T>()` to iterate over components.

### Input (`input`)
- **Key States**: `input.is_key_pressed(VirtualKeyCode::W)`
- **Mouse Delta**: Use `input.mouse_delta()` for camera rotation.

### Camera (`camera`)
The camera handles its own updates based on input by default, but can be manually manipulated.

## UI System
Xylux automatically generates a UI panel on the left side of the screen.
- **Loaded Meshes List**: Shows the names of all meshes loaded via `load_assets`.
- **Dynamic Updates**: The UI is synchronized every time new assets are loaded.

## Troubleshooting

- **Empty Screen**: If you don't see anything, ensure `sync_renderer()` was called (it is called by `new()` and `load_assets()`).
- **Missing Models**: Check that the file paths are correct relative to the executable or project root.
