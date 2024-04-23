use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::config::{ConfigSurfaceTypes, ConfigTemplateBuilder};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext};
use glutin::display::{Display, DisplayApiPreference, GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::num::NonZeroU32;
use std::time::Instant;
use winit::event::{DeviceEvent, Event, Modifiers, StartCause, WindowEvent};
use winit::event_loop::{EventLoopBuilder, EventLoopWindowTarget};
use winit::platform::windows::{EventLoopBuilderExtWindows, WindowBuilderExtWindows};
use winit::window::WindowBuilder;

use crate::app::context::AppContext;
use crate::config::{AppConfig, Fullscreen};
use crate::gl::create_gl;
use crate::handler::AppHandler;
use crate::util::get_monitor;

pub mod context;

pub struct MayApp {
    config: AppConfig,
}

impl MayApp {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn run<P: 'static, H: AppHandler<P>>(&mut self, mut handler: H) {
        let event_loop = EventLoopBuilder::<P>::with_user_event()
            .with_any_thread(self.config.window.any_thread)
            .build()
            .expect("Failed to create event loop");

        let window = WindowBuilder::new()
            .with_decorations(self.config.window.decorations)
            .with_resizable(self.config.window.resizable)
            .with_transparent(self.config.window.transparent)
            .with_maximized(self.config.window.maximized)
            .with_title(&self.config.window.title)
            .with_fullscreen(self.config.window.fullscreen.map(|fullscreen| {
                let monitor = event_loop.primary_monitor().unwrap_or_else(|| {
                    event_loop
                        .available_monitors()
                        .next()
                        .expect("Failed to get any monitor")
                });

                match fullscreen {
                    Fullscreen::Exclusive => winit::window::Fullscreen::Exclusive(
                        monitor
                            .clone()
                            .video_modes()
                            .next()
                            .expect("Failed to get any monitor's video mode"),
                    ),
                    Fullscreen::Borderless => {
                        winit::window::Fullscreen::Borderless(Some(monitor.clone()))
                    }
                }
            }))
            .with_position(self.config.window.position)
            .with_window_level(self.config.window.level)
            .with_blur(self.config.window.blur)
            .with_visible(self.config.window.visible)
            .with_active(self.config.window.active)
            .with_skip_taskbar(self.config.window.skip_taskbar)
            .with_taskbar_icon(self.config.window.taskbar_icon.clone())
            .with_window_icon(self.config.window.window_icon.clone())
            .build(&event_loop)
            .expect("Failed to create window");

        window.set_min_inner_size(self.config.window.min_size);
        window.set_max_inner_size(self.config.window.max_size);

        let (mut gl_ctx, mut gl_surface, mut canvas) = create_gl(&window, &self.config.graphics);

        event_loop
            .run(move |event, elwt| match event {
                Event::NewEvents(cause) => handler.on_new_event(
                    cause,
                    AppContext {
                        canvas: &mut canvas,
                        window: &window,
                        elwt,
                    },
                ),

                Event::WindowEvent { event, window_id } if window.id() == window_id => handler
                    .on_window_event(
                        event,
                        AppContext {
                            canvas: &mut canvas,
                            window: &window,
                            elwt,
                        },
                    ),

                Event::DeviceEvent { device_id, event } => handler.on_device_event(
                    device_id,
                    event,
                    AppContext {
                        canvas: &mut canvas,
                        window: &window,
                        elwt,
                    },
                ),

                Event::UserEvent(event) => handler.on_proxy_event(
                    event,
                    AppContext {
                        canvas: &mut canvas,
                        window: &window,
                        elwt,
                    },
                ),

                Event::Suspended => handler.on_suspended(AppContext {
                    window: &window,
                    canvas: &canvas,
                    elwt,
                }),

                Event::Resumed => handler.on_resumed(AppContext {
                    window: &window,
                    canvas: &canvas,
                    elwt,
                }),

                Event::AboutToWait => handler.on_about_to_wait(AppContext {
                    window: &window,
                    canvas: &canvas,
                    elwt,
                }),

                Event::LoopExiting => handler.on_existing(AppContext {
                    window: &window,
                    canvas: &canvas,
                    elwt,
                }),

                Event::MemoryWarning => handler.on_mem_warn(AppContext {
                    window: &window,
                    canvas: &canvas,
                    elwt,
                }),

                _ => (),
            })
            .expect("Failed to run event loop");
    }
}
