use crate::config::GraphicsConfig;
use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::config::{ConfigSurfaceTypes, ConfigTemplateBuilder};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::num::NonZeroU32;
use winit::window::Window;

pub fn create_gl(
    window: &Window,
    config: &GraphicsConfig,
) -> (
    PossiblyCurrentContext,
    Surface<WindowSurface>,
    Canvas<OpenGl>,
) {
    let display = unsafe {
        let pref = DisplayApiPreference::Egl;

        #[cfg(target_os = "macos")]
        let pref = DisplayApiPreference::Cgl;

        #[cfg(target_os = "windows")]
        let pref = DisplayApiPreference::WglThenEgl(Some(window.raw_window_handle()));

        Display::new(window.raw_display_handle(), pref)
    }
    .expect("Failed to create Gl display");

    let gl_config = unsafe {
        display.find_configs(
            ConfigTemplateBuilder::new()
                .compatible_with_native_window(window.raw_window_handle())
                .prefer_hardware_accelerated(config.hardware_acceleration)
                .with_api(config.gl)
                .prefer_hardware_accelerated(Some(true))
                .with_surface_type(ConfigSurfaceTypes::WINDOW)
                .with_multisampling(config.multisampling)
                .build(),
        )
    }
    .expect("Failed to find Gl config")
    .next()
    .expect("Failed to get any Gl config");

    let surface = unsafe {
        display.create_window_surface(
            &gl_config,
            &SurfaceAttributesBuilder::<WindowSurface>::new().build(
                window.raw_window_handle(),
                NonZeroU32::new_unchecked(window.inner_size().width),
                NonZeroU32::new_unchecked(window.inner_size().height),
            ),
        )
    }
    .expect("Failed to create Gl surface");

    let context = unsafe {
        display.create_context(
            &gl_config,
            &ContextAttributesBuilder::new().build(Some(window.raw_window_handle())),
        )
    }
    .expect("Failed to create Gl context")
    .make_current(&surface)
    .expect("Failed to make Gl context current");

    let canvas = Canvas::new(
        unsafe {
            OpenGl::new_from_function_cstr(|cstr| display.get_proc_address(cstr) as *const _)
        }
        .expect("Failed to create OpenGl renderer"),
    )
    .expect("Failed to create OpenGl canvas");

    (context, surface, canvas)
}
