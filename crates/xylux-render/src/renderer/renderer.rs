use ash::vk;
use crate::pipeline::Pipeline;
use crate::vulkan::context::{VulkanContext, MAX_FRAMES_IN_FLIGHT};

use xylux_ecs::World;
use crate::renderer::{render_pass, framebuffers, commands};
use xylux_window::XyluxWindow; // << Importar tu wrapper de ventana

pub struct Renderer {
    // Hacemos el contexto mutable para actualizar el estado de sincronización
    pub context: VulkanContext,
    pub pipeline: Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub render_pass: vk::RenderPass,
    pub(crate) current_frame: usize,
}

impl Renderer {
    // ✅ Cambiar &Window por &XyluxWindow
    pub fn new(window: &XyluxWindow) -> Self {
        // 1️⃣ Crear contexto Vulkan
        let context = VulkanContext::new(window);

        // 2️⃣ Crear render pass usando la información del contexto
        let render_pass = render_pass::create_render_pass(&context.device, context.swapchain_format());

        // 3️⃣ Crear framebuffers
        let framebuffers = framebuffers::create_framebuffers(
            &context.device,
            render_pass,
            context.swapchain_image_views(),
            context.swapchain_extent(),
        );

        // 4️⃣ Crear pipeline
        let pipeline = Pipeline::new(&context.device, render_pass, context.swapchain_extent());

        Self {
            context,
            pipeline,
            framebuffers,
            render_pass,
            current_frame: 0,
        }
    }

    pub fn render(&mut self, world: &mut World, window: &XyluxWindow) {
        commands::render_frame(self, world, window);
        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    pub fn device_wait_idle(&self) {
        unsafe {
            self.context.device.device_wait_idle().unwrap();
        }
    }

    fn cleanup_swapchain(&self) {
        unsafe {
            for &framebuffer in &self.framebuffers {
                self.context.device.destroy_framebuffer(framebuffer, None);
            }
            self.pipeline.cleanup(&self.context.device);
            self.context.device.destroy_render_pass(self.render_pass, None);
            self.context.cleanup_swapchain_resources();
        }
    }

    pub fn recreate_swapchain(&mut self, window: &XyluxWindow) {
        self.device_wait_idle();
        self.cleanup_swapchain();

        // Recrear swapchain y sus dependencias
        self.context.recreate_swapchain_resources(window);

        self.render_pass = render_pass::create_render_pass(
            &self.context.device,
            self.context.swapchain_format(),
        );
        self.pipeline = Pipeline::new(
            &self.context.device,
            self.render_pass,
            self.context.swapchain_extent(),
        );
        self.framebuffers = framebuffers::create_framebuffers(
            &self.context.device,
            self.render_pass,
            self.context.swapchain_image_views(),
            self.context.swapchain_extent(),
        );
    }

    pub fn cleanup(&self) {
        // Esperar a que la GPU termine todas las operaciones pendientes antes de limpiar.
        self.device_wait_idle();
        self.cleanup_swapchain();
        self.context.cleanup();
    }
}
