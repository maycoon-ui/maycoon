#![warn(missing_docs)]

pub use glutin::config::Api as Gl;

pub mod app;
pub mod config;
pub mod error;
pub mod gl;
pub mod render;
pub mod state;
pub mod util;
pub mod widget;

pub mod window {
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

pub mod layout {
    pub use taffy::{
        AvailableSpace, Dimension, Display, FlexDirection, FlexWrap, GridAutoFlow, Layout, Line,
        MinMax, Size, SizingMode, Style,
    };
}
