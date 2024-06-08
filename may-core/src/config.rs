use nalgebra::{Point2, Vector2};
pub use vello::AaConfig;
pub use wgpu_types::PresentMode;
pub use winit::platform::windows::CornerPreference;
pub use winit::window::{
    BadIcon, Cursor, CursorIcon, CustomCursor, Icon as WindowIcon, WindowButtons, WindowLevel,
};

use may_theme::theme::Theme;

/// Maycoon Application Configuration Structure.
#[derive(Clone)]
pub struct MayConfig<T: Theme> {
    /// Window Configuration
    pub window: WindowConfig,
    /// Renderer Configuration.
    pub render: RenderConfig,
    /// Theme of the Application.
    pub theme: T,
}

/// Window configuration.
#[derive(Clone)]
pub struct WindowConfig {
    /// The title of the window.
    pub title: String,
    /// The inner size of the window.
    pub size: Vector2<f64>,
    /// The minimum size of the window.
    pub min_size: Option<Vector2<f64>>,
    /// The maximum size of the window.
    pub max_size: Option<Vector2<f64>>,
    /// If the window should be resizeable.
    pub resizable: bool,
    /// If the window should be maximized on startup.
    pub maximized: bool,
    /// The window mode.
    pub mode: WindowMode,
    /// The window level.
    pub level: WindowLevel,
    /// If the window should be visible on startup.
    pub visible: bool,
    /// If the window background should be blurred.
    pub blur: bool,
    /// If the window background should be transparent. May not be compatible on all system.
    pub transparent: bool,
    /// The desired initial position for the window.
    pub position: Option<Point2<f64>>,
    /// If the window should be active/focused on startup.
    pub active: bool,
    /// The enabled window buttons.
    pub buttons: WindowButtons,
    /// If the window should be decorated (have borders).
    pub decorations: bool,
    /// If the corners should be e.g. rounded. Only supported on Windows 11.
    pub corners: CornerPreference,
    /// The resize increments of the window. Not supported everywhere.
    pub resize_increments: Option<Vector2<f64>>,
    /// Prevents window capturing by some apps (not all though).
    pub content_protected: bool,
    /// The window icon.
    pub icon: Option<WindowIcon>,
    /// The window cursor.
    pub cursor: Cursor,
    /// If the window should exit/close on close request (pressing the close window button).
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

/// Renderer configuration.
#[derive(Clone)]
pub struct RenderConfig {
    /// The antialiasing config
    pub antialiasing: AaConfig,
    /// If the backend should use the CPU for most drawing operations.
    ///
    /// **NOTE:** The GPU is still used during rasterization.
    pub cpu: bool,
    /// The presentation mode of the window/surface.
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

/// The window mode.
#[derive(Clone, Debug, Default)]
pub enum WindowMode {
    /// The default windowed mode.
    #[default]
    Windowed,
    /// Size the window to fill the screen and remove borders. This is more modern, than default Fullscreen.
    Borderless,
    /// Legacy Fullscreen mode.
    Fullscreen,
}
