use ash::vk;
use ash::khr::swapchain;
use sdl3::video::Window;
use crate::pipeline::Pipeline;
use crate::vulkan::VulkanContext;
use xylux_ecs::{World, Query, Transform};

pub struct Renderer {
    context: VulkanContext,
    pipeline: Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    swapchain_extent: vk::Extent2D,
    swapchain_loader: swapchain::Device,
    render_pass: vk::RenderPass,
    swapchain_image_views: Vec<vk::ImageView>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let context = VulkanContext::new(window);

        // Inicializar el swapchain loader
        let swapchain_loader = swapchain::Device::new(&context.instance, &context.device);

        // Obtener el formato y tamaño del swapchain desde VulkanContext usando getters
        let swapchain_extent = context.swapchain_extent();
        let swapchain_format = context.swapchain_format();
        let swapchain_image_views = context.swapchain_image_views.clone();

        // Crear render pass
        let render_pass = Self::create_render_pass(&context.device, swapchain_format);

        // Crear framebuffers
        let framebuffers = Self::create_framebuffers(
            &context.device,
            render_pass,
            &swapchain_image_views,
            swapchain_extent,
        );

        let pipeline = Pipeline::new(&context.device, swapchain_format);

        Self {
            context,
            pipeline,
            framebuffers,
            swapchain_extent,
            swapchain_loader,
            render_pass,
            swapchain_image_views,
        }
    }

    fn create_render_pass(device: &ash::Device, swapchain_format: vk::Format) -> vk::RenderPass {
        let color_attachment = vk::AttachmentDescription {
            format: swapchain_format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        };

        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass = vk::SubpassDescription {
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            ..Default::default()
        };

        let render_pass_info = vk::RenderPassCreateInfo {
            attachment_count: 1,
            p_attachments: &color_attachment,
            subpass_count: 1,
            p_subpasses: &subpass,
            ..Default::default()
        };

        unsafe { device.create_render_pass(&render_pass_info, None).unwrap() }
    }

    fn create_framebuffers(
        device: &ash::Device,
        render_pass: vk::RenderPass,
        image_views: &[vk::ImageView],
        extent: vk::Extent2D,
    ) -> Vec<vk::Framebuffer> {
        image_views
            .iter()
            .map(|&image_view| {
                let attachments = [image_view];
                let create_info = vk::FramebufferCreateInfo {
                    render_pass,
                    attachment_count: attachments.len() as u32,
                    p_attachments: attachments.as_ptr(),
                    width: extent.width,
                    height: extent.height,
                    layers: 1,
                    ..Default::default()
                };
                unsafe { device.create_framebuffer(&create_info, None).unwrap() }
            })
            .collect()
    }

    pub fn render(&self, world: &mut World) {
        let (image_index, _) = unsafe {
            self.swapchain_loader
                .acquire_next_image(
                    self.context.swapchain,
                    u64::MAX,
                    vk::Semaphore::null(),
                    vk::Fence::null(),
                )
                .unwrap()
        };

        let command_buffer = self.context.command_buffers[image_index as usize];

        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };

        unsafe {
            self.context
                .device
                .begin_command_buffer(command_buffer, &begin_info)
                .unwrap();

            let clear_color = vk::ClearValue {
    color: vk::ClearColorValue { float32: [1.0, 0.0, 0.0, 1.0] }, // rojo brillante
};


            let render_pass_info = vk::RenderPassBeginInfo {
                render_pass: self.render_pass,
                framebuffer: self.framebuffers[image_index as usize],
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: self.swapchain_extent,
                },
                clear_value_count: 1,
                p_clear_values: &clear_color,
                ..Default::default()
            };

            self.context.device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );

            self.context.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.pipeline,
            );

            let mut query = Query::<&Transform>::new(world);
            for _transform in query.iter() {
                // Aquí iría tu draw call
            }

            self.context.device.cmd_end_render_pass(command_buffer);
            self.context
                .device
                .end_command_buffer(command_buffer)
                .unwrap();

            let submit_info = vk::SubmitInfo {
                command_buffer_count: 1,
                p_command_buffers: &command_buffer,
                ..Default::default()
            };

            self.context
                .device
                .queue_submit(self.context.queue, &[submit_info], vk::Fence::null())
                .unwrap();

            let present_info = vk::PresentInfoKHR {
                swapchain_count: 1,
                p_swapchains: &self.context.swapchain,
                p_image_indices: &image_index,
                ..Default::default()
            };

            self.swapchain_loader
                .queue_present(self.context.queue, &present_info)
                .unwrap();

            self.context
                .device
                .queue_wait_idle(self.context.queue)
                .unwrap();
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            for &framebuffer in &self.framebuffers {
                self.context.device.destroy_framebuffer(framebuffer, None);
            }
            for &image_view in &self.swapchain_image_views {
                self.context.device.destroy_image_view(image_view, None);
            }
            self.context.device.destroy_render_pass(self.render_pass, None);
        }
        self.pipeline.cleanup(&self.context.device);
        self.context.cleanup();
    }
}
