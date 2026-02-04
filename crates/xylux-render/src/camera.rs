use glam::{Mat4, Vec3};
use sdl3::event::Event;
use sdl3::mouse::MouseButton;

pub struct Camera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub aspect: f32,
    
    // State
    is_dragging: bool,
    last_mouse_pos: (f32, f32),
}

impl Camera {
    pub fn new(target: Vec3, distance: f32, aspect: f32) -> Self {
        Self {
            target,
            distance,
            yaw: -90.0f32.to_radians(),
            pitch: 0.0,
            fov: 45.0f32.to_radians(),
            aspect,
            is_dragging: false,
            last_mouse_pos: (0.0, 0.0),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if *mouse_btn == MouseButton::Left || *mouse_btn == MouseButton::Right {
                    self.is_dragging = true;
                    self.last_mouse_pos = (*x, *y);
                }
            }
            Event::MouseButtonUp { .. } => {
                self.is_dragging = false;
            }
            Event::MouseMotion { x, y, .. } => {
                if self.is_dragging {
                    let dx = *x - self.last_mouse_pos.0;
                    let dy = *y - self.last_mouse_pos.1;
                    
                    let sensitivity = 0.005;
                    self.yaw += dx * sensitivity;
                    self.pitch += dy * sensitivity;

                    // Clamp pitch
                    self.pitch = self.pitch.clamp(-1.5, 1.5);

                    self.last_mouse_pos = (*x, *y);
                }
            }
            Event::MouseWheel { y, .. } => {
                let scroll_sensitivity = 1.0;
                self.distance -= *y * scroll_sensitivity;
                self.distance = self.distance.clamp(1.0, 50.0);
            }
            _ => {}
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let x = self.distance * self.yaw.cos() * self.pitch.cos();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.yaw.sin() * self.pitch.cos();

        let eye = self.target + Vec3::new(x, y, z);
        Mat4::look_at_rh(eye, self.target, Vec3::Y)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        let mut proj = Mat4::perspective_rh(self.fov, self.aspect, 0.1, 100.0);
        proj.col_mut(1).y *= -1.0; // Vulkan flip
        proj
    }

    pub fn get_mvp(&self, model: Mat4) -> Mat4 {
        self.get_projection_matrix() * self.get_view_matrix() * model
    }
}
