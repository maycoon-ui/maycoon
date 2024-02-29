#![warn(missing_docs)]

pub use glutin::config::Api as Gl;

pub mod app;
pub mod error;
pub mod widget;
pub mod config;
pub mod vg {
    pub use femtovg::{Align, Baseline, Color, FillRule, LineCap, LineJoin, Paint, Path, Quad, rgb, Solidity, Transform2D, FontId};
}

pub mod layout {
    pub use taffy::{
        AlignContent,
        AlignItems,
        AlignSelf,
        AvailableSpace,
        Dimension,
        Display,
        FlexDirection,
        FlexWrap,
        GridAutoFlow,
        GridPlacement,
        GridTrackRepetition,
        JustifyContent,
        JustifyItems,
        JustifySelf,
        Layout,
        LengthPercentage,
        LengthPercentageAuto,
        Line,
        MinMax,
        Overflow,
        Point,
        Position,
        Rect,
        Size,
        Style,
    };
}

pub mod math {
    pub use mint::{
        EulerAngles,
        Point2,
        Quaternion,
        Vector2,
    };
}

pub mod window {
    pub use winit::dpi as dpi;
    pub use winit::event_loop::ControlFlow;
    pub use winit::window::{ResizeDirection, WindowLevel, Icon, BadIcon, CursorGrabMode};
}

