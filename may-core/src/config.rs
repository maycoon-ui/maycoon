use nalgebra::{Point2, Vector2};
pub use vello::AaConfig;
pub use wgpu_types::PresentMode;
pub use winit::platform::windows::CornerPreference;
pub use winit::window::{
    BadIcon, Cursor, CursorIcon, CustomCursor, Icon as WindowIcon, WindowButtons, WindowLevel,
};

use may_theme::theme::Theme;

#[derive(Clone)]
pub struct MayConfig<T: Theme> {
    pub window: WindowConfig,
    pub render: RenderConfig,
    pub theme: T,
}

#[derive(Clone)]
pub struct WindowConfig {
    pub title: String,
    pub size: Vector2<f64>,
    pub min_size: Option<Vector2<f64>>,
    pub max_size: Option<Vector2<f64>>,
    pub resizable: bool,
    pub maximized: bool,
    pub mode: WindowMode,
    pub level: WindowLevel,
    pub visible: bool,
    pub blur: bool,
    pub transparent: bool,
    pub position: Option<Point2<f64>>,
    pub active: bool,
    pub buttons: WindowButtons,
    pub decorations: bool,
    pub corners: CornerPreference,
    pub resize_increments: Option<Vector2<f64>>,
    pub content_protected: bool,
    pub icon: Option<WindowIcon>,
    pub cursor: Cursor,
    pub close_on_request: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "New App".to_string(),
            size: Vector2::new(800.0, 600.0),
            min_size: None,
            max_size: None,
            resizable: true,
            maximized: false,
            mode: WindowMode::default(),
            level: Default::default(),
            visible: true,
            blur: false,
            transparent: false,
            position: None,
            active: true,
            buttons: WindowButtons::all(),
            decorations: true,
            corners: Default::default(),
            resize_increments: None,
            content_protected: false,
            icon: None,
            cursor: Cursor::default(),
            close_on_request: true,
        }
    }
}

#[derive(Clone)]
pub struct RenderConfig {
    pub antialiasing: AaConfig,
    pub cpu: bool,
    pub present_mode: PresentMode,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            antialiasing: AaConfig::Area,
            cpu: false,
            present_mode: PresentMode::AutoNoVsync,
        }
    }
}

#[derive(Clone, Debug)]
pub enum WindowMode {
    Windowed,
    Borderless,
    Fullscreen,
}

impl Default for WindowMode {
    fn default() -> Self {
        Self::Windowed
    }
}
