use ash::{vk, Instance, Device};

pub fn create_image(
    instance: &Instance,
    device: &Device,
    physical_device: vk::PhysicalDevice,
    width: u32,
    height: u32,
    format: vk::Format,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> (vk::Image, vk::DeviceMemory) {
    let image_info = vk::ImageCreateInfo {
        image_type: vk::ImageType::TYPE_2D,
        extent: vk::Extent3D {
            width,
            height,
            depth: 1,
        },
        mip_levels: 1,
        array_layers: 1,
        format,
        tiling,
        initial_layout: vk::ImageLayout::UNDEFINED,
        usage,
        samples: vk::SampleCountFlags::TYPE_1,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        ..Default::default()
    };

    let image = unsafe { device.create_image(&image_info, None).unwrap() };

    let mem_requirements = unsafe { device.get_image_memory_requirements(image) };
    let memory_type_index = find_memory_type(
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

    let image_memory = unsafe { device.allocate_memory(&alloc_info, None).unwrap() };

    unsafe {
        device.bind_image_memory(image, image_memory, 0).unwrap();
    }

    (image, image_memory)
}

pub fn create_image_view(
    device: &Device,
    image: vk::Image,
    format: vk::Format,
    aspect_flags: vk::ImageAspectFlags,
) -> vk::ImageView {
    let view_info = vk::ImageViewCreateInfo {
        image,
        view_type: vk::ImageViewType::TYPE_2D,
        format,
        subresource_range: vk::ImageSubresourceRange {
            aspect_mask: aspect_flags,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        },
        ..Default::default()
    };

    unsafe { device.create_image_view(&view_info, None).unwrap() }
}

pub fn find_memory_type(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> u32 {
    let mem_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };

    for i in 0..mem_properties.memory_type_count {
        if (type_filter & (1 << i)) != 0
            && (mem_properties.memory_types[i as usize].property_flags & properties) == properties
        {
            return i;
        }
    }

    panic!("Failed to find suitable memory type");
}

pub fn find_depth_format(instance: &Instance, physical_device: vk::PhysicalDevice) -> vk::Format {
    find_supported_format(
        instance,
        physical_device,
        &[vk::Format::D32_SFLOAT, vk::Format::D32_SFLOAT_S8_UINT, vk::Format::D24_UNORM_S8_UINT],
        vk::ImageTiling::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}

fn find_supported_format(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    candidates: &[vk::Format],
    tiling: vk::ImageTiling,
    features: vk::FormatFeatureFlags,
) -> vk::Format {
    for &format in candidates {
        let props = unsafe { instance.get_physical_device_format_properties(physical_device, format) };

        if tiling == vk::ImageTiling::LINEAR && (props.linear_tiling_features & features) == features {
            return format;
        } else if tiling == vk::ImageTiling::OPTIMAL && (props.optimal_tiling_features & features) == features {
            return format;
        }
    }

    panic!("Failed to find supported format");
}
