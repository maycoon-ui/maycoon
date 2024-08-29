#![warn(missing_docs)]

//! Create beautiful and lightning fast UI Applications with Rust.
//!
//! See [maycoon-ui.github.io/](https://maycoon-ui.github.io/) for more information.

pub use nalgebra as math;
pub use peniko as color;

pub use maycoon_core as core;
pub use maycoon_theme as theme;
pub use maycoon_widgets as widgets;

#[cfg(feature = "macros")]
pub use maycoon_macros as macros;
