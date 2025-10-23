use maycoon_core::app::info::AppInfo;
use maycoon_core::app::update::Update;
use maycoon_core::layout::{Dimension, LayoutNode, LayoutStyle, StyleNode};
use maycoon_core::widget::{Widget, WidgetLayoutExt};
use maycoon_theme::id::WidgetId;
use maycoon_theme::theme::Theme;
use nalgebra::Vector2;

use crate::icon::svg::SvgIcon;
use maycoon_core::app::context::AppContext;
use maycoon_core::signal::MaybeSignal;
use maycoon_core::vgi::Scene;
use maycoon_core::vgi::kurbo::{Affine, Vec2};

/// Contains the [SvgIcon] struct for representing a rendered SVG Icon.
pub mod svg;

/// A simple icon widget to display SVG icons using [vello_svg] and [usvg].
///
/// ### Theming
/// The widget itself only draws the underlying icon, so theming is useless.
pub struct Icon {
    layout_style: MaybeSignal<LayoutStyle>,
    icon: MaybeSignal<SvgIcon>,
}

impl Icon {
    /// Creates a new icon widget from the given svg icon.
    pub fn new(icon: impl Into<MaybeSignal<SvgIcon>>) -> Self {
        Self {
            layout_style: LayoutStyle {
                size: Vector2::new(Dimension::length(8.0), Dimension::length(8.0)),
                ..Default::default()
            }
            .into(),
            icon: icon.into(),
        }
    }
}

impl Widget for Icon {
    fn render(
        &mut self,
        scene: &mut dyn Scene,
        _: &mut dyn Theme,
        layout_node: &LayoutNode,
        _: &AppInfo,
        _: AppContext,
    ) {
        let icon = self.icon.get();

        // The size is divided, as otherwise the icon would be either too large (with 1.0) or too tiny (with 0.1 somehow getting converted to 0.0)
        let affine = Affine::scale_non_uniform(
            layout_node.layout.size.width as f64 / 100.0,
            layout_node.layout.size.height as f64 / 100.0,
        )
        .then_translate(Vec2::new(
            layout_node.layout.location.x as f64,
            layout_node.layout.location.y as f64,
        ));

        scene.draw_svg(icon.tree(), Some(affine));
    }

    fn layout_style(&self) -> StyleNode {
        StyleNode {
            style: self.layout_style.get().clone(),
            children: Vec::new(),
        }
    }

    fn update(&mut self, _: &LayoutNode, _: AppContext, _: &AppInfo) -> Update {
        Update::empty()
    }

    fn widget_id(&self) -> WidgetId {
        WidgetId::new("maycoon-widgets", "Icon")
    }
}

impl WidgetLayoutExt for Icon {
    fn set_layout_style(&mut self, layout_style: impl Into<MaybeSignal<LayoutStyle>>) {
        self.layout_style = layout_style.into();
    }
}
