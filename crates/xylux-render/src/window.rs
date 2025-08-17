use sdl3::video::{Window};
use sdl3::Sdl;


pub struct XyluxWindow {
    pub sdl: Sdl,
    pub window: Window,
}

impl XyluxWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl = sdl3::init().expect("Failed to init SDL3");
        let video = sdl.video().expect("Failed to get SDL3 video subsystem");

        let window = video.window(title, width, height)
            .vulkan()
            .resizable()
            .build()
            .expect("Failed to create SDL3 window");

        Self { sdl, window }
    }
}
