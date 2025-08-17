use ash::{vk, Entry, Instance};
use ash_window::create_surface;
use sdl3::video::Window;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use super::{
    instance::create_instance,
    device::{select_physical_device, create_device},
    swapchain::create_swapchain,
    command::{create_command_pool, create_command_buffers},
};

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

        let display_handle = window.display_handle().unwrap().into();
        let window_handle = window.window_handle().unwrap().into();
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

    pub fn swapchain_format(&self) -> vk::Format { self.swapchain_format }
    pub fn swapchain_extent(&self) -> vk::Extent2D { self.swapchain_extent }
    pub fn swapchain_image_views(&self) -> &[vk::ImageView] { &self.swapchain_image_views }

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
