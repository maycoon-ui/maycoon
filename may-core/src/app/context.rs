use winit::window::Window;

use crate::app::info::AppInfo;

pub struct Context<'a> {
    pub window: &'a Window,
    pub app_info: &'a AppInfo,
}
