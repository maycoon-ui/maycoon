#![warn(missing_docs)]

//! Widget library for Maycoon => See `maycoon` crate.
//!
//! Contains a collection of beautiful maycoon widgets.

/// Contains the [text::Text] widget.
pub mod text;

/// Contains the [button::Button] widget.
pub mod button;

/// Contains the [container::Container] widget.
pub mod container;

/// Contains the [image::Image] widget.
pub mod image;

/// Contains the [checkbox::Checkbox] widget.
pub mod checkbox;

/// Contains the [slider::Slider] widget.
pub mod slider;

/// Contains the [fetcher::WidgetFetcher] widget.
pub mod fetcher;

/// Contains the [canvas::Canvas] widget.
#[cfg(feature = "canvas")]
pub mod canvas;

/// Contains the [gesture_detector::GestureDetector] widget.
pub mod gesture_detector;

/// Contains the [icon::Icon] widget.
#[cfg(feature = "svg")]
pub mod icon;

/// Contains the [animator::Animator] widget and associated structures.
pub mod animator;
