use ash::vk;
use std::fs;
use std::io::Cursor;
use std::path::Path;

pub struct Pipeline {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}

impl Pipeline {
    pub fn new(device: &ash::Device, render_pass: vk::RenderPass, extent: vk::Extent2D) -> Self {
        // Ruta absoluta a la carpeta de shaders
        let shader_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("shaders");
        let vertex_shader_path = shader_dir.join("shader.vert.spv");
        let fragment_shader_path = shader_dir.join("shader.frag.spv");

        let vertex_shader_code = fs::read(&vertex_shader_path)
            .unwrap_or_else(|_| panic!("Failed to read vertex shader: {:?}", vertex_shader_path));
        let fragment_shader_code = fs::read(&fragment_shader_path)
            .unwrap_or_else(|_| panic!("Failed to read fragment shader: {:?}", fragment_shader_path));

        let vertex_shader_module = create_shader_module(device, &vertex_shader_code);
        let fragment_shader_module = create_shader_module(device, &fragment_shader_code);

        let entry_point = std::ffi::CString::new("main").unwrap();
        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                module: vertex_shader_module,
                p_name: entry_point.as_ptr(),
                stage: vk::ShaderStageFlags::VERTEX,
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                module: fragment_shader_module,
                p_name: entry_point.as_ptr(),
                stage: vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            },
        ];

        // Configuración básica de pipeline
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: 0,
            ..Default::default()
        };

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent,
        };

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            p_viewports: &viewport,
            scissor_count: 1,
            p_scissors: &scissor,
            ..Default::default()
        };

        let rasterizer = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: 0,
            rasterizer_discard_enable: 0,
            polygon_mode: vk::PolygonMode::FILL,
            line_width: 1.0,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::CLOCKWISE,
            ..Default::default()
        };

        let multisampling = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::ONE,
            dst_color_blend_factor: vk::BlendFactor::ZERO,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        };

        let color_blending = vk::PipelineColorBlendStateCreateInfo {
            attachment_count: 1,
            p_attachments: &color_blend_attachment,
            ..Default::default()
        };

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe {
            device.create_pipeline_layout(&pipeline_layout_info, None).unwrap()
        };

        let pipeline_info = vk::GraphicsPipelineCreateInfo {
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_input_info,
            p_input_assembly_state: &input_assembly,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &rasterizer,
            p_multisample_state: &multisampling,
            p_depth_stencil_state: std::ptr::null(),
            p_color_blend_state: &color_blending,
            layout: pipeline_layout,
            render_pass,
            subpass: 0,
            ..Default::default()
        };

        let pipeline = unsafe {
            device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .unwrap()[0]
        };

        unsafe {
            device.destroy_shader_module(vertex_shader_module, None);
            device.destroy_shader_module(fragment_shader_module, None);
        }

        Self {
            pipeline,
            pipeline_layout,
        }
    }

    pub fn cleanup(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

fn create_shader_module(device: &ash::Device, code: &[u8]) -> vk::ShaderModule {
    // El código del shader debe estar alineado para un slice de `u32`.
    // `ash::util::read_spv` se encarga de esto de forma segura.
    let code_aligned =
        ash::util::read_spv(&mut Cursor::new(code)).expect("Failed to read SPIR-V code");
    let create_info = vk::ShaderModuleCreateInfo {
        code_size: code_aligned.len() * std::mem::size_of::<u32>(),
        p_code: code_aligned.as_ptr(),
        ..Default::default()
    };
    unsafe { device.create_shader_module(&create_info, None).unwrap() }
}
