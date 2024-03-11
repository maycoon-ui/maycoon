use std::fmt::Debug;

use femtovg::Color;

use crate::scheme::Scheme;

/// Theme trait to define the appearances of UI Widgets.
pub trait Theme: Debug {
    /// The background color of the window.
    fn window_background(&self) -> Color;

    /// Returns the scheme of the given widget.
    ///
    /// If the widget is unknown to the theme, this should return [None].
    fn scheme_of(&self, id: String) -> Option<Scheme>;

    /// The default/fallback scheme for
    fn default_scheme_of(&self, widget_type: WidgetType) -> Scheme;
}

/// Type/Category of a widget.
#[derive(Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Default)]
pub enum WidgetType {
    /// Other widget types that are not listed (yet).
    #[default]
    Other,

    /// A Widget that can be interacted with.
    Interactive,

    /// A widget that contains content like text, images and other information.
    Content,

    /// A disabled widget like a disabled button.
    Disabled,

    /// A widget container for other widgets to be put in.
    Container,

    /// Like a container, but collapsed to a single widget like a dropdown or menu.
    Collapsed,
}
