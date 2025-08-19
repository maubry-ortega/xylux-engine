use ash::vk;
use crate::renderer::renderer::Renderer;
use xylux_ecs::{World, Query, Transform};
use xylux_window::XyluxWindow;

// Ya no usamos 'extensions::khr::Swapchain', usamos el módulo público swapchain
use ash::khr::swapchain;

pub fn render_frame(renderer: &mut Renderer, world: &mut World, window: &XyluxWindow) {
    let context = &mut renderer.context;
    let device = &context.device;
    let current_frame = renderer.current_frame;

    unsafe {
        // 1. Esperar a que el frame que vamos a usar esté disponible (su fence).
        device.wait_for_fences(&[context.in_flight_fences[current_frame]], true, u64::MAX)
            .expect("Failed to wait for fence");

        // 2. Adquirir la siguiente imagen del swapchain.
        let swapchain_loader = swapchain::Device::new(&context.instance, &context.device);
        let result = swapchain_loader
            .acquire_next_image(
                context.swapchain,
                u64::MAX,
                context.image_available_semaphores[current_frame], // Señalizar este semáforo cuando la imagen esté lista.
                vk::Fence::null(), // No usar un fence aquí.
            );

        let image_index = match result {
            Ok((index, _)) => index,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                renderer.recreate_swapchain(window);
                return;
            }
            Err(error) => panic!("Failed to acquire swapchain image: {}", error),
        };

        // Comprobar si un frame anterior está usando esta imagen y esperar si es así.
        if context.images_in_flight[image_index as usize] != vk::Fence::null() {
            device.wait_for_fences(&[context.images_in_flight[image_index as usize]], true, u64::MAX).unwrap();
        }
        // Marcar la imagen como en uso por este frame.
        context.images_in_flight[image_index as usize] = context.in_flight_fences[current_frame];

        let command_buffer = context.command_buffers[current_frame];
        assert!(command_buffer != vk::CommandBuffer::null(), "Command buffer is null!");

        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };

        // 3. Grabar el command buffer.
        device.begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        let clear_color = vk::ClearValue {
            color: vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] }, // Negro
        };

        let render_pass_info = vk::RenderPassBeginInfo {
            render_pass: renderer.render_pass,
            framebuffer: renderer.framebuffers[image_index as usize],
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: context.swapchain_extent(),
            },
            clear_value_count: 1,
            p_clear_values: &clear_color,
            ..Default::default()
        };

        device.cmd_begin_render_pass(command_buffer, &render_pass_info, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, renderer.pipeline.pipeline);
        device.cmd_draw(command_buffer, 3, 1, 0, 0); // Dibujar el triángulo del shader.

        let mut query = Query::<&Transform>::new(world);
        for _ in query.iter() {
            // Aquí irían las llamadas de draw
        }

        device.cmd_end_render_pass(command_buffer);
        device.end_command_buffer(command_buffer)
            .expect("Failed to end command buffer");

        // 4. Enviar el command buffer a la GPU.
        let wait_semaphores = [context.image_available_semaphores[current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [context.render_finished_semaphores[current_frame]];

        let submit_info = vk::SubmitInfo {
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
            ..Default::default()
        };

        device.reset_fences(&[context.in_flight_fences[current_frame]]).unwrap();
        device.queue_submit(context.queue, &[submit_info], context.in_flight_fences[current_frame])
            .expect("Failed to submit queue");

        // 5. Presentar la imagen en pantalla.
        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(), // Esperar a que el renderizado termine.
            swapchain_count: 1,
            p_swapchains: &context.swapchain,
            p_image_indices: &image_index,
            ..Default::default()
        };

        let result = swapchain_loader.queue_present(context.queue, &present_info);

        match result {
            Ok(false) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                renderer.recreate_swapchain(window);
            }
            Err(e) => panic!("Failed to present swapchain image: {}", e),
            _ => {}
        }
    }
}
