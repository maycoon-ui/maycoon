use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::config::{ConfigSurfaceTypes, ConfigTemplateBuilder};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::num::NonZeroU32;
use std::time::Instant;
use taffy::{Dimension, Size, Style, TaffyTree};
use winit::event::{Event, Modifiers, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::platform::windows::{EventLoopBuilderExtWindows, WindowBuilderExtWindows};
use winit::window::WindowBuilder;

use crate::app::context::{AppCommand, AppContext};
use crate::app::page::Page;
use crate::app::render::{build_widget, layout_widget, render_sketches, update_widget};
use crate::config::{AppConfig, Fullscreen};
use crate::widget::interaction::InteractionInfo;
use crate::widget::update::Update;
use crate::widget::{DummyWidget, Widget};

pub mod context;
pub mod page;
pub mod render;

pub struct MayApp {
    config: AppConfig,
    page: Box<dyn Page>,
}

impl MayApp {
    pub fn new(config: AppConfig, page: impl Page + 'static) -> Self {
        Self {
            config,
            page: Box::new(page),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoopBuilder::new()
            .with_any_thread(self.config.window.any_thread)
            .build()
            .expect("Failed to create event loop");

        let monitor = event_loop.primary_monitor().unwrap_or_else(|| {
            event_loop
                .available_monitors()
                .next()
                .expect("Failed to get any monitor")
        });

        let window = WindowBuilder::new()
            .with_decorations(self.config.window.decorations)
            .with_resizable(self.config.window.resizable)
            .with_transparent(self.config.window.transparent)
            .with_maximized(self.config.window.maximized)
            .with_title(&self.config.window.title)
            .with_fullscreen(self.config.window.fullscreen.map(|fullscreen| {
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

        let (gl_ctx, gl_surface, mut canvas) = {
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
                        .prefer_hardware_accelerated(self.config.graphics.hardware_acceleration)
                        .with_api(self.config.graphics.gl)
                        .prefer_hardware_accelerated(Some(true))
                        .with_surface_type(ConfigSurfaceTypes::WINDOW)
                        .with_multisampling(self.config.graphics.multisampling)
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
                    OpenGl::new_from_function_cstr(|cstr| {
                        display.get_proc_address(cstr) as *const _
                    })
                }
                .expect("Failed to create OpenGl renderer"),
            )
            .expect("Failed to create OpenGl canvas");

            (context, surface, canvas)
        };

        let mut taffy = TaffyTree::<()>::new();

        let mut info = InteractionInfo {
            keys: Vec::new(),
            cursor: None,
            modifiers: Modifiers::default(),
        };

        let mut dpi = window.scale_factor();

        let mut widget: Box<dyn Widget> = Box::new(DummyWidget::new());

        let mut update = Update::all();

        let mut frames_per_second: u32 = 0;

        let mut frames: u32 = 0;

        let mut now = Instant::now();

        let window_node = taffy
            .new_leaf(Style {
                size: Size::<Dimension> {
                    width: Dimension::Length(window.inner_size().width as f32),
                    height: Dimension::Length(window.inner_size().height as f32),
                },
                ..Default::default()
            })
            .expect("Failed to create window node");

        let mut first_run = true;

        #[cfg(feature = "default-font")]
        {
            canvas
                .add_font_mem(include_bytes!("../../../assets/data/Roboto-Regular.ttf"))
                .expect("Failed to add default font");
        }

        {
            let mut app_ctx = AppContext {
                window: &window,
                monitor: &monitor,
                commands: Vec::new(),
                dpi,
                update,
                canvas: &mut canvas,
                fps: frames_per_second,
            };

            self.page.init(&mut app_ctx);

            for cmd in app_ctx.commands {
                match cmd {
                    AppCommand::Exit => {
                        // todo: add warning log message
                        std::process::exit(0);
                    }

                    AppCommand::SetControl(ctrl) => {
                        event_loop.set_control_flow(ctrl);
                    }
                }
            }
        }

        event_loop
            .run(move |event, elwt| {
                match event {
                    Event::WindowEvent {
                        window_id,
                        event: window_event,
                    } if window_id == window.id() => {
                        match window_event {
                            WindowEvent::Resized(new_size) => {
                                canvas.set_size(new_size.width, new_size.height, dpi as f32);
                                taffy
                                    .set_style(
                                        window_node,
                                        Style {
                                            size: Size::<Dimension> {
                                                width: Dimension::Length(new_size.width as f32),
                                                height: Dimension::Length(new_size.height as f32),
                                            },
                                            ..Default::default()
                                        },
                                    )
                                    .expect("Failed to set style");
                                update.insert(Update::FORCE);
                                update.insert(Update::EVAL);
                            }

                            WindowEvent::CloseRequested => {
                                if self.config.window.close_on_request {
                                    elwt.exit();
                                }
                            }

                            WindowEvent::DroppedFile(file) => {
                                update.insert(Update::EVAL);
                                // todo
                            }

                            WindowEvent::HoveredFile(file) => {
                                update.insert(Update::EVAL);
                                // todo
                            }

                            WindowEvent::HoveredFileCancelled => {
                                update.insert(Update::EVAL);
                                // todo
                            }

                            WindowEvent::KeyboardInput {
                                event: key_event, ..
                            } => {
                                info.keys.push(key_event);
                                update.insert(Update::EVAL);
                            }

                            WindowEvent::ModifiersChanged(mods) => {
                                info.modifiers = mods;
                                update.insert(Update::EVAL);
                            }

                            WindowEvent::CursorMoved { position, .. } => {
                                info.cursor = Some(position);
                                update.insert(Update::EVAL);
                            }

                            WindowEvent::CursorLeft { .. } => {
                                info.cursor = None;
                                update.insert(Update::EVAL);
                            }

                            WindowEvent::MouseWheel { delta, phase, .. } => {
                                update.insert(Update::EVAL);
                                // todo
                            }

                            WindowEvent::MouseInput { state, button, .. } => {
                                update.insert(Update::EVAL);
                                // todo
                            }

                            WindowEvent::RedrawRequested => {
                                if self.config.debug.immediate_redraw {
                                    window.request_redraw();
                                }

                                update.insert(Update::EVAL);
                            }

                            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                                dpi = scale_factor;
                                update.insert(Update::FORCE);
                                update.insert(Update::EVAL);
                            }

                            _ => (),
                        }
                    }

                    Event::LoopExiting => {
                        // todo
                    }

                    Event::MemoryWarning => {
                        // todo
                    }

                    _ => (),
                }

                if update.contains(Update::EVAL) {
                    frames += 1;

                    let size = (canvas.width().clone(), canvas.height().clone());

                    // todo: wrap in context or draw if update
                    canvas.clear_rect(
                        0,
                        0,
                        size.0,
                        size.1,
                        self.config.graphics.theme.window_background(),
                    );

                    {
                        let mut app_ctx = AppContext {
                            window: &window,
                            monitor: &monitor,
                            commands: Vec::new(),
                            dpi,
                            update,
                            canvas: &mut canvas,
                            fps: frames_per_second,
                        };

                        widget = self.page.render(&mut app_ctx);

                        update = app_ctx.update;

                        for commands in app_ctx.commands {
                            match commands {
                                AppCommand::Exit => elwt.exit(),
                                AppCommand::SetControl(ctrl) => {
                                    elwt.set_control_flow(ctrl);
                                }
                            }
                        }
                    }

                    if first_run {
                        layout_widget(&widget, &mut taffy, window_node);
                        first_run = false;
                    }

                    update_widget(&mut widget, &info, &taffy, window_node, 0, update);

                    let sketches = build_widget(
                        &mut widget,
                        window_node,
                        &mut taffy,
                        self.config.graphics.force_antialiasing,
                        &self.config.graphics.theme,
                        &mut update,
                    );

                    render_sketches(sketches, &mut canvas);

                    canvas.flush();

                    gl_surface
                        .swap_buffers(&gl_ctx)
                        .expect("Failed to swap buffers");

                    if now.elapsed().as_secs() >= 1 {
                        now = Instant::now();
                        frames_per_second = frames;
                        frames = 0;
                    }
                }
            })
            .expect("Failed to run event loop");
    }
}
