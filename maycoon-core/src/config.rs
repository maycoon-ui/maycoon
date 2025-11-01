use nalgebra::{Point2, Vector2};
use std::num::NonZeroUsize;
pub use vello::AaConfig;
pub use winit::window::{
    BadIcon, Cursor, CursorIcon, CustomCursor, Icon as WindowIcon, WindowButtons, WindowLevel,
};

use crate::vgi::VectorGraphicsInterface;
use maycoon_theme::theme::Theme;

/// Maycoon Application Configuration Structure.
#[derive(Clone, Debug)]
pub struct MayConfig<T: Theme, V: VectorGraphicsInterface> {
    /// Window Configuration
    pub window: WindowConfig,
    /// Task Runner Configuration. If [None] (default), the task runner won't be enabled.
    pub tasks: Option<TasksConfig>,
    /// Theme of the Application.
    pub theme: T,
    /// The configuration of the vector graphics interface.
    pub graphics: V::Config,
}

impl<T: Default + Theme, V: VectorGraphicsInterface> Default for MayConfig<T, V> {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            tasks: None,
            theme: T::default(),
            graphics: V::Config::default(),
        }
    }
}

/// Window configuration.
#[derive(Clone, Debug)]
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
            resize_increments: None,
            content_protected: false,
            icon: None,
            cursor: Cursor::default(),
            close_on_request: true,
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

/// Configuration structure for the integrated [TaskRunner](crate::tasks::TaskRunner).
///
/// The task runner isn't used by maycoon internally, but can be used to spawn asynchronous tasks and integrate them with the UI.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TasksConfig {
    /// The stack size of each thread of the task runner thread pool. Defaults to 1 MB.
    pub stack_size: usize,
    /// The amount of worker threads of the task runner thread pool. Defaults to half of the available threads.
    pub workers: NonZeroUsize,
}

impl Default for TasksConfig {
    fn default() -> Self {
        Self {
            stack_size: 1024 * 1024, // 1 MB
            workers: NonZeroUsize::new(
                std::thread::available_parallelism()
                    .expect("Failed to get available threads")
                    .get()
                    / 2,
            )
            .unwrap(),
        }
    }
}
