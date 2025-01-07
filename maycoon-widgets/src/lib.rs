#![warn(missing_docs)]

//! Widget library for Maycoon => See `maycoon` crate.
//!
//! Contains a collection of beautiful maycoon widgets.

/// Contains various traits for extending widget capability.
pub mod ext;

/// Contains the [text::Text] widget.
pub mod text;

/// Contains the [button::Button] widget.
pub mod button;

/// Contains the [container::Container] widget.
pub mod container;

/// Contains the [dummy::DummyWidget] widget.
pub mod dummy;

/// Contains the [image::Image] widget.
pub mod image;

/// Contains the [checkbox::Checkbox] widget.
pub mod checkbox;

/// Contains the [slider::Slider] widget.
pub mod slider;

/// Contains the [canvas::Canvas] widget.
#[cfg(feature = "canvas")]
pub mod canvas;
