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

    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub vertex_count: u32,
    pub ui_start_index: u32,
}

impl Renderer {
    // ✅ Cambiar &Window por &XyluxWindow
    pub fn new(window: &XyluxWindow) -> Self {
        // 1️⃣ Crear contexto Vulkan
        let context = VulkanContext::new(window);

        // 2️⃣ Obtener formato de profundidad
        let depth_format = crate::vulkan::image::find_depth_format(&context.instance, context.physical_device);

        // 3️⃣ Crear render pass usando la información del contexto
        let render_pass = render_pass::create_render_pass(
            &context.device, 
            context.swapchain_format(),
            depth_format
        );

        // 4️⃣ Crear framebuffers
        let framebuffers = framebuffers::create_framebuffers(
            &context.device,
            render_pass,
            context.swapchain_image_views(),
            context.depth_image_view,
            context.swapchain_extent(),
        );

        // 5️⃣ Crear pipeline
        let pipeline = Pipeline::new(&context.device, render_pass, context.swapchain_extent());

        Self {
            context,
            pipeline,
            framebuffers,
            render_pass,
            current_frame: 0,
            vertex_buffer: vk::Buffer::null(),
            vertex_buffer_memory: vk::DeviceMemory::null(),
            vertex_count: 0,
            ui_start_index: 0,
        }
    }

    pub fn upload_vertices(&mut self, vertices: &[crate::vertex::Vertex]) {
        if self.vertex_buffer != vk::Buffer::null() {
            unsafe {
                self.context.device.destroy_buffer(self.vertex_buffer, None);
                self.context.device.free_memory(self.vertex_buffer_memory, None);
            }
        }

        let buffer_size = (vertices.len() * std::mem::size_of::<crate::vertex::Vertex>()) as vk::DeviceSize;
        let (buffer, memory) = crate::vulkan::buffer::create_buffer(
            &self.context.instance,
            &self.context.device,
            self.context.physical_device,
            buffer_size,
            vk::ImageUsageFlags::empty(),
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        crate::vulkan::buffer::copy_data_to_buffer(&self.context.device, memory, vertices);

        self.vertex_buffer = buffer;
        self.vertex_buffer_memory = memory;
        self.vertex_count = vertices.len() as u32;
        self.ui_start_index = self.vertex_count; // Default: Todo es escena
    }

    pub fn set_ui_start_index(&mut self, index: u32) {
        self.ui_start_index = index;
    }

    pub fn render(&mut self, world: &mut World, window: &XyluxWindow, camera: &xylux_core::Camera) {
        commands::render_frame(self, world, window, camera);
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

        let depth_format = crate::vulkan::image::find_depth_format(&self.context.instance, self.context.physical_device);

        self.render_pass = render_pass::create_render_pass(
            &self.context.device,
            self.context.swapchain_format(),
            depth_format,
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
            self.context.depth_image_view,
            self.context.swapchain_extent(),
        );
    }

    pub fn cleanup(&self) {
        // Esperar a que la GPU termine todas las operaciones pendientes antes de limpiar.
        unsafe {
             let _ = self.context.device.device_wait_idle();
        }
        if self.vertex_buffer != vk::Buffer::null() {
            unsafe {
                self.context.device.destroy_buffer(self.vertex_buffer, None);
                self.context.device.free_memory(self.vertex_buffer_memory, None);
            }
        }
        self.cleanup_swapchain();
        self.context.cleanup(); // Warning: if Context doesn't impl Drop, this is good.
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.cleanup();
    }
}
