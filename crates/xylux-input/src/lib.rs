use glam::Vec2;
use sdl3::event::Event;
use sdl3::mouse::MouseButton;
use std::collections::HashSet;

pub struct InputContext {
    pub mouse_position: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_delta: f32, // Y-scroll
    pub mouse_buttons: HashSet<MouseButton>,
    pub is_mouse_captured: bool,
}

impl InputContext {
    pub fn new() -> Self {
        Self {
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_delta: 0.0,
            mouse_buttons: HashSet::new(),
            is_mouse_captured: false,
        }
    }

    pub fn reset_per_frame(&mut self) {
        self.mouse_delta = Vec2::ZERO;
        self.scroll_delta = 0.0;
    }

    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::MouseMotion { x, y, xrel, yrel, .. } => {
                self.mouse_position = Vec2::new(*x, *y);
                self.mouse_delta = Vec2::new(*xrel, *yrel);
            }
            Event::MouseButtonDown { mouse_btn, .. } => {
                self.mouse_buttons.insert(*mouse_btn);
            }
            Event::MouseButtonUp { mouse_btn, .. } => {
                self.mouse_buttons.remove(mouse_btn);
            }
            Event::MouseWheel { y, .. } => {
                self.scroll_delta = *y;
            }
            _ => {}
        }
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }
}
