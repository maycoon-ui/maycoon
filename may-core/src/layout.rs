pub use taffy::style_helpers::*;
pub use taffy::Layout;
pub use taffy::Point;
pub use taffy::Size;
pub use taffy::Style as LayoutStyle;

/// The computed layout with children nodes.
pub struct LayoutNode {
    /// The computed layout of this node.
    pub layout: Layout,
    /// The children of this node.
    pub children: Vec<LayoutNode>,
}

/// The raw layout styles with children nodes.
pub struct StyleNode {
    /// The layout style of this node.
    pub style: LayoutStyle,
    /// The children of this node.
    pub children: Vec<StyleNode>,
}
