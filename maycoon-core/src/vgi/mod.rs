use nalgebra::Vector2;
use std::any::Any;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use crate::vgi::kurbo::{
    Affine, Circle, CircleSegment, CubicBez, Ellipse, QuadBez, Rect, RoundedRect, Stroke, Triangle,
};
pub use peniko::*;

#[cfg(feature = "svg")]
pub use usvg as svg;

/// Contains the [vello::VectorGraphicsInterface] which uses [vello] as the backend.
///
/// This is the recommended graphics backend.
///
/// Requires the `vello-vg` feature (enabled by default).
#[cfg(feature = "vello-vg")]
pub mod vello;

/// The default graphics backend for maycoon.
///
/// See [vello::VectorGraphicsInterface] for more.
///
/// Requires the `vello-vg` feature (enabled by default).
#[cfg(feature = "vello-vg")]
pub type DefaultGraphics = vello::VectorGraphicsInterface;

/// A trait describing ways to render vector graphics.
///
/// This is a universal interface for 2D vector graphics rendering.
pub trait VectorGraphicsInterface: Debug + 'static {
    /// The error used by most graphics operations.
    type Error: Error;
    /// A direct interface for drawing vector graphics using this interface.
    type Scene: Scene;
    /// A configuration struct for initializing the interface.
    type Config: Debug + Default + Clone;

    /// Creates a new vector graphics interface using the given configuration.
    ///
    /// Returns an [Err] if the interface could not be created.
    fn new(config: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Initializes the interface.
    ///
    /// This will be called on [winit::event::Event::Resumed] to initialize the graphics interface.
    ///
    /// Returns an [Err] if the interface could not be initialized.
    fn init(
        &mut self,
        window: Arc<Window>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), Self::Error>;

    /// Renders the scene to the window.
    ///
    /// This method should render the scene on a window surface and **present** the surface too.
    ///
    /// Returns an [Err] if the scene could not be rendered.
    fn render(
        &mut self,
        window: Arc<Window>,
        event_loop: &ActiveEventLoop,
        scene: &Self::Scene,
        bg_color: Color,
    ) -> Result<(), Self::Error>;

    /// Resizes the window.
    ///
    /// This method should resize the window surface to the given size.
    ///
    /// **NOTE**: Do not resize the window itself. This is already done automatically.
    ///
    /// Returns an [Err] if the window could not be resized.
    fn resize(
        &mut self,
        window: Arc<Window>,
        event_loop: &ActiveEventLoop,
        size: Vector2<u32>,
    ) -> Result<(), Self::Error>;

    /// Uninitializes the interface.
    ///
    /// This will be called on [winit::event::Event::Suspended] to uninitialize the graphics interface.
    ///
    /// Returns an [Err] if the interface could not be uninitialized.
    fn uninit(
        &mut self,
        window: Arc<Window>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), Self::Error>;

    /// Destroys the interface.
    ///
    /// This will be called on [winit::event::WindowEvent::Destroyed] to destroy the graphics interface.
    ///
    /// Returns an [Err] if the interface could not be destroyed.
    fn destroy(
        &mut self,
        window: Arc<Window>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), Self::Error>;
}

/// An interface for drawing vector graphics onto a canvas.
///
/// Must be provided by the [VectorGraphicsInterface].
pub trait Scene: 'static {
    /// Creates a new empty scene.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns this [Scene] as an [Any] reference.
    fn as_any(&self) -> &dyn Any;

    /// Returns this [Scene] as a mutable [Any] reference.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Clones this [Scene] and returns it boxed.
    ///
    /// Used to keep object safety while allowing the scene to be cloned.
    fn dyn_clone(&self) -> Box<dyn Scene>;

    /// Resets the [Scene] to its initial state.
    ///
    /// This scene should be equal to [Scene::new] after this call.
    fn reset(&mut self);

    /// Appends the given [Scene] to this [Scene].
    ///
    /// Apply an optional transform to the scene.
    fn append(&mut self, other: &dyn Scene, transform: Option<Affine>);

    /// Draws a rectangle onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the rectangle.
    ///
    /// Optionally, strokes and does not fill the rectangle.
    fn draw_rect(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        rect: &Rect,
    );

    /// Draws a rounded rectangle onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the rectangle.
    ///
    /// Optionally, strokes and does not fill the rectangle.
    fn draw_rounded_rect(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        rect: &RoundedRect,
    );

    /// Draws a circle onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the circle.
    ///
    /// Optionally, strokes and does not fill the circle.
    fn draw_circle(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        circle: &Circle,
    );

    /// Draws a circle segment onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the circle segment.
    ///
    /// Optionally, strokes and does not fill the circle segment.
    fn draw_circle_segment(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        circle_segment: &CircleSegment,
    );

    /// Draws an ellipse onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the ellipse.
    ///
    /// Optionally, strokes and does not fill the ellipse.
    fn draw_ellipse(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        ellipse: &Ellipse,
    );

    /// Draws a cubic bezier onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the cubic bezier.
    ///
    /// Optionally, strokes and does not fill the cubic bezier.
    fn draw_cubic_bezier(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        cubic_bez: &CubicBez,
    );

    /// Draws a quadratic bezier onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the quadratic bezier.
    ///
    /// Optionally, strokes and does not fill the quadratic bezier.
    fn draw_quadratic_bezier(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        quad_bez: &QuadBez,
    );

    /// Draws a triangle onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the triangle.
    ///
    /// Optionally, strokes and does not fill the triangle.
    fn draw_triangle(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        triangle: &Triangle,
    );

    /// Draws an image onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the image (after the position is inserted).
    fn draw_image(&mut self, img: &ImageBrush, transform: Option<Affine>, position: Vector2<f32>);

    /// Draws text onto the [Scene] with the given brush.
    ///
    /// Apply an optional transform to the text (after the position is inserted).
    ///
    /// You can also specify hinting, size, and the line gap.
    ///
    /// Furthermore, you can choose if the text should be wrapped by passing `max_width`.
    fn draw_text(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        position: Vector2<f32>,
        text: &str,
        hinting: bool,
        font: &FontData,
        size: f32,
        line_gap: f32,
        max_width: f32,
    );

    // TODO: add `Affine` transform arg as soon as vello_svg supports it
    /// Draws an SVG onto the [Scene] with an optional transform.
    ///
    /// Only enabled if the [svg] feature is enabled.
    #[cfg(feature = "svg")]
    fn draw_svg(&mut self, svg: &usvg::Tree, transform: Option<Affine>);
}
