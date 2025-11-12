use crate::vgi::kurbo::{
    Affine, Circle, CircleSegment, CubicBez, Ellipse, QuadBez, Rect, RoundedRect, Stroke, Triangle,
};
use crate::vgi::{FontData, Scene, VectorGraphicsInterface};
use nalgebra::Vector2;
use peniko::{Brush, Color, ImageBrush};
use std::any::Any;
use std::convert::Infallible;
use std::fmt::Debug;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

/// A dummy graphics backend that does nothing.
///
/// Useful for testing.
#[derive(Debug)]
pub struct DummyGraphics;

impl VectorGraphicsInterface for DummyGraphics {
    type Error = Infallible;
    type Scene = DummyScene;
    type Config = ();

    fn new(_: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn init(
        &mut self,
        _window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn render(
        &mut self,
        _window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
        _scene: &Self::Scene,
        _bg_color: Color,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn resize(
        &mut self,
        _window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
        _size: Vector2<u32>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn uninit(
        &mut self,
        _window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn destroy(
        &mut self,
        _window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A dummy scene that does nothing.
///
/// Useful for testing.
pub struct DummyScene;

impl Scene for DummyScene {
    fn new() -> Self
    where
        Self: Sized,
    {
        unimplemented!("This is a dummy scene")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Scene> {
        Box::new(DummyScene)
    }

    fn reset(&mut self) {}

    fn append(&mut self, _other: &dyn Scene, _transform: Option<Affine>) {}

    fn draw_rect(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _rect: &Rect,
    ) {
    }

    fn draw_rounded_rect(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _rect: &RoundedRect,
    ) {
    }

    fn draw_circle(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _circle: &Circle,
    ) {
    }

    fn draw_circle_segment(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _circle_segment: &CircleSegment,
    ) {
    }

    fn draw_ellipse(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _ellipse: &Ellipse,
    ) {
    }

    fn draw_cubic_bezier(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _cubic_bez: &CubicBez,
    ) {
    }

    fn draw_quadratic_bezier(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _quad_bez: &QuadBez,
    ) {
    }

    fn draw_triangle(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _stroke: Option<&Stroke>,
        _triangle: &Triangle,
    ) {
    }

    fn draw_image(
        &mut self,
        _img: &ImageBrush,
        _transform: Option<Affine>,
        _position: Vector2<f32>,
    ) {
    }

    fn draw_text(
        &mut self,
        _brush: &Brush,
        _transform: Option<Affine>,
        _position: Vector2<f32>,
        _text: &str,
        _hinting: bool,
        _font: &FontData,
        _size: f32,
        _line_gap: f32,
        _max_width: f32,
    ) {
    }

    #[cfg(feature = "svg")]
    fn draw_svg(&mut self, _svg: &usvg::Tree, _transform: Option<Affine>) {}
}
