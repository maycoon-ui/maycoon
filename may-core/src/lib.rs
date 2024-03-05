#![warn(missing_docs)]

pub use glutin::config::Api as Gl;

pub mod app;
pub mod config;
pub mod error;
pub mod widget;

pub mod layout {
    pub use taffy::{
        AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, FlexDirection,
        FlexWrap, GridAutoFlow, GridPlacement, GridTrackRepetition, JustifyContent, JustifyItems,
        JustifySelf, Layout, LengthPercentage, LengthPercentageAuto, Line, MinMax, Overflow, Point,
        Position, Rect, Size, Style,
    };
}

pub mod window {
    pub use winit::dpi;
    pub use winit::event::{
        DeviceId, ElementState, KeyEvent, Modifiers, MouseButton, MouseScrollDelta,
    };
    pub use winit::event_loop::ControlFlow;
    pub use winit::keyboard;
    pub use winit::window::{BadIcon, CursorGrabMode, Icon, ResizeDirection, WindowLevel};
}
