use crate::ext::WidgetLayoutExt;
use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::state::{State, Val};
use maycoon_core::vg::kurbo::{Affine, Vec2};
use maycoon_core::vg::Scene;
use maycoon_core::widget::Widget;
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;
use vello_svg::usvg;
use vello_svg::usvg::Options;

pub use usvg::ImageRendering;
pub use usvg::ShapeRendering;
pub use usvg::TextRendering;

/// Error type for parsing SVGs with [`usvg`].
pub type SvgError = usvg::Error;

/// A simple icon widget to display SVG icons using [`vello_svg`] and [`usvg`].
///
/// ### Theming
/// The widget itself only draws the underlying icon, so theming is useless.
pub struct Icon<S: State> {
    layout_style: Val<S, LayoutStyle>,
    icon: Val<S, SvgIcon>,
}

impl<S: State> Icon<S> {
    /// Creates a new icon widget from the given svg icon.
    pub fn new(icon: impl Into<Val<S, SvgIcon>>) -> Self {
        Self {
            layout_style: Val::new_val(LayoutStyle {
                size: Vector2::new(Dimension::Length(8.0), Dimension::Length(8.0)),
                ..Default::default()
            }),
            icon: icon.into(),
        }
    }
}

impl<S: State> Widget<S> for Icon<S> {
    fn render(
        &mut self,
        scene: &mut Scene,
        _: &mut dyn Theme,
        _: &AppInfo,
        layout_node: &LayoutNode,
        state: &S,
    ) {
        let icon = self.icon.get_mut(state);

        // The size is divided, as otherwise the icon would be either too large (with 1.0) or too tiny (with 0.1 somehow getting converted to 0.0)
        let affine = Affine::scale_non_uniform(
            layout_node.layout.size.width as f64 / 100.0,
            layout_node.layout.size.height as f64 / 100.0,
        )
        .then_translate(Vec2::new(
            layout_node.layout.location.x as f64,
            layout_node.layout.location.y as f64,
        ));

        scene.append(icon.scene(), Some(affine));
    }

    fn layout_style(&mut self, state: &S) -> StyleNode {
        StyleNode {
            style: self.layout_style.get_ref(state).clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: &mut S, _: &AppInfo) -> Update {
        self.icon.invalidate();

        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Icon")
    }
}

impl<S: State> WidgetLayoutExt<S> for Icon<S> {
    fn set_layout_style(&mut self, layout_style: impl Into<Val<S, LayoutStyle>>) {
        self.layout_style = layout_style.into();
    }
}

/// An SVG icon rendered as a Vello [`Scene`].
pub struct SvgIcon(Scene);

impl SvgIcon {
    /// Creates a new icon from the given SVG source.
    /// Returns [`Ok`] if the SVG could be parsed, [`Err`] otherwise.
    ///
    /// **This calls [`Self::new_custom`] with the following options:**
    /// - [`ShapeRendering::GeometricPrecision`] for precise shape rendering.
    /// - [`TextRendering::OptimizeLegibility`] for good text rendering.
    /// - [`ImageRendering::OptimizeSpeed`] for fast image rendering.
    pub fn new(source: impl AsRef<str>) -> Result<Self, SvgError> {
        Self::new_custom(
            source,
            ShapeRendering::GeometricPrecision,
            TextRendering::OptimizeLegibility,
            ImageRendering::OptimizeSpeed,
        )
    }

    /// Creates a new icon from the given SVG source.
    /// Returns [`Ok`] if the SVG could be parsed, [`Err`] otherwise.
    ///
    /// This method allows customizing the SVG rendering options.
    pub fn new_custom(
        source: impl AsRef<str>,
        shape_rendering: ShapeRendering,
        text_rendering: TextRendering,
        image_rendering: ImageRendering,
    ) -> Result<Self, SvgError> {
        let tree = usvg::Tree::from_str(
            source.as_ref(),
            &Options {
                shape_rendering,
                text_rendering,
                image_rendering,
                ..Default::default()
            },
        )?;

        let scene = vello_svg::render_tree(&tree);

        Ok(Self(scene))
    }

    /// Returns the underlying [`Scene`].
    pub fn scene(&self) -> &Scene {
        &self.0
    }
}

impl From<Scene> for SvgIcon {
    fn from(scene: Scene) -> Self {
        Self(scene)
    }
}
