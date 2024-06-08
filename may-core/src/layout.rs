pub use taffy::style_helpers::*;
pub use taffy::Layout;
pub use taffy::Point;
pub use taffy::Size;
pub use taffy::Style as LayoutStyle;

pub struct LayoutNode {
    pub layout: Layout,
    pub children: Vec<LayoutNode>,
}

pub struct StyleNode {
    pub style: LayoutStyle,
    pub children: Vec<StyleNode>,
}
