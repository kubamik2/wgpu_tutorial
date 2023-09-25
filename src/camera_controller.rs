use winit::event::{WindowEvent, KeyboardInput, VirtualKeyCode};

pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_space_pressed: bool,
    pub is_shift_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed, is_forward_pressed: false, is_backward_pressed: false, is_left_pressed: false, is_right_pressed: false, is_space_pressed: false, is_shift_pressed: false }
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
                    VirtualKeyCode::Space => self.is_space_pressed = is_pressed,
                    VirtualKeyCode::LShift => self.is_shift_pressed = is_pressed,
                    _ => ()
                }
            },
            _ => ()
        }
    }

    pub fn update_camera(&self, camera: &mut crate::Camera) {
        if self.is_forward_pressed {
            camera.position += camera.direction * self.speed;
        }

        if self.is_backward_pressed {
            camera.position -= camera.direction * self.speed;
        }

        if self.is_right_pressed {
            camera.yaw.0 += 0.001;
        }

        if self.is_left_pressed {
            camera.yaw.0 -= 0.001;
        }

        if self.is_space_pressed {
            camera.position.y += 0.004;
        }

        if self.is_shift_pressed {
            camera.position.y -= 0.004;
        }
    }
}