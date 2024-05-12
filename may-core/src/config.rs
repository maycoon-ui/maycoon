use ui_math::point::Point;
use ui_math::size::Size;
use winit::window::{Icon, WindowLevel};

use may_theme::theme::Theme;
use may_theme::themes::MayTheme;

use crate::Gl;

#[derive(Debug)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub graphics: GraphicsConfig,
    pub theme: Box<dyn Theme>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            graphics: GraphicsConfig::default(),
            theme: Box::new(MayTheme::Light),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub position: Point,
    pub resizable: bool,
    pub fullscreen: Option<Fullscreen>,
    pub decorations: bool,
    pub maximized: bool,
    pub transparent: bool,
    pub any_thread: bool,
    pub level: WindowLevel,
    pub blur: bool,
    pub visible: bool,
    pub active: bool,
    pub skip_taskbar: bool,
    pub close_on_request: bool,
    pub window_icon: Option<Icon>,
    pub taskbar_icon: Option<Icon>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "My Maycoon App".to_string(),
            size: Size::new(1000.0, 600.0),
            min_size: None,
            max_size: None,
            position: Point::new(0.0, 0.0),
            resizable: true,
            fullscreen: None,
            decorations: true,
            maximized: false,
            transparent: false,
            any_thread: false,
            level: WindowLevel::Normal,
            blur: false,
            visible: true,
            active: true,
            skip_taskbar: false,
            close_on_request: true,
            window_icon: None,
            taskbar_icon: None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Default)]
pub enum Fullscreen {
    Exclusive,
    #[default]
    Borderless,
}

#[derive(Debug)]
pub struct GraphicsConfig {
    pub gl: Gl,
    pub multisampling: u8,
    pub hardware_acceleration: Option<bool>,
    pub force_antialiasing: bool,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            gl: Gl::OPENGL,
            multisampling: 1,
            hardware_acceleration: None,
            force_antialiasing: true,
        }
    }
}
