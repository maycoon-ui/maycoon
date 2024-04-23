#![warn(missing_docs)]

pub use glutin::config::Api as Gl;

pub mod app;
pub mod config;
pub mod error;
pub mod gl;
pub mod handler;
pub mod util;

pub mod window {
    pub use winit::dpi;
    pub use winit::event::{
        DeviceId, ElementState, KeyEvent, Modifiers, MouseButton, MouseScrollDelta,
    };
    pub use winit::event_loop::ControlFlow;
    pub use winit::keyboard;
    pub use winit::window::{BadIcon, CursorGrabMode, Icon, ResizeDirection, WindowLevel};
}

pub mod vg {
    pub use femtovg::{
        rgb, Align, Baseline, Color, FillRule, FontId, LineCap, LineJoin, Paint, Path, Quad,
        Solidity, Transform2D,
    };
}
