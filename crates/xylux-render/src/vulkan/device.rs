use ash::{vk, Instance};
use ash::Entry;

pub fn select_physical_device(entry: &Entry, instance: &Instance, surface: vk::SurfaceKHR) -> (vk::PhysicalDevice, u32) {
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

pub fn create_device(instance: &Instance, physical_device: vk::PhysicalDevice, queue_family_index: u32) -> (ash::Device, vk::Queue) {
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
