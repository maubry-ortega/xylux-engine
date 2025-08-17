use ash::{vk, Entry, Instance};
use sdl3::video::Window;

pub fn create_swapchain(
    entry: &Entry,
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    surface: vk::SurfaceKHR,
    window: &Window,
) -> (
    vk::SwapchainKHR,
    vk::Format,
    vk::Extent2D,
    Vec<vk::Image>,
    Vec<vk::ImageView>,
) {
    // Loader surface y swapchain
    let surface_loader = ash::khr::surface::Instance::new(entry, instance);
    let swapchain_loader = ash::khr::swapchain::Device::new(instance, device);

    let formats = unsafe {
        surface_loader.get_physical_device_surface_formats(physical_device, surface)
            .expect("Failed to get surface formats")
    };
    let surface_format = formats.iter().find(|f| f.format == vk::Format::B8G8R8A8_SRGB)
        .cloned().unwrap_or(formats[0]);

    let present_modes = unsafe {
        surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
            .expect("Failed to get present modes")
    };
    let present_mode = present_modes.iter().cloned().find(|&m| m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO);

    let caps = unsafe {
        surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
            .expect("Failed to get surface capabilities")
    };

    let (w, h) = window.size();
    let extent = if caps.current_extent.width != u32::MAX {
        caps.current_extent
    } else {
        vk::Extent2D {
            width: w.clamp(caps.min_image_extent.width, caps.max_image_extent.width),
            height: h.clamp(caps.min_image_extent.height, caps.max_image_extent.height),
        }
    };

    let mut image_count = (caps.min_image_count + 1).min(if caps.max_image_count > 0 { caps.max_image_count } else { u32::MAX });

    let swapchain_ci = vk::SwapchainCreateInfoKHR {
        surface,
        min_image_count: image_count,
        image_format: surface_format.format,
        image_color_space: surface_format.color_space,
        image_extent: extent,
        image_array_layers: 1,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode: vk::SharingMode::EXCLUSIVE,
        pre_transform: caps.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode,
        clipped: vk::TRUE,
        old_swapchain: vk::SwapchainKHR::null(),
        ..Default::default()
    };

    let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_ci, None).expect("Failed to create swapchain") };
    let images = unsafe { swapchain_loader.get_swapchain_images(swapchain).expect("Failed to get swapchain images") };
    let image_views: Vec<vk::ImageView> = images.iter().map(|&image| {
        let view_ci = vk::ImageViewCreateInfo {
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
        unsafe { device.create_image_view(&view_ci, None).expect("Failed to create image view") }
    }).collect();

    (swapchain, surface_format.format, extent, images, image_views)
}
