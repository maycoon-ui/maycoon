use nalgebra::Vector2;
pub use taffy::{
    AlignContent, AlignItems, AlignSelf, Dimension, Display, FlexDirection, FlexWrap, GridAutoFlow,
    GridPlacement, JustifyContent, JustifyItems, JustifySelf, Layout, LengthPercentage,
    LengthPercentageAuto, Line, NodeId, Overflow, Position, Rect, TaffyError, TaffyResult,
    TaffyTree,
};

/// Defines different aspects and properties of a widget layout.
#[derive(Clone, PartialEq, Debug)]
pub struct LayoutStyle {
    /// What layout strategy should be used?
    pub display: Display,

    /// How children overflowing their container should affect layout.
    pub overflow: (Overflow, Overflow),

    /// How much space (in points) should be reserved for scrollbars.
    pub scrollbar_width: f32,

    /// What should the position value of this struct use as a base offset?
    pub position: Position,

    /// How should the position of this element be tweaked relative to the layout defined?
    pub inset: Rect<LengthPercentageAuto>,

    /// Sets the initial size of the item.
    pub size: Vector2<Dimension>,

    /// Controls the minimum size of the item.
    pub min_size: Vector2<Dimension>,

    /// Controls the maximum size of the item.
    pub max_size: Vector2<Dimension>,

    /// Sets the preferred aspect ratio for the item
    ///
    /// The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    /// How large should the margin be on each side?
    pub margin: Rect<LengthPercentageAuto>,

    /// How large should the padding be on each side?
    pub padding: Rect<LengthPercentage>,

    /// How large should the border be on each side?
    pub border: Rect<LengthPercentage>,

    /// How this node's children aligned in the cross/block axis?
    pub align_items: Option<AlignItems>,

    /// How this node should be aligned in the cross/block axis
    /// Falls back to the parents [AlignItems] if not set.
    pub align_self: Option<AlignSelf>,

    /// How this node's children should be aligned in the inline axis.
    pub justify_items: Option<AlignItems>,

    /// How this node should be aligned in the inline axis
    /// Falls back to the parents [JustifyItems] if not set.
    pub justify_self: Option<AlignSelf>,

    /// How should content contained within this item be aligned in the cross/block axis?
    pub align_content: Option<AlignContent>,

    /// How should content contained within this item be aligned in the main/inline axis?
    pub justify_content: Option<JustifyContent>,

    /// How large should the gaps between items in a grid or flex container be?
    pub gap: Vector2<LengthPercentage>,

    /// Which direction does the main axis flow in?
    pub flex_direction: FlexDirection,

    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: FlexWrap,

    /// Sets the initial main axis size of the item.
    pub flex_basis: Dimension,

    /// The relative rate at which this item grows when it is expanding to fill space.
    ///
    /// 0.0 is the default value, and this value must be positive.
    pub flex_grow: f32,

    /// The relative rate at which this item shrinks when it is contracting to fit into space.
    ///
    /// 1.0 is the default value, and this value must be positive.
    pub flex_shrink: f32,

    /// Controls how items get placed into the grid for auto-placed items.
    pub grid_auto_flow: GridAutoFlow,

    /// Defines which row in the grid the item should start and end at.
    pub grid_row: Line<GridPlacement>,

    /// Defines which column in the grid the item should start and end at.
    pub grid_column: Line<GridPlacement>,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        LayoutStyle {
            display: Display::default(),
            overflow: (Overflow::Visible, Overflow::Visible),
            scrollbar_width: 0.0,
            position: Position::Relative,
            inset: Rect::auto(),
            margin: Rect::zero(),
            padding: Rect::zero(),
            border: Rect::zero(),
            size: Vector2::new(Dimension::auto(), Dimension::auto()),
            min_size: Vector2::new(Dimension::auto(), Dimension::auto()),
            max_size: Vector2::new(Dimension::auto(), Dimension::auto()),
            aspect_ratio: None,
            gap: Vector2::new(LengthPercentage::length(0.0), LengthPercentage::length(0.0)),
            align_items: None,
            align_self: None,
            justify_items: None,
            justify_self: None,
            align_content: None,
            justify_content: None,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::auto(),
            grid_auto_flow: GridAutoFlow::Row,
            grid_row: Line {
                start: GridPlacement::Auto,
                end: GridPlacement::Auto,
            },
            grid_column: Line {
                start: GridPlacement::Auto,
                end: GridPlacement::Auto,
            },
        }
    }
}

impl From<LayoutStyle> for taffy::Style {
    fn from(value: LayoutStyle) -> Self {
        taffy::Style {
            display: value.display,
            overflow: taffy::Point {
                x: value.overflow.0,
                y: value.overflow.1,
            },
            scrollbar_width: value.scrollbar_width,
            position: value.position,
            inset: value.inset,
            margin: value.margin,
            padding: value.padding,
            border: value.border,
            size: taffy::Size {
                width: value.size.x,
                height: value.size.y,
            },
            min_size: taffy::Size {
                width: value.min_size.x,
                height: value.min_size.y,
            },
            max_size: taffy::Size {
                width: value.max_size.x,
                height: value.max_size.y,
            },
            aspect_ratio: value.aspect_ratio,
            gap: taffy::Size {
                width: value.gap.x,
                height: value.gap.y,
            },
            align_items: value.align_items,
            align_self: value.align_self,
            justify_items: value.justify_items,
            justify_self: value.justify_self,
            align_content: value.align_content,
            justify_content: value.justify_content,
            flex_direction: value.flex_direction,
            flex_wrap: value.flex_wrap,
            flex_grow: value.flex_grow,
            flex_shrink: value.flex_shrink,
            flex_basis: value.flex_basis,
            grid_auto_flow: value.grid_auto_flow,
            grid_row: value.grid_row,
            grid_column: value.grid_column,
            ..Default::default()
        }
    }
}

/// The computed layout with children nodes.
#[derive(Debug)]
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
