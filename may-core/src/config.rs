use euclid::default::Size2D;
use euclid::Point2D;
use may_theme::theme::Theme;
use winit::window::WindowLevel;

#[derive(Clone, Debug)]
pub struct MayConfig<T: Theme> {
    pub window: WindowConfig,
    pub render: RenderConfig,
    pub theme: T,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RenderConfig {
    pub preference: DevicePreference,
    pub mode: RenderLevel,
    pub max_frame_latency: u128,
    pub parallel: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            preference: DevicePreference::default(),
            mode: RenderLevel::default(),
            max_frame_latency: 16,
            parallel: true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RenderLevel {
    Auto,
    Primary,
    Secondary,
}

impl Default for RenderLevel {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DevicePreference {
    LowPower,
    HighPerformance,
    Software,
}

impl Default for DevicePreference {
    fn default() -> Self {
        Self::HighPerformance
    }
}

#[derive(Clone, Debug)]
pub struct WindowConfig {
    pub size: Size2D<u32>,
    pub title: String,
    pub decorations: bool,
    pub resizable: bool,
    pub transparent: bool,
    pub maximized: bool,
    pub fullscreen: Option<Fullscreen>,
    pub position: Point2D<f64, f64>,
    pub level: winit::window::WindowLevel,
    pub blur: bool,
    pub visible: bool,
    pub active: bool,
    pub skip_taskbar: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            size: Size2D::new(1000, 600),
            title: "My Awesome App".to_string(),
            decorations: true,
            resizable: true,
            transparent: false,
            maximized: false,
            fullscreen: None,
            position: Point2D::default(),
            level: WindowLevel::Normal,
            blur: false,
            visible: true,
            active: true,
            skip_taskbar: false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Fullscreen {
    Exclusive,
    Borderless,
}

impl Default for Fullscreen {
    fn default() -> Self {
        Self::Exclusive
    }
}
