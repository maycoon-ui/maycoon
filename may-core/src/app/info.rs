use nalgebra::Vector2;
use winit::event::{DeviceId, ElementState, KeyEvent, MouseButton};

pub struct AppInfo {
    pub cursor_pos: Option<Vector2<f64>>,
    pub keys: Vec<(DeviceId, KeyEvent)>,
    pub buttons: Vec<(DeviceId, MouseButton, ElementState)>,
}

impl AppInfo {
    pub fn reset(&mut self) {
        self.buttons.clear();
        self.keys.clear();
    }
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            cursor_pos: None,
            keys: Vec::with_capacity(4),
            buttons: Vec::with_capacity(2),
        }
    }
}
