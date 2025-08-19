use ash::{vk, Entry, Instance, Device};
use ash_window::create_surface;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use xylux_window::XyluxWindow;

use super::{
    instance::create_instance,
    device::{select_physical_device, create_device},
    swapchain::create_swapchain,
    command::{create_command_pool, create_command_buffers},
};

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,
    pub queue: vk::Queue,
    pub surface: vk::SurfaceKHR,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,

    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub images_in_flight: Vec<vk::Fence>,
}

impl VulkanContext {
    pub fn new(window: &XyluxWindow) -> Self {
        // 1️⃣ Cargar Vulkan entry
        let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry") };

        // 2️⃣ Crear instancia
        let instance = create_instance(&entry, window);

        // 3️⃣ Crear surface multiplataforma usando tu abstracción
        let display_handle = window.window.display_handle().expect("Failed to get display handle").into();
        let window_handle = window.window.window_handle().expect("Failed to get window handle").into();
        let surface = unsafe {
            create_surface(&entry, &instance, display_handle, window_handle, None)
                .expect("Failed to create Vulkan surface")
        };

        // 4️⃣ Elegir physical device y queue
        let (physical_device, queue_family_index) = select_physical_device(&entry, &instance, surface);
        let (device, queue) = create_device(&instance, physical_device, queue_family_index);

        // 5️⃣ Crear swapchain usando tu abstracción
        let (swapchain, swapchain_format, swapchain_extent, swapchain_images, swapchain_image_views) =
            create_swapchain(&entry, &instance, physical_device, &device, surface, window);

        // 6️⃣ Crear command pool y buffers
        let command_pool = create_command_pool(&device, queue_family_index);
        let command_buffers = create_command_buffers(&device, command_pool, MAX_FRAMES_IN_FLIGHT);

        // 7️⃣ Crear objetos de sincronización
        let mut image_available_semaphores = vec![];
        let mut render_finished_semaphores = vec![];
        let mut in_flight_fences = vec![];
        let images_in_flight = swapchain_images.iter().map(|_| vk::Fence::null()).collect();

        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                image_available_semaphores
                    .push(device.create_semaphore(&semaphore_info, None).unwrap());
                render_finished_semaphores
                    .push(device.create_semaphore(&semaphore_info, None).unwrap());
                in_flight_fences.push(device.create_fence(&fence_info, None).unwrap());
            }
        }

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
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            images_in_flight,
        }
    }

    pub fn swapchain_format(&self) -> vk::Format {
        self.swapchain_format
    }

    pub fn swapchain_extent(&self) -> vk::Extent2D {
        self.swapchain_extent
    }

    pub fn swapchain_image_views(&self) -> &[vk::ImageView] {
        &self.swapchain_image_views
    }

    pub fn cleanup_swapchain_resources(&self) {
        unsafe {
            for &image_view in &self.swapchain_image_views {
                self.device.destroy_image_view(image_view, None);
            }
            super::swapchain::destroy_swapchain(&self.instance, &self.device, self.swapchain);
        }
    }

    pub fn recreate_swapchain_resources(&mut self, window: &XyluxWindow) {
        let (swapchain, format, extent, images, image_views) =
            create_swapchain(&self.entry, &self.instance, self.physical_device, &self.device, self.surface, window);

        self.swapchain = swapchain;
        self.swapchain_format = format;
        self.swapchain_extent = extent;
        self.swapchain_images = images;
        self.swapchain_image_views = image_views;
        self.images_in_flight = self.swapchain_images.iter().map(|_| vk::Fence::null()).collect();
    }

    pub fn cleanup(&self) {
        unsafe {
            // 0️⃣ Destruir objetos de sincronización
            for i in 0..MAX_FRAMES_IN_FLIGHT {
                self.device.destroy_semaphore(self.render_finished_semaphores[i], None);
                self.device.destroy_semaphore(self.image_available_semaphores[i], None);
                self.device.destroy_fence(self.in_flight_fences[i], None);
            }

            // 1️⃣ Destruir command pool
            self.device.destroy_command_pool(self.command_pool, None);

            // 2️⃣ Destruir surface
            let surface_loader = ash::khr::surface::Instance::new(&self.entry, &self.instance);
            surface_loader.destroy_surface(self.surface, None);

            // 5️⃣ Destruir device
            self.device.destroy_device(None);

            // 6️⃣ Destruir instancia
            self.instance.destroy_instance(None);
        }
    }
}
