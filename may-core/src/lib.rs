#![warn(missing_docs)]

//! Core library for Maycoon => See `maycoon` crate.
//!
//! Contains core app logic and widget types.

#[cfg(feature = "vg")]
pub use vello as vg;

/// Contains useful types for interacting with winit.
pub mod window {
    pub use winit::event::*;
    pub use winit::keyboard::*;
}

/// Contains app functionality.
pub mod app;

/// Contains the [MayConfig](config::MayConfig) struct.
pub mod config;

/// Contains useful types and functions for layout interaction.
pub mod layout;

/// Contains app state management features
pub mod state;

/// Contains the core widget functionalities
pub mod widget;
