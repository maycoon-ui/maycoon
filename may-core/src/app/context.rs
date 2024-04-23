use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::ExternalError;
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::monitor::MonitorHandle;
use winit::platform::windows::WindowExtWindows;
use winit::window::{CursorGrabMode, ResizeDirection, Window};

use crate::config::Fullscreen;

pub struct AppContext<'a, P: 'static> {
    pub window: &'a Window,
    pub canvas: &'a Canvas<OpenGl>,
    pub elwt: &'a EventLoopWindowTarget<P>,
}
