use ash::{vk, Entry, Instance, Device};
use ash::khr::{surface, swapchain};
use xylux_window::XyluxWindow;

pub fn create_swapchain(
    entry: &Entry,
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    device: &Device,
    surface: vk::SurfaceKHR,
    window: &XyluxWindow,
) -> (vk::SwapchainKHR, vk::Format, vk::Extent2D, Vec<vk::Image>, Vec<vk::ImageView>) {

    let surface_loader = surface::Instance::new(entry, instance);

    let formats = unsafe {
        surface_loader
            .get_physical_device_surface_formats(physical_device, surface)
            .expect("Failed to get surface formats")
    };
    let surface_format = formats
        .iter()
        .find(|f| f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
        .cloned()
        .unwrap_or(formats[0]);

    let present_modes = unsafe {
        surface_loader
            .get_physical_device_surface_present_modes(physical_device, surface)
            .expect("Failed to get present modes")
    };
    let present_mode = present_modes
        .iter()
        .cloned()
        .find(|&m| m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO);

    let capabilities = unsafe {
        surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface)
            .expect("Failed to get surface capabilities")
    };

    let extent = if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    } else {
        let (width, height) = window.window.size();
        vk::Extent2D {
            width: width.clamp(capabilities.min_image_extent.width, capabilities.max_image_extent.width),
            height: height.clamp(capabilities.min_image_extent.height, capabilities.max_image_extent.height),
        }
    };

    let image_count = (capabilities.min_image_count + 1).min(
        if capabilities.max_image_count > 0 { capabilities.max_image_count } else { u32::MAX },
    );

    let swapchain_info = vk::SwapchainCreateInfoKHR {
        surface,
        min_image_count: image_count,
        image_format: surface_format.format,
        image_color_space: surface_format.color_space,
        image_extent: extent,
        image_array_layers: 1,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode: vk::SharingMode::EXCLUSIVE,
        pre_transform: capabilities.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode,
        clipped: vk::TRUE,
        ..Default::default()
    };

    let swapchain_loader = swapchain::Device::new(instance, device);
    let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_info, None).expect("Failed to create swapchain") };
    let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).expect("Failed to get swapchain images") };

    let swapchain_image_views: Vec<vk::ImageView> = swapchain_images
        .iter()
        .map(|&image| {
            let create_info = vk::ImageViewCreateInfo {
                image,
                view_type: vk::ImageViewType::TYPE_2D,
                format: surface_format.format,
                components: vk::ComponentMapping::default(),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };
            unsafe { device.create_image_view(&create_info, None).expect("Failed to create image view") }
        })
        .collect();

    (swapchain, surface_format.format, extent, swapchain_images, swapchain_image_views)
}

pub fn destroy_swapchain(
    instance: &Instance,
    device: &Device,
    swapchain: vk::SwapchainKHR,
) {
    let swapchain_loader = swapchain::Device::new(instance, device);
    unsafe { swapchain_loader.destroy_swapchain(swapchain, None) };
}
