use ash::vk;
use sdl3::video::Window;

use crate::pipeline::Pipeline;
use crate::vulkan::context::VulkanContext;
use crate::vulkan::create_swapchain; // Función para crear swapchain
use crate::renderer::{render_pass, framebuffers, commands};

use xylux_ecs::World; // Para usar World si es necesario

pub struct Renderer {
    pub context: VulkanContext,
    pub pipeline: Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub swapchain_extent: vk::Extent2D,
    pub render_pass: vk::RenderPass,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        // Crear contexto Vulkan
        let mut context = VulkanContext::new(window);

        // Crear swapchain
        let (swapchain, format, extent, images, image_views) = create_swapchain(
            &context.entry,
            &context.instance,
            context.physical_device,
            &context.device,
            context.surface,
            window,
        );

        // Guardamos los swapchain images y el swapchain en el contexto
        context.swapchain = swapchain;
        context.swapchain_images = images.clone();

        // Crear render pass
        let render_pass = render_pass::create_render_pass(&context.device, format);

        // Crear framebuffers
        let framebuffers = framebuffers::create_framebuffers(
            &context.device,
            render_pass,
            &image_views.to_vec(), // convertir slice a Vec
            extent,
        );

        // Crear pipeline
        let pipeline = Pipeline::new(&context.device, format);

        Self {
            context,
            pipeline,
            framebuffers,
            swapchain_extent: extent,
            render_pass,
            swapchain_images: images,
            swapchain_image_views: image_views,
        }
    }

    pub fn render(&self, world: &mut World) {
        commands::render_frame(self, world);
    }

    pub fn device_wait_idle(&self) {
        unsafe { self.context.device.device_wait_idle().unwrap(); }
    }

    pub fn cleanup(&self) {
        unsafe {
            for &fb in &self.framebuffers {
                self.context.device.destroy_framebuffer(fb, None);
            }
            for &iv in &self.swapchain_image_views {
                self.context.device.destroy_image_view(iv, None);
            }
            self.context.device.destroy_render_pass(self.render_pass, None);
        }
        self.pipeline.cleanup(&self.context.device);
        self.context.cleanup();
    }
}
