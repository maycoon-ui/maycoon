use ui_math::point::Point;
use winit::event::{ElementState, KeyEvent, Modifiers, MouseButton};

pub struct AppInfo {
    pub cursor_pos: Option<Point>,
    pub keys: Vec<KeyEvent>,
    pub buttons: Vec<(ElementState, MouseButton)>,
    pub modifiers: Modifiers,
}

impl AppInfo {
    pub fn reset(&mut self) {
        self.keys.clear();
        self.buttons.clear();
    }
}
