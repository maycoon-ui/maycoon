pub mod app;
pub mod config;
pub mod state;
pub mod widget;

pub mod layout {
    pub use taffy::style;
    pub use taffy::Style as LayoutStyle;
}

pub mod render {
    pub use pathfinder_canvas::CanvasRenderingContext2D as CanvasContext;
}

// TODO: make custom Style type for Taffy and euclid compat
// TODO: make custom Layout type for Taffy and euclid compat
