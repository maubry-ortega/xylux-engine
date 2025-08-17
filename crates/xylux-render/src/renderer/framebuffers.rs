use ash::vk;

pub fn create_framebuffers(
    device: &ash::Device,
    render_pass: vk::RenderPass,
    image_views: &[vk::ImageView],
    extent: vk::Extent2D,
) -> Vec<vk::Framebuffer> {
    image_views.iter().map(|&image_view| {
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
        unsafe { device.create_framebuffer(&create_info, None).expect("Failed to create framebuffer") }
    }).collect()
}
