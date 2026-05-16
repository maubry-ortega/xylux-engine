# Xylux Engine

<p align="center"><img src="/assets/logo_xylux.png" width="400" alt="Xylux Engine Logo"></p>

> "Build from the XY. Light up the world."

**Xylux** es un motor de juegos 3D **compacto, modular y de rendimiento extremo**, escrito en **Rust**. Está diseñado desde cero para desarrolladores que prefieren un enfoque *code-first*, ofreciendo control total y una arquitectura moderna basada en Data-Oriented Design.

Nuestra mascota es **Luxi**, una luciérnaga que simboliza la filosofía del motor: pequeño, brillante y potente.
<p align="center"><img src="/assets/luxi.png" width="400" alt="Xylux Engine Logo"></p>

## ✨ Estado Actual y Características Implementadas

El motor se encuentra en una fase inicial de desarrollo, pero ya cuenta con una base sólida y funcional.

- **🚀 ECS de Alto Rendimiento (`xylux-ecs`)**:
  - **Arquitectura SoA (Struct of Arrays)** para un acceso a memoria amigable con la caché.
  - **Índices Generacionales** para un manejo seguro y eficiente del ciclo de vida de las entidades.
  - **Queries ultra-rápidas** basadas en la intersección de `BitVec`s.
  - **Sistema de Tareas (`TaskGraph`)** para ejecutar sistemas con gestión de dependencias.

- **🎨 Renderizador Vulkan (`xylux-render`)**:
  - Construido sobre **Ash** para un control de bajo nivel sobre la GPU.
  - Pipeline de renderizado básico, capaz de mostrar primitivas en pantalla.

- **🖼️ Gestión de Ventanas y Eventos (`xylux-window`)**:
  - Abstracción sobre **SDL3**, proporcionando una base robusta y multiplataforma para la creación de ventanas, manejo de input y audio.

## 🏁 Cómo Empezar

### Prerrequisitos

Asegúrate de tener instalada la última versión estable de Rust y las dependencias de Vulkan para tu sistema operativo.

```bash
rustup update stable
```

### Ejecutar el Ejemplo

Para ver el motor en acción, clona el repositorio y ejecuta el ejemplo `hello_triangle`:

```bash
git clone https://github.com/maubry-ortega/xylux-engine.git
cd xylux-engine
cargo run -p hello_triangle
```

## 📂 Estructura del Proyecto

- `crates/`: Contiene los módulos principales del motor (workspace de Rust).
  - `xylux-ecs`: El núcleo del Entity-Component-System.
  - `xylux-render`: El backend de renderizado con Vulkan.
  - `xylux-window`: La capa de abstracción de ventanas.
  - ... y otros futuros crates del motor.
- `examples/`: Proyectos de ejemplo que demuestran el uso del motor.
- `docs/`: Documentación del motor y del lenguaje de scripting.

## 💡 Visión y Roadmap

El objetivo a largo plazo es construir un motor completo que incluya un lenguaje de scripting propio —**Alux**— y una máquina virtual minimalista —**AluxVM**—. Consulta el documento de diseño para más detalles sobre el stack tecnológico, las mejores prácticas y el roadmap completo.
```