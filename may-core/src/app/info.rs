use ui_math::point::Point;

pub struct AppInfo {
    pub cursor_pos: Option<Point>,
}

impl AppInfo {
    pub fn reset(&mut self) {}
}
