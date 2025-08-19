use ash::{vk, Entry, Instance};
use xylux_window::XyluxWindow;
use std::ffi::{c_char, CStr, CString};

/// Crea la instancia Vulkan usando XyluxWindow
pub fn create_instance(entry: &Entry, window: &XyluxWindow) -> Instance {
    // Información de la aplicación
    let app_name = CString::new("Xylux Engine").unwrap();
    let engine_name = CString::new("Xylux Engine").unwrap();
    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        application_version: vk::make_api_version(0, 1, 0, 0),
        p_engine_name: engine_name.as_ptr(),
        engine_version: vk::make_api_version(0, 1, 0, 0),
        api_version: vk::API_VERSION_1_3,
        ..Default::default()
    };

    // --- Capas de validación (solo en debug) ---
    let validation_layers = [CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    let validation_layer_names: Vec<*const c_char> = if cfg!(debug_assertions) {
        println!("[Debug] Habilitando capas de validación de Vulkan.");
        validation_layers.iter().map(|name| name.as_ptr()).collect()
    } else {
        Vec::new()
    };

    // --- Extensiones ---
    // Extensiones requeridas por la ventana (SDL3/winit)
    let mut extensions = window
        .window
        .vulkan_instance_extensions()
        .expect("Failed to enumerate required extensions");

    // Añadir extensión de debug si las capas de validación están activas
    if cfg!(debug_assertions) {
        extensions.push(ash::ext::debug_utils::NAME.to_str().unwrap().to_string());
    }

    // --- Diagnóstico: Imprimir extensiones disponibles y solicitadas ---
    if cfg!(debug_assertions) {
        let available_extensions = unsafe {
            entry
                .enumerate_instance_extension_properties(None)
                .expect("Failed to enumerate instance extensions")
        };

        println!("\n--- Extensiones de Instancia Vulkan Disponibles ---");
        for ext in &available_extensions {
            let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
            println!("  - {:?}", name);
        }

        println!("\n--- Extensiones de Instancia Vulkan Solicitadas ---");
        for ext_name in &extensions {
            println!("  - {}", ext_name);
        }
        println!("---\n");
    }

    // Convertir a punteros *const c_char para la API de Vulkan
    let extension_names_c: Vec<CString> = extensions
        .iter()
        .map(|ext| CString::new(ext.as_str()).unwrap())
        .collect();

    let extension_names_ptr: Vec<*const c_char> =
        extension_names_c.iter().map(|cstr| cstr.as_ptr()).collect();

    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        enabled_layer_count: validation_layer_names.len() as u32,
        pp_enabled_layer_names: validation_layer_names.as_ptr(),
        enabled_extension_count: extension_names_ptr.len() as u32,
        pp_enabled_extension_names: extension_names_ptr.as_ptr(),
        ..Default::default()
    };

    // Crear instancia Vulkan
    let instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create Vulkan instance")
    };

    instance
}
