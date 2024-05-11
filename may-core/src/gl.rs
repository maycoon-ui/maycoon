use std::num::NonZeroU32;

use cfg_if::cfg_if;
use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::config::{ConfigSurfaceTypes, ConfigTemplateBuilder};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::window::Window;

use crate::config::GraphicsConfig;

pub unsafe fn create_gl(
    window: &Window,
    config: &GraphicsConfig,
) -> (
    PossiblyCurrentContext,
    Surface<WindowSurface>,
    Canvas<OpenGl>,
) {
    let display = Display::new(window.raw_display_handle(), get_display_preference(window))
        .expect("Failed to create Gl display");

    let gl_config = display
        .find_configs(
            ConfigTemplateBuilder::new()
                .compatible_with_native_window(window.raw_window_handle())
                .prefer_hardware_accelerated(config.hardware_acceleration)
                .with_api(config.gl)
                .prefer_hardware_accelerated(Some(true))
                .with_surface_type(ConfigSurfaceTypes::WINDOW)
                .with_multisampling(config.multisampling)
                .build(),
        )
        .expect("Failed to find Gl config")
        .next()
        .expect("Failed to get any Gl config");

    let surface = display
        .create_window_surface(
            &gl_config,
            &SurfaceAttributesBuilder::<WindowSurface>::new().build(
                window.raw_window_handle(),
                NonZeroU32::new_unchecked(window.inner_size().width),
                NonZeroU32::new_unchecked(window.inner_size().height),
            ),
        )
        .expect("Failed to create Gl surface");

    let context = display
        .create_context(
            &gl_config,
            &ContextAttributesBuilder::new().build(Some(window.raw_window_handle())),
        )
        .expect("Failed to create Gl context")
        .make_current(&surface)
        .expect("Failed to make Gl context current");

    let canvas = Canvas::new(
        OpenGl::new_from_function_cstr(|cstr| display.get_proc_address(cstr) as *const _)
            .expect("Failed to create OpenGl renderer"),
    )
    .expect("Failed to create OpenGl canvas");

    (context, surface, canvas)
}

pub fn get_display_preference<H: HasRawWindowHandle>(handle: &H) -> DisplayApiPreference {
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            DisplayApiPreference::Cgl
        } else if #[cfg(target_os = "windows")] {
            DisplayApiPreference::EglThenWgl(Some(
                handle.raw_window_handle()
            ))
        } else {
            DisplayApiPreference::Egl
        }
    }
}
