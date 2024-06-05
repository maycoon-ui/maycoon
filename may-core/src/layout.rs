pub use taffy::*;

pub struct LayoutNode {
    pub layout: Layout,
    pub children: Vec<LayoutNode>,
}

pub struct StyleNode {
    pub style: Style,
    pub children: Vec<StyleNode>,
}
