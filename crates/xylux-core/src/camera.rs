use glam::{Mat4, Vec3};
use sdl3::mouse::MouseButton;

pub struct Camera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub aspect: f32,
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
        }
    }

    pub fn update(&mut self, input: &xylux_input::InputContext) {
        // Handle Rotation (Left or Right Click Drag)
        if input.is_button_pressed(MouseButton::Left) || input.is_button_pressed(MouseButton::Right) {
             let sensitivity = 0.005;
             self.yaw += input.mouse_delta.x * sensitivity;
             self.pitch += input.mouse_delta.y * sensitivity;

             // Clamp pitch
             self.pitch = self.pitch.clamp(-1.5, 1.5);
        }
        
        // Handle Zoom (Scroll)
        if input.scroll_delta != 0.0 {
            let scroll_sensitivity = 1.0;
            self.distance -= input.scroll_delta * scroll_sensitivity;
            self.distance = self.distance.clamp(1.0, 50.0);
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
