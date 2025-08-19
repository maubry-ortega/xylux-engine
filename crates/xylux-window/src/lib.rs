use sdl3::video::Window;
use sdl3::Sdl;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

pub struct XyluxWindow {
    pub sdl: Sdl,
    pub window: Window,
}

impl XyluxWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl = sdl3::init().expect("Failed to init SDL3");
        let video = sdl.video().expect("Failed to get SDL3 video subsystem");

        let window = video
            .window(title, width, height)
            .vulkan()
            .resizable()
            .build()
            .expect("Failed to create SDL3 window");

        Self { sdl, window }
    }

    pub fn run_loop<F>(&mut self, mut frame_callback: F)
    where
        F: FnMut(&XyluxWindow),
    {
        let mut event_pump = self.sdl.event_pump().expect("Failed to get event pump");

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }

            frame_callback(self);
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}