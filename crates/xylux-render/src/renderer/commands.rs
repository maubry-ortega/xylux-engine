use ash::vk;
use crate::renderer::Renderer;
use xylux_ecs::{World, Query, Transform};

// Ya no usamos 'extensions::khr::Swapchain', usamos el módulo público swapchain
use ash::khr::swapchain;

pub fn render_frame(renderer: &Renderer, world: &mut World) {
    // Creamos el loader local usando el módulo público swapchain
    let swapchain_loader = swapchain::Device::new(&renderer.context.instance, &renderer.context.device);

    // Adquirir la siguiente imagen del swapchain
    let (image_index, _) = unsafe {
        swapchain_loader
            .acquire_next_image(
                renderer.context.swapchain,
                u64::MAX,
                vk::Semaphore::null(),
                vk::Fence::null(),
            )
            .expect("Failed to acquire next swapchain image")
    };

    let command_buffer = renderer.context.command_buffers[image_index as usize];
    assert!(command_buffer != vk::CommandBuffer::null(), "Command buffer is null!");

    let begin_info = vk::CommandBufferBeginInfo {
        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        ..Default::default()
    };

    unsafe {
        renderer.context.device.begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        let clear_color = vk::ClearValue {
            color: vk::ClearColorValue { float32: [1.0, 0.0, 0.0, 1.0] },
        };

        let render_pass_info = vk::RenderPassBeginInfo {
            render_pass: renderer.render_pass,
            framebuffer: renderer.framebuffers[image_index as usize],
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: renderer.swapchain_extent,
            },
            clear_value_count: 1,
            p_clear_values: &clear_color,
            ..Default::default()
        };

        renderer.context.device.cmd_begin_render_pass(command_buffer, &render_pass_info, vk::SubpassContents::INLINE);
        renderer.context.device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, renderer.pipeline.pipeline);

        let mut query = Query::<&Transform>::new(world);
        for _ in query.iter() {
            // Aquí irían las llamadas de draw
        }

        renderer.context.device.cmd_end_render_pass(command_buffer);
        renderer.context.device.end_command_buffer(command_buffer)
            .expect("Failed to end command buffer");

        let submit_info = vk::SubmitInfo {
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            ..Default::default()
        };

        renderer.context.device.queue_submit(renderer.context.queue, &[submit_info], vk::Fence::null())
            .expect("Failed to submit queue");

        let present_info = vk::PresentInfoKHR {
            swapchain_count: 1,
            p_swapchains: &renderer.context.swapchain,
            p_image_indices: &image_index,
            ..Default::default()
        };

        swapchain_loader.queue_present(renderer.context.queue, &present_info)
            .expect("Failed to present swapchain image");

        renderer.context.device.queue_wait_idle(renderer.context.queue)
            .expect("Failed to wait device idle");
    }
}
