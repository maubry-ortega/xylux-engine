use ash::vk;
use ash::Device;

pub fn create_command_pool(device: &Device, queue_family_index: u32) -> vk::CommandPool {
    let pool_info = vk::CommandPoolCreateInfo {
        queue_family_index,
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
        ..Default::default()
    };

    unsafe {
        device
            .create_command_pool(&pool_info, None)
            .expect("Failed to create command pool")
    }
}

pub fn create_command_buffers(
    device: &Device,
    command_pool: vk::CommandPool,
    count: usize,
) -> Vec<vk::CommandBuffer> {
    let alloc_info = vk::CommandBufferAllocateInfo {
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: count as u32,
        ..Default::default()
    };

    unsafe {
        device
            .allocate_command_buffers(&alloc_info)
            .expect("Failed to allocate command buffers")
    }
}

pub fn cleanup_command_pool(device: &Device, command_pool: vk::CommandPool) {
    unsafe {
        device.destroy_command_pool(command_pool, None);
    }
}
