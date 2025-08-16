# Xylux Engine — Documento de Diseño e Implementación Completo (Versión 0.3)

**Versión:** 0.3  
**Fecha:** 2025-08-15  
**Autores:** Basado en el borrador inicial (v0.2) y extensiones de mejores prácticas, stack tecnológico, roadmap y checklists.  
**Propósito:** Este documento compila toda la información profesional para el desarrollo del motor Xylux. Úsalo como base para el repositorio GitHub. Para agregar: Crea `docs/Design-v0.3.md` en tu repo, copia este contenido, y commitea (`git add docs/ && git commit -m "Agrega documento de diseño completo v0.3"`).

---

## 0. Resumen Ejecutivo (Elevator Pitch)

Xylux es un motor de juegos 3D **compacto, modular y de rendimiento extremo**, escrito en **Rust**, diseñado para desarrolladores que trabajan íntegramente desde código. Incluye un lenguaje de scripting propio —**Alux** (anteriormente Lux)— y una máquina virtual minimalista —**AluxVM**— inspirada en Wren pero con **tipado fuertemente inferido**, coroutines nativas y opciones AOT/JIT-lite. La filosofía combina precisión matemática ("Xy") y claridad visual ("Lux"), con mascota **Luxi**, una luciérnaga guía.

**Público objetivo:** Desarrolladores indie y equipos pequeños que buscan control total, rendimiento AAA y export a WebAssembly o modo headless para servidores.

**Cambios en v0.3:** Integración de mejores prácticas AAA (DOD, ECS híbrido, pooling), stack actualizado con Ash para Vulkan (control completo), VM opcional en Zig/Nim, roadmap detallado y checklists paso a paso.

---

## 1. Identidad y Marca

### 1.1 Nombre y Significado
- **Xylux:** Combinación de *Xy* (coordenadas / precisión espacial) y *Lux* (luz / claridad visual). Representa un motor que parte del espacio técnico hacia visualizaciones elegantes.

### 1.2 Mascota
- **Luxi (luciérnaga):** Símbolo de guía y claridad. Pequeña, brillante y poderosa — refleja el motor: compacto pero impactante.

### 1.3 Taglines Sugeridos
- "Build from the XY. Light up the world."
- "Game logic. Pure light."
- "Xylux — Engineered from the coordinates up."

### 1.4 Guía Visual Rápida
- **Colores:** Cian brillante (principal), amarillo cálido (acento luciérnaga), grises neutros para texto.
- **Tipografía:** Inter o similar (legible en UI y docs).
- **Estética:** Flat + SVG animado para logo; pixel-art opcional para merchandising.

---

## 2. Objetivos y Alcance

### 2.1 Objetivo Principal
Crear un motor 3D optimizado para bajo coste en tiempo y memoria, con APIs limpias en Rust y scripting Alux de alto rendimiento y seguridad de tipos.

### 2.2 Metas Medibles
- **MVP:** Escena 3D con render PBR básico, ECS, scripting hot-reload y ejemplo jugable (>60 FPS en hardware mainstream).
- **Performance Target:** Profiling con uso eficiente de CPU/GPU; benchmarks reales para decisiones.
- **Portabilidad:** Nativo en Linux/Windows/macOS; export WebAssembly.

### 2.3 Filosofía de Diseño
- **Code-first:** Interacción por API, no editores gráficos.
- **Simplicidad Poderosa:** Interfaces minimalistas y composables.
- **Medición sobre Intuición:** Basado en benchmarks.
- **Modularidad:** Subsistemas reemplazables.
- **Seguridad y Rendimiento:** Modelos de memoria que minimicen pausas.

---

## 3. Stack Tecnológico Principal

| Componente       | Tecnología Principal | Alternativas/Opciones | Razón/Inspiración |
|------------------|----------------------|-----------------------|-------------------|
| **Núcleo Motor** | Rust (con DOD via ECS propio) | - | Simplicidad, borrow checker; inspirado en Bevy/Unity DOTS. |
| **Renderizado**  | Rust + Ash (Vulkan puro) | wgpu como fallback (feature flag) | Control AAA (command buffers, multi-queue); soporta ray tracing. Multiplataforma con MoltenVK. |
| **ECS**          | Rust (crate propio `xylux-ecs` con SoA) | Inspirado en Bevy pero minimal | Caché locality; queries compiladas y paralelo con Rayon. |
| **Input**        | Rust + winit | - | Multiplataforma (teclado/mouse/gamepad/touch). Jerárquico con pila. |
| **Audio**        | Rust + cpal (minimal) | Kira o Wwise SDK (opcional) | Bajo overhead; positional audio. |
| **Física**       | Rust + Rapier3D | Propio collider (AABB) | Integración ECS; paralelo seguro. |
| **Assets/Memoria**| Rust + Serde/RON + Pooling (bumpalo) | Tokio para async streaming | Evita GC; LOD automático y pooling. |
| **Scripting (Alux)** | Lenguaje propio: Sintaxis Wren-like con tipado inferido, coroutines | VM en Zig (principal) o Nim | Embebible, hot-reload; bindings automáticos. |
| **VM para Alux** | Zig (low-level, no GC) | Nim (GC ligero, macros) | Auditable (<10k LOC); embed via FFI en Rust. |
| **CLI/Tooling**  | Rust (clap) | mdBook para docs | Scaffolding, build, run. |
| **LSP/IDE**      | Rust (tower-lsp para Alux) | VSCode/Neovim extensions | Completado, diagnostics. |
| **Paralelismo**  | Rust + Rayon (jobs) | Tokio para async | Task graph con dependencias. |
| **Networking**   | Rust + Tokio (opcional, headless) | - | Para servidores; ECS sincronizable. |
| **Testing**      | Rust (cargo test/bench con Criterion) | GitHub Actions CI | Unitarios para VM; perf regressions. |
| **Portabilidad** | Cross-compile Cargo; Vulkan | WebAssembly export (Rust/Wasm + Vulkan polyfill) | Linux/Windows/macOS/Android; headless para CI. |

**Dependencias Mínimas:** `ash`, `winit`, `glam`, `rayon`, `serde`, `ron`, `rapier3d`, `cpal`, `tokio`, `criterion`. Feature flags para opcionales.

**Estructura del Proyecto:**
```
xylux/
├── crates/          # Workspace Rust
│   ├── xylux-core/
│   ├── xylux-render/
│   ├── xylux-ecs/
│   ├── xylux-input/
│   ├── xylux-audio/
│   ├── xylux-tools/
│   ├── alux-compiler/  # Compiler Alux (Rust/Nim)
│   ├── alux-vm/        # VM (Zig/Nim)
│   ├── xylux-cli/
├── examples/        # hello_cube
├── docs/            # Este MD y mdBook
├── Cargo.toml       # Workspace
└── README.md
```

---

## 4. Mejores Prácticas Implementadas

### 4.1 Arquitectura del Motor
- **Data-Oriented Design (DOD):** SoA para componentes (e.g., `Vec<Vec3>` para positions). Beneficio: +50% en iteraciones masivas.
- **Job System + Task Graph:** Grafo de dependencias con Rayon para paralelo seguro. Inspirado en Unreal.

Ejemplo en Rust:
```rust
pub struct World {
    positions: Vec<glam::Vec3>,
    // ...
}

pub struct TaskGraph {
    // ...
}
```

### 4.2 Gestión de Entidades/Jugador
- **ECS Híbrido:** Datos en DOD, lógica en OOP.
- **Input Jerárquico:** Pila de handlers con prioridades.

Ejemplo:
```rust
pub struct InputStack(Vec<Box<dyn InputHandler>>);
```

### 4.3 Pipeline de Renderizado
- **Deferred Rendering + Command Buffers:** Usando Ash para Vulkan stages.
- **LOD Automático:** Distancia-based para meshes.

Ejemplo:
```rust
pub struct RenderPipeline {
    // Vulkan device, buffers...
}
```

### 4.4 Gestión de Memoria/Assets
- **Pooling:** Reutiliza objetos (e.g., balas).
- **Streaming Async:** Carga en background con Tokio.

Ejemplo:
```rust
pub struct Pool<T> {
    // ...
}
```

### 4.5 Integración Alux
- Sintaxis para ECS/Paralelismo:
  ```
  system MovePlayer(query: Query<Mut<Transform>, With<Player>>) {
      // ...
  }
  ```
- Tipos: `v3`, `Handle<Texture>`.

Ejemplo Jugador AAA:
```rust
entity Player {
    transform: Transform,
    controller: PlayerController,
}
```

---

## 5. Diseño del Lenguaje Alux

### 5.1 Objetivos
- Tipado inferido fuerte.
- Sencillez para juegos: Coroutines (`yield`, `spawn`).
- Embebible y rápido.

### 5.2 Sintaxis Ejemplo
```
fn start() {
  let player = find("Player")
  spawn(function patrol() {
    while true { wait(2.0); patrol_point() }
  })
}
```

### 5.3 Tipos Principales
- Primitivos: `int`, `float`, `bool`, `string`.
- Vectores: `vec3`, `mat4`.
- Motor: `entity`, `handle<T>`.
- Coroutines: `Task<T>`.

### 5.4 Archivos
- Fuente: `.alux`.
- Bytecode: `.aluxc`.

---

## 6. AluxVM — Máquina Virtual

- **Principios:** Pequeña (<10k LOC), bytecode compacto, arenas de memoria.
- **Pipeline:** Lexer → Parser → AST → Bytecode → Ejecución.
- **Integración:** Bindings automáticos; hot-reload con checksum.
- **Opciones:** AOT/JIT-lite para hot-paths.

---

## 7. Modo Headless y Backend Lógico
- Flag `--headless` para desactivar render/audio.
- Útil para servidores, tests deterministas.

---

## 8. LSP e Integración con Editores
- `alux-lsp` en Rust: Completado, diagnostics.
- Soporte VSCode/Neovim.

---

## 9. Tooling y CLI
- Comandos: `xylux new <project>`, `run`, `build --target wasm`, `test`, `package`.

---

## 10. Optimización y Métricas
- **Estructuras:** BVH para estáticas, Octree para dinámicos.
- **Profiling:** Hooks para JSON dumps; benchmarks con Criterion.
- **Algoritmos:** Instancing, descriptor pooling; elegir por benchmarks reales.

---

## 11. Calidad, Testing y CI
- Tests unitarios/integración.
- Benchmarks automáticos en CI (GitHub Actions matrix).
- `cargo fmt`, `clippy`; normas de PRs.

---

## 12. Seguridad, Versiones y Compatibilidad
- SemVer para crates.
- Bytecode versionado con checksum.
- Política de deprecación documentada.

---

## 13. Riesgos y Mitigaciones
- Alcance grande: Roadmap MVP delimitado.
- Multiplataforma: Priorizar Linux + Web en MVP.
- Lenguaje propio: Subset minimal inicial.

---

## 14. Roadmap y Milestones

**Fase 0 (0-1 semana):** Repo setup, CLI stub, docs iniciales, logo SVG, Alux REPL mínimo.

**Fase 1 (1-4 semanas):** Alux compiler (lexer/parser), VM básica, hello_cube con render Ash.

**Fase 2 (4-8 semanas):** Render mejoras (culling, instancing), ECS estable, hot-reload, LSP prototipo, headless.

**Fase 3 (8-16 semanas):** Física/audio/assets, paralelismo, LOD/pooling.

**Fase 4 (16-24 semanas):** Optimización AOT/JIT, networking, docs completas, export Wasm.

**Milestones:** MVP en semana 8; v1.0 en semana 24.

---

## 15. Checklists para Implementación desde Cero

### Checklist 1: Setup General y Repo
- [X] Crea repo GitHub: "xylux-engine".
- [X] Clona local.
- [X] Crea estructura (ver stack).
- [X] Agrega .gitignore.
- [X] Commit inicial.

### Checklist 2: Setup IDE para Rust
- [X] Instala Rust (stable).
- [X] Verifica `rustc --version`.
- [X] Instala VSCode + rust-analyzer o Neovim LSP.
- [X] Cargo workspace en root.
- [X] Prueba `cargo build`.
- [X] Configura GitHub Actions CI.

### Checklist 3: Motor Núcleo en Rust
- [ ] Crate ECS: SoA, spawn_entity.
- [ ] Crate Render: Ash init, draw triangle.
- [ ] Crate Core: Loop, integrate ECS+Render.
- [ ] Example hello_cube.
- [ ] Benchmarks con Criterion.

### Checklist 4: Lenguaje Alux
- [ ] Compiler: Lexer/parser (Rust/Nim).
- [ ] Bytecode generator.
- [ ] REPL.
- [ ] Bindings a ECS.
- [ ] Test sintaxis.

### Checklist 5: VM Alux en Zig/Nim
- [ ] Instala Zig/Nim.
- [ ] Interpreter bytecode.
- [ ] Coroutines, profiling.
- [ ] FFI a Rust.
- [ ] Hot-reload.
- [ ] Test ejecución.

### Checklist 6: Integración y Polish
- [ ] CLI con clap.
- [ ] Docs mdBook.
- [ ] Multiplataforma tests.
- [ ] Optimizaciones (pooling, LOD).
- [ ] Release v0.1.

---

## 16. Entregables Iniciales (v0.3)
- Este documento como `docs/Design-v0.3.md`.
- Repo con estructura y CLI stub.
- REPL/VM minimal y hello_cube.
- Logo SVG y ASCII para CLI.

---

## 17. Siguientes Pasos Concretos
1. Agrega este MD a `docs/` en tu repo GitHub.
2. Implementa Fase 0: CLI stub y REPL Alux.
3. Commitea cambios y push a GitHub.
4. Mide progreso con checklists; ajusta basado en benchmarks.

---

## 18. Notas Finales
Este documento v0.3 es completo y profesional, orientado a rendimiento extremo y desarrollo code-first. Mantén cultura de medición (benchmarks) y documentación. Para contribuciones, usa PRs en GitHub. Si necesitas actualizaciones, itera en v0.4.