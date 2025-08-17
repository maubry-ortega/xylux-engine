use ash::{vk, Entry, Instance};
use sdl3::video::Window;
use std::ffi::CString;
use raw_window_handle::HasDisplayHandle;

pub fn create_instance(entry: &Entry, window: &Window) -> Instance {
    let app_name = CString::new("Xylux Engine").unwrap();
    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        application_version: vk::make_api_version(0, 1, 0, 0),
        api_version: vk::API_VERSION_1_3,
        ..Default::default()
    };

    let display_handle = window.display_handle().unwrap().into();
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
