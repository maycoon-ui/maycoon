use crate::icon::{ImageRendering, ShapeRendering, SvgError, TextRendering};
use maycoon_core::vg::Scene;
use vello_svg::usvg;
use vello_svg::usvg::Options;

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
