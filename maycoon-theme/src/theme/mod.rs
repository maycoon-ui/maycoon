use peniko::Color;

use crate::globals::Globals;
use crate::id::WidgetId;
use crate::style::{DefaultStyles, Style};

/// The Celeste Theme.
pub mod celeste;

/// Base trait for all themes.
pub trait Theme {
    /// Return the [`Style`] of the given widget using its ID.
    /// Returns [`None`] if the theme does not have styles for the given widget.
    /// In that case, you should use [`Theme::defaults`] to get widget style defaults.
    fn of(&self, id: WidgetId) -> Option<Style>;
    /// Get the default widget styles.
    fn defaults(&self) -> DefaultStyles;
    /// Get the background color of the window.
    fn window_background(&self) -> Color;
    /// Get global style values.
    fn globals(&self) -> &Globals;
    /// Get mutable global style values.
    fn globals_mut(&mut self) -> &mut Globals;
}
