use ash::vk;
use std::fs;

pub struct Pipeline {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
    pub render_pass: vk::RenderPass,
}

impl Pipeline {
    pub fn new(device: &ash::Device, swapchain_format: vk::Format) -> Self {
    let vertex_shader_code = fs::read("shaders/triangle.vert.spv").unwrap();
    let fragment_shader_code = fs::read("shaders/triangle.frag.spv").unwrap();

    let vertex_shader_module = create_shader_module(device, &vertex_shader_code);
    let fragment_shader_module = create_shader_module(device, &fragment_shader_code);

    let entry_point = std::ffi::CString::new("main").unwrap();
    let vertex_shader_stage = vk::PipelineShaderStageCreateInfo {
        module: vertex_shader_module,
        p_name: entry_point.as_ptr(),
        stage: vk::ShaderStageFlags::VERTEX,
        ..Default::default()
    };
    let fragment_shader_stage = vk::PipelineShaderStageCreateInfo {
        module: fragment_shader_module,
        p_name: entry_point.as_ptr(),
        stage: vk::ShaderStageFlags::FRAGMENT,
        ..Default::default()
    };

    let shader_stages = [vertex_shader_stage, fragment_shader_stage];

    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo {
        vertex_binding_description_count: 0,
        p_vertex_binding_descriptions: std::ptr::null(),
        vertex_attribute_description_count: 0,
        p_vertex_attribute_descriptions: std::ptr::null(),
        ..Default::default()
    };

    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: 0,
        ..Default::default()
    };

    let viewport = vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: 800.0, // Deberías usar el extent real del swapchain
        height: 600.0,
        min_depth: 0.0,
        max_depth: 1.0,
    };

    let scissor = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: vk::Extent2D { width: 800, height: 600 },
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
        depth_bias_enable: 0,
        depth_bias_constant_factor: 0.0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
        ..Default::default()
    };

    let multisampling = vk::PipelineMultisampleStateCreateInfo {
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        sample_shading_enable: 0,
        min_sample_shading: 1.0,
        p_sample_mask: std::ptr::null(),
        alpha_to_coverage_enable: 0,
        alpha_to_one_enable: 0,
        ..Default::default()
    };

    let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
        blend_enable: 0, // Cambiado de false a 0 (u32)
        src_color_blend_factor: vk::BlendFactor::ONE,
        dst_color_blend_factor: vk::BlendFactor::ZERO,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::ONE,
        dst_alpha_blend_factor: vk::BlendFactor::ZERO,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::RGBA,
    };

    let color_blending = vk::PipelineColorBlendStateCreateInfo {
        logic_op_enable: 0,
        logic_op: vk::LogicOp::COPY,
        attachment_count: 1,
        p_attachments: &color_blend_attachment,
        blend_constants: [0.0, 0.0, 0.0, 0.0],
        ..Default::default()
    };

    let pipeline_layout_info = vk::PipelineLayoutCreateInfo {
        set_layout_count: 0,
        p_set_layouts: std::ptr::null(),
        push_constant_range_count: 0,
        p_push_constant_ranges: std::ptr::null(),
        ..Default::default()
    };

    let pipeline_layout = unsafe { 
        device.create_pipeline_layout(&pipeline_layout_info, None).unwrap() 
    };

    let render_pass = create_render_pass(device, swapchain_format);

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
        p_dynamic_state: std::ptr::null(),
        layout: pipeline_layout,
        render_pass,
        subpass: 0,
        base_pipeline_handle: vk::Pipeline::null(),
        base_pipeline_index: -1,
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
        render_pass,
    }
}

    pub fn cleanup(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);
        }
    }
}

fn create_shader_module(device: &ash::Device, code: &[u8]) -> vk::ShaderModule {
    let create_info = vk::ShaderModuleCreateInfo {
        code_size: code.len(),
        p_code: code.as_ptr() as *const u32,
        ..Default::default()
    };
    unsafe { device.create_shader_module(&create_info, None).unwrap() }
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