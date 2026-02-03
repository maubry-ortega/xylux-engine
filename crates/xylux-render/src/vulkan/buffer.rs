use ash::{vk, Instance, Device};

pub fn create_buffer(
    instance: &Instance,
    device: &Device,
    physical_device: vk::PhysicalDevice,
    size: vk::DeviceSize,
    _usage: vk::ImageUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> (vk::Buffer, vk::DeviceMemory) {
    let buffer_info = vk::BufferCreateInfo {
        size,
        usage: vk::BufferUsageFlags::VERTEX_BUFFER,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        ..Default::default()
    };

    let buffer = unsafe { device.create_buffer(&buffer_info, None).unwrap() };

    let mem_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
    let memory_type_index = super::image::find_memory_type(
        instance,
        physical_device,
        mem_requirements.memory_type_bits,
        properties,
    );

    let alloc_info = vk::MemoryAllocateInfo {
        allocation_size: mem_requirements.size,
        memory_type_index,
        ..Default::default()
    };

    let buffer_memory = unsafe { device.allocate_memory(&alloc_info, None).unwrap() };

    unsafe {
        device.bind_buffer_memory(buffer, buffer_memory, 0).unwrap();
    }

    (buffer, buffer_memory)
}

pub fn copy_data_to_buffer<T: Copy>(
    device: &Device,
    buffer_memory: vk::DeviceMemory,
    data: &[T],
) {
    let size = (data.len() * std::mem::size_of::<T>()) as vk::DeviceSize;
    unsafe {
        let ptr = device
            .map_memory(buffer_memory, 0, size, vk::MemoryMapFlags::empty())
            .unwrap();
        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut T, data.len());
        device.unmap_memory(buffer_memory);
    }
}
