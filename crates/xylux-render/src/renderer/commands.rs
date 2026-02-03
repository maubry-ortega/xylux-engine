use ash::vk;
use crate::renderer::renderer::Renderer;
use xylux_ecs::World;
use xylux_window::XyluxWindow;

// Ya no usamos 'extensions::khr::Swapchain', usamos el módulo público swapchain
use ash::khr::swapchain;

pub fn render_frame(renderer: &mut Renderer, _world: &mut World, window: &XyluxWindow, camera: &crate::camera::Camera) {
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

        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue { float32: [0.8, 0.8, 0.2, 1.0] }, // Amarillo 'escena 3D'
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 },
            },
        ];

        let render_pass_info = vk::RenderPassBeginInfo {
            render_pass: renderer.render_pass,
            framebuffer: renderer.framebuffers[image_index as usize],
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: context.swapchain_extent(),
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
            ..Default::default()
        };

        device.cmd_begin_render_pass(command_buffer, &render_pass_info, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, renderer.pipeline.pipeline);

        // --- CÁLCULO DE MVP ---
        let model = glam::Mat4::IDENTITY; // Model en el origen, sin rotación por ahora (o controlada externamente)
        let mvp = camera.get_mvp(model);

        // Pasar MVP vía Push Constant
        let constants_bytes: &[u8] = std::slice::from_raw_parts(
            &mvp as *const glam::Mat4 as *const u8,
            std::mem::size_of::<glam::Mat4>(),
        );
        
        device.cmd_push_constants(
            command_buffer,
            renderer.pipeline.pipeline_layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            constants_bytes,
        );

        // Enlazar Vertex Buffer
        if renderer.vertex_buffer != vk::Buffer::null() {
            let buffers = [renderer.vertex_buffer];
            let offsets = [0];
            device.cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &offsets);

            // PASS 1: Escena 3D (Grid + Modelo)
            // Se dibuja desde 0 hasta ui_start_index
            let scene_count = if renderer.ui_start_index > 0 { 
                renderer.ui_start_index 
            } else { 
                renderer.vertex_count 
            };

            if scene_count > 0 {
                device.cmd_draw(command_buffer, scene_count, 1, 0, 0);
            }

            // PASS 2: UI Overlay (Panel Lateral)
            // Se dibuja desde ui_start_index hasta vertex_count
            // Usamos Matriz Identidad para renderizar en pantalla completa (NDC)
            if renderer.ui_start_index < renderer.vertex_count {
                let ui_count = renderer.vertex_count - renderer.ui_start_index;
                let identity_mvp = glam::Mat4::IDENTITY;
                
                let ui_constants: &[u8] = std::slice::from_raw_parts(
                    &identity_mvp as *const glam::Mat4 as *const u8,
                    std::mem::size_of::<glam::Mat4>(),
                );
                
                device.cmd_push_constants(
                    command_buffer,
                    renderer.pipeline.pipeline_layout,
                    vk::ShaderStageFlags::VERTEX,
                    0,
                    ui_constants,
                );

                device.cmd_draw(command_buffer, ui_count, 1, renderer.ui_start_index, 0);
            }
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
