use winit::event::{WindowEvent, KeyboardInput, VirtualKeyCode};

pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed, is_forward_pressed: false, is_backward_pressed: false, is_left_pressed: false, is_right_pressed: false }
    }

    pub fn process_events(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == winit::event::ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => self.is_forward_pressed = is_pressed,
                    VirtualKeyCode::S => self.is_backward_pressed = is_pressed,
                    VirtualKeyCode::A => self.is_left_pressed = is_pressed,
                    VirtualKeyCode::D => self.is_right_pressed = is_pressed,
                    _ => ()
                }
            },
            _ => ()
        }
    }

    pub fn update_camera(&self, camera: &mut crate::Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }

        if self.is_backward_pressed && forward_mag > self.speed {
            camera.eye -= forward_norm * self.speed;
        }
        
        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }

        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }

    }
}