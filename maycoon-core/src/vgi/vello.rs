use crate::vgi::FontData;
use crate::vgi::kurbo::Stroke;
use crate::{tasks, vgi};
use nalgebra::Vector2;
use peniko::kurbo::{CubicBez, Triangle};
use peniko::{Brush, Color, Fill, ImageBrush};
use skrifa::instance::Size;
use skrifa::setting::VariationSetting;
use skrifa::{FontRef, MetadataProvider};
use std::any::Any;
use std::fmt::Debug;
use std::num::NonZeroUsize;
use std::sync::Arc;
pub use vello::AaConfig as Antialiasing;
use vello::kurbo::{
    Affine, Circle, CircleSegment, Ellipse, QuadBez, Rect, RoundedRect, Shape, Vec2,
};
use vello::util::{DeviceHandle, RenderContext, RenderSurface};
use vello::{AaSupport, Error, Glyph, RenderParams, Renderer, RendererOptions};
pub use wgpu_types::PresentMode;
use wgpu_types::{CommandEncoderDescriptor, TextureViewDescriptor};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

/// A vector graphics interface using [vello] as the backend.
///
/// This is the recommended graphics backend.
///
/// Requires the `vello-vg` feature (enabled by default).
///
/// Uses [vello] and [wgpu](https://crates.io/crates/wgpu) to render vector graphics.
pub struct VectorGraphicsInterface<'a> {
    config: VectorGraphicsConfig,
    context: RenderContext,
    renderer: Option<Renderer>,
    surface: Option<RenderSurface<'a>>,
    device: usize,
}

impl<'a> vgi::VectorGraphicsInterface<'a> for VectorGraphicsInterface<'a> {
    type Error = Error;
    type Scene = Scene;
    type Config = VectorGraphicsConfig;

    fn new(config: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            config,
            context: RenderContext::new(),
            renderer: None,
            surface: None,
            device: 0,
        })
    }

    fn init(&mut self, window: Arc<Window>, _: &ActiveEventLoop) -> Result<(), Self::Error> {
        let size = window.inner_size();

        self.surface = Some(tasks::block_on(self.context.create_surface(
            window,
            size.width,
            size.height,
            self.config.present_mode,
        ))?);

        self.device = (self.config.device_selector)(&self.context.devices);
        let device = &self.context.devices[self.device];

        self.renderer = Some(Renderer::new(
            &device.device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: match self.config.antialiasing {
                    Antialiasing::Area => AaSupport::area_only(),
                    Antialiasing::Msaa8 => AaSupport {
                        area: false,
                        msaa8: true,
                        msaa16: false,
                    },
                    Antialiasing::Msaa16 => AaSupport {
                        area: false,
                        msaa8: false,
                        msaa16: true,
                    },
                },
                num_init_threads: Some(self.config.init_threads),
                pipeline_cache: None,
            },
        )?);

        Ok(())
    }

    fn render(
        &mut self,
        window: Arc<Window>,
        _: &ActiveEventLoop,
        scene: &Self::Scene,
        bg_color: Color,
    ) -> Result<(), Self::Error> {
        let renderer = self
            .renderer
            .as_mut()
            .expect("Vector graphics not initialized yet");
        let surface = self
            .surface
            .as_ref()
            .expect("Vector graphics not initialized yet");
        let device_handle = &self.context.devices[self.device];

        renderer.render_to_texture(
            &device_handle.device,
            &device_handle.queue,
            &scene.scene,
            &surface.target_view,
            &RenderParams {
                base_color: bg_color,
                width: window.inner_size().width,
                height: window.inner_size().height,
                antialiasing_method: self.config.antialiasing,
            },
        )?;

        let surface_texture = surface
            .surface
            .get_current_texture()
            .expect("Failed to get current surface texture");

        let mut encoder = device_handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Surface Blit Encoder"),
            });

        surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &surface.target_view,
            &surface_texture
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );

        device_handle.queue.submit([encoder.finish()]);

        window.pre_present_notify();

        surface_texture.present();

        Ok(())
    }

    fn resize(
        &mut self,
        _: Arc<Window>,
        _: &ActiveEventLoop,
        size: Vector2<u32>,
    ) -> Result<(), Self::Error> {
        self.context.resize_surface(
            self.surface
                .as_mut()
                .expect("Vector graphics not initialized yet"),
            size.x,
            size.y,
        );

        Ok(())
    }

    fn uninit(&mut self, _: Arc<Window>, _: &ActiveEventLoop) -> Result<(), Self::Error> {
        self.renderer = None;
        self.surface = None;

        Ok(())
    }

    fn destroy(&mut self, _: Arc<Window>, _: &ActiveEventLoop) -> Result<(), Self::Error> {
        for device in &self.context.devices {
            device.device.destroy();
        }

        Ok(())
    }
}

impl Debug for VectorGraphicsInterface<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VectorGraphicsInterface")
            .field("config", &self.config)
            .field("context", &"<hidden>")
            .field("renderer", &"<hidden>")
            .field("surface", &self.surface)
            .field("device", &self.device)
            .finish()
    }
}

/// A configuration struct for initializing the [VectorGraphicsInterface].
///
/// This struct is passed to [VectorGraphicsInterface::new].
#[derive(Debug, Copy, Clone)]
pub struct VectorGraphicsConfig {
    /// The present mode to use for the surface. Defaults to [PresentMode::AutoNoVsync].
    pub present_mode: PresentMode,
    /// A function to select a device. All devices that are given to this function are compatible.
    ///
    /// Default is to panic if the list is empty or select the first device.
    pub device_selector: fn(&Vec<DeviceHandle>) -> usize,
    /// Use the CPU for some rendering work.
    ///
    /// **NOTE**: This does not disable GPU-usage. Rasterization will still be done by the GPU:
    pub use_cpu: bool,
    /// The antialiasing configuration for the renderer. Defaults to [Antialiasing::Area].
    pub antialiasing: Antialiasing,
    /// The threads to use for renderer initialization. Defaults to 50% of [std::thread::available_parallelism] or 1 if it fails.
    pub init_threads: NonZeroUsize,
}

impl Default for VectorGraphicsConfig {
    fn default() -> Self {
        Self {
            present_mode: PresentMode::AutoNoVsync,
            device_selector: |dev| {
                if dev.is_empty() {
                    panic!("No compatible devices found.")
                } else {
                    0
                }
            },
            use_cpu: false,
            antialiasing: Antialiasing::Area,
            init_threads: NonZeroUsize::new(
                std::thread::available_parallelism()
                    .map(|p| ((p.get() as f32 * 0.5).round() as usize).clamp(1, usize::MAX))
                    .unwrap_or(1),
            )
            .unwrap(),
        }
    }
}

/// A scene to draw on. Uses [vello::Scene] under the hood.
#[derive(Clone, Default)]
pub struct Scene {
    scene: vello::Scene,
}

impl Scene {
    fn draw(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        shape: &impl Shape,
    ) {
        let transform = transform.unwrap_or_default();

        if let Some(stroke) = stroke {
            self.scene.stroke(stroke, transform, brush, None, shape);
        } else {
            self.scene
                .fill(Fill::NonZero, transform, brush, None, shape);
        }
    }
}

impl vgi::Scene for Scene {
    fn new() -> Self {
        Self {
            scene: vello::Scene::new(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn vgi::Scene> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.scene.reset();
    }

    fn append(&mut self, other: &dyn vgi::Scene, transform: Option<Affine>) {
        let any = other.as_any();
        let scene = any.downcast_ref::<Scene>().unwrap();

        self.scene.append(&scene.scene, transform)
    }

    fn draw_rect(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        rect: &Rect,
    ) {
        self.draw(brush, transform, stroke, rect)
    }

    fn draw_rounded_rect(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        rect: &RoundedRect,
    ) {
        self.draw(brush, transform, stroke, rect)
    }

    fn draw_circle(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        circle: &Circle,
    ) {
        self.draw(brush, transform, stroke, circle)
    }

    fn draw_circle_segment(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        circle_segment: &CircleSegment,
    ) {
        self.draw(brush, transform, stroke, circle_segment)
    }

    fn draw_ellipse(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        ellipse: &Ellipse,
    ) {
        self.draw(brush, transform, stroke, ellipse)
    }

    fn draw_cubic_bezier(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        cubic_bez: &CubicBez,
    ) {
        self.draw(brush, transform, stroke, cubic_bez)
    }

    fn draw_quadratic_bezier(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        quad_bez: &QuadBez,
    ) {
        self.draw(brush, transform, stroke, quad_bez)
    }

    fn draw_triangle(
        &mut self,
        brush: &Brush,
        transform: Option<Affine>,
        stroke: Option<&Stroke>,
        triangle: &Triangle,
    ) {
        self.draw(brush, transform, stroke, triangle)
    }

    fn draw_image(&mut self, img: &ImageBrush, transform: Option<Affine>, position: Vector2<f32>) {
        let transform = transform
            .unwrap_or_default()
            .with_translation(Vec2::new(position.x as f64, position.y as f64));

        self.scene.draw_image(img, transform);
    }

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
    ) {
        let font_ref = FontRef::new(font.data.as_ref()).expect("Failed to load font ref");

        let location = font_ref.axes().location::<&[VariationSetting; 0]>(&[]);

        let metrics = font_ref.metrics(Size::new(size), &location);

        let glyph_metrics = font_ref.glyph_metrics(Size::new(size), &location);

        let line_height = metrics.ascent + metrics.descent + metrics.leading;

        let charmap = font_ref.charmap();

        let mut pen_x = position.x;

        let mut pen_y = position.y + size;

        self.scene
            .draw_glyphs(font)
            .font_size(size)
            .transform(transform.unwrap_or_default())
            .brush(brush)
            .normalized_coords(bytemuck::cast_slice(location.coords()))
            .hint(hinting)
            .draw(
                &peniko::Style::Fill(Fill::NonZero),
                text.chars().filter_map(|c| {
                    if c == '\n' {
                        pen_y += line_height + line_gap;
                        pen_x = position.x;
                        return None;
                    }

                    let gid = charmap.map(c).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    let x = pen_x;

                    pen_x += advance;

                    Some(Glyph {
                        id: gid.to_u32(),
                        x,
                        y: pen_y,
                    })
                }),
            );
    }

    #[cfg(feature = "svg")]
    fn draw_svg(&mut self, svg: &usvg::Tree, affine: Option<Affine>) {
        let mut scene = vello::Scene::new();

        vello_svg::append_tree(&mut scene, svg);

        self.scene.append(&scene, affine);
    }
}
