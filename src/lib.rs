pub use may_core as core;
pub use may_theme as theme;
pub use may_widgets as widgets;

pub mod vg {
    pub use femtovg::{
        rgb, Align, Baseline, Color, FillRule, FontId, LineCap, LineJoin, Paint, Path, Quad,
        Solidity, Transform2D,
    };
}

pub mod math {
    pub use mint::{EulerAngles, Point2, Quaternion, Vector2};
}
