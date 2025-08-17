use ash::{vk, Entry, Instance};
use ash_window::create_surface;
use sdl3::video::Window;
use std::ffi::CString;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub device: ash::Device,
    pub physical_device: vk::PhysicalDevice,
    pub queue: vk::Queue,
    pub surface: vk::SurfaceKHR,
    pub swapchain: vk::SwapchainKHR,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
}

impl VulkanContext {
    pub fn new(window: &Window) -> Self {
        let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry") };
        let instance = create_instance(&entry, window);

        let display_handle: raw_window_handle::RawDisplayHandle = window.display_handle().expect("Failed to get display handle").into();
        let window_handle: raw_window_handle::RawWindowHandle = window.window_handle().expect("Failed to get window handle").into();

        let surface = unsafe {
            create_surface(&entry, &instance, display_handle, window_handle, None)
                .expect("Failed to create surface")
        };

        let (physical_device, queue_family_index) = select_physical_device(&entry, &instance, surface);
        let (device, queue) = create_device(&instance, physical_device, queue_family_index);
        let (swapchain, swapchain_format, swapchain_extent, swapchain_images, swapchain_image_views) =
            create_swapchain(&entry, &instance, physical_device, &device, surface, window);
        let command_pool = create_command_pool(&device, queue_family_index);
        let command_buffers = create_command_buffers(&device, command_pool, swapchain_images.len());

        Self {
            entry,
            instance,
            device,
            physical_device,
            queue,
            surface,
            swapchain,
            swapchain_format,
            swapchain_extent,
            swapchain_images,
            swapchain_image_views,
            command_pool,
            command_buffers,
        }
    }

    // Métodos públicos para que Renderer acceda de forma segura
    pub fn swapchain_format(&self) -> vk::Format {
        self.swapchain_format
    }

    pub fn swapchain_extent(&self) -> vk::Extent2D {
        self.swapchain_extent
    }

    pub fn swapchain_image_views(&self) -> &[vk::ImageView] {
        &self.swapchain_image_views
    }

    pub fn cleanup(&self) {
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);

            for &image_view in &self.swapchain_image_views {
                self.device.destroy_image_view(image_view, None);
            }

            let swapchain_loader = ash::khr::swapchain::Device::new(&self.instance, &self.device);
            swapchain_loader.destroy_swapchain(self.swapchain, None);

            self.device.destroy_device(None);

            let surface_loader = ash::khr::surface::Instance::new(&self.entry, &self.instance);
            surface_loader.destroy_surface(self.surface, None);

            self.instance.destroy_instance(None);
        }
    }
}

// ===========================================
// Funciones auxiliares
// ===========================================

fn create_instance(entry: &Entry, window: &Window) -> Instance {
    let app_name = CString::new("Xylux Engine").unwrap();
    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        application_version: vk::make_api_version(0, 1, 0, 0),
        api_version: vk::API_VERSION_1_3,
        ..Default::default()
    };

    let display_handle: raw_window_handle::RawDisplayHandle = window.display_handle().expect("Failed to get display handle").into();
    let extensions = ash_window::enumerate_required_extensions(display_handle)
        .expect("Failed to enumerate required extensions");

    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
        ..Default::default()
    };

    unsafe { entry.create_instance(&create_info, None).unwrap() }
}

fn select_physical_device(entry: &Entry, instance: &Instance, surface: vk::SurfaceKHR) -> (vk::PhysicalDevice, u32) {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    let surface_loader = ash::khr::surface::Instance::new(entry, instance);

    for device in devices {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(device) };
        for (index, family) in queue_families.iter().enumerate() {
            if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                let supports_surface = unsafe {
                    surface_loader.get_physical_device_surface_support(device, index as u32, surface).unwrap()
                };
                if supports_surface {
                    return (device, index as u32);
                }
            }
        }
    }
    panic!("No suitable GPU found");
}

fn create_device(instance: &Instance, physical_device: vk::PhysicalDevice, queue_family_index: u32) -> (ash::Device, vk::Queue) {
    let queue_priorities = [1.0f32];
    let queue_info = vk::DeviceQueueCreateInfo {
        queue_family_index,
        queue_count: 1,
        p_queue_priorities: queue_priorities.as_ptr(),
        ..Default::default()
    };

    let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];

    let device_info = vk::DeviceCreateInfo {
        queue_create_info_count: 1,
        p_queue_create_infos: &queue_info,
        enabled_extension_count: device_extensions.len() as u32,
        pp_enabled_extension_names: device_extensions.as_ptr(),
        ..Default::default()
    };

    let device = unsafe { instance.create_device(physical_device, &device_info, None).unwrap() };
    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
    (device, queue)
}

// ===========================================
// Creamos swapchain y devolvemos toda la info necesaria
// ===========================================
fn create_swapchain(
    entry: &Entry,
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    surface: vk::SurfaceKHR,
    window: &Window,
) -> (vk::SwapchainKHR, vk::Format, vk::Extent2D, Vec<vk::Image>, Vec<vk::ImageView>) {
    let surface_loader = ash::khr::surface::Instance::new(entry, instance);
    let capabilities = unsafe { surface_loader.get_physical_device_surface_capabilities(physical_device, surface).unwrap() };
    let formats = unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface).unwrap() };

    let surface_format = formats
        .iter()
        .find(|f| f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
        .unwrap_or(&formats[0]);

    let extent = if capabilities.current_extent.width != std::u32::MAX {
        capabilities.current_extent
    } else {
        let size = window.size();
        vk::Extent2D {
            width: size.0.clamp(capabilities.min_image_extent.width, capabilities.max_image_extent.width),
            height: size.1.clamp(capabilities.min_image_extent.height, capabilities.max_image_extent.height),
        }
    };

    let image_count = (capabilities.min_image_count + 1).min(capabilities.max_image_count);

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
        present_mode: vk::PresentModeKHR::FIFO,
        clipped: vk::TRUE,
        ..Default::default()
    };

    let swapchain_loader = ash::khr::swapchain::Device::new(instance, device);
    let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_info, None).unwrap() };
    let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

    let mut swapchain_image_views = Vec::with_capacity(swapchain_images.len());
    for &image in &swapchain_images {
        let subresource_range = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };
        let components = vk::ComponentMapping {
            r: vk::ComponentSwizzle::IDENTITY,
            g: vk::ComponentSwizzle::IDENTITY,
            b: vk::ComponentSwizzle::IDENTITY,
            a: vk::ComponentSwizzle::IDENTITY,
        };
        let create_info = vk::ImageViewCreateInfo {
            image,
            view_type: vk::ImageViewType::TYPE_2D,
            format: surface_format.format,
            components,
            subresource_range,
            ..Default::default()
        };
        let image_view = unsafe { device.create_image_view(&create_info, None).unwrap() };
        swapchain_image_views.push(image_view);
    }

    (swapchain, surface_format.format, extent, swapchain_images, swapchain_image_views)
}

fn create_command_pool(device: &ash::Device, queue_family_index: u32) -> vk::CommandPool {
    let pool_info = vk::CommandPoolCreateInfo {
        queue_family_index,
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
        ..Default::default()
    };
    unsafe { device.create_command_pool(&pool_info, None).unwrap() }
}

fn create_command_buffers(device: &ash::Device, command_pool: vk::CommandPool, count: usize) -> Vec<vk::CommandBuffer> {
    let alloc_info = vk::CommandBufferAllocateInfo {
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: count as u32,
        ..Default::default()
    };
    unsafe { device.allocate_command_buffers(&alloc_info).unwrap() }
}
