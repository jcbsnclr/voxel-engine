use std::collections::HashMap;

use winit::event::*;

pub struct InputManager {
    key_state: HashMap<VirtualKeyCode, bool>,
    mouse_delta: (f64, f64)
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            key_state: HashMap::new(),
            mouse_delta: (0.0, 0.0)
        }
    }

    pub fn process_keyboard(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input: KeyboardInput { 
                state, virtual_keycode, .. 
            }, .. } => if let Some(key) = virtual_keycode {
                self.key_state.insert(key.clone(), state == &ElementState::Pressed);
            },

            _ => ()
        }
    }

    pub fn process_mouse(&mut self, delta: (f64, f64)) {
        self.mouse_delta = delta;
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.key_state.get(&key)
            .cloned().unwrap_or(false)
    }

    pub fn delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    pub fn dx(&self) -> f64 { self.mouse_delta.0 }
    pub fn dy(&self) -> f64 { self.mouse_delta.1 }
}