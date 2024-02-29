pub mod page;
pub mod context;
pub mod render;

use std::num::NonZeroU32;
use femtovg::Canvas;
use femtovg::renderer::OpenGl;
use glutin::config::{ConfigSurfaceTypes, ConfigTemplateBuilder};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use taffy::{Dimension, NodeId, PrintTree, Size, Style, TaffyTree};
use winit::event::{Event, Modifiers, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::platform::windows::{EventLoopBuilderExtWindows, MonitorHandleExtWindows, WindowBuilderExtWindows};
use winit::window::WindowBuilder;
use crate::app::context::{AppCommand, AppContext};
use crate::app::page::Page;
use crate::app::render::{draw_root_widget, render_sketches};
use crate::config::{AppConfig, Fullscreen};
use crate::widget::{PathMode, Sketch, Widget};
use crate::widget::interaction::{Action, InteractionInfo};

pub struct MayApp {
    config: AppConfig,
    page: Box<dyn Page>
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
            event_loop.available_monitors()
                .next().expect("Failed to get any monitor")
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
                        monitor.clone().video_modes()
                            .next()
                            .expect("Failed to get any monitor's video mode")
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

        let (gl_display, gl_ctx, gl_surface, mut canvas) = {
            let display = unsafe {
                let pref = DisplayApiPreference::Egl;

                #[cfg(target_os = "macos")]
                let pref = DisplayApiPreference::Cgl;

                #[cfg(target_os = "windows")]
                let pref = DisplayApiPreference::WglThenEgl(
                    Some(window.raw_window_handle())
                );

                Display::new(
                    window.raw_display_handle(),
                    pref
                )
            }.expect("Failed to create Gl display");

            let gl_config = unsafe {
                display.find_configs(
                    ConfigTemplateBuilder::new()
                        .compatible_with_native_window(window.raw_window_handle())
                        .prefer_hardware_accelerated(self.config.graphics.hardware_acceleration)
                        .with_api(self.config.graphics.gl)
                        .prefer_hardware_accelerated(Some(
                            true
                        ))
                        .with_surface_type(ConfigSurfaceTypes::WINDOW)
                        .with_multisampling(self.config.graphics.multisampling)
                        .build()
                )
            }.expect("Failed to find Gl config").next().expect("Failed to get any Gl config");

            let surface = unsafe {
                display.create_window_surface(
                    &gl_config,
                    &SurfaceAttributesBuilder::<WindowSurface>::new()
                        .build(
                            window.raw_window_handle(),
                            NonZeroU32::new_unchecked(window.inner_size().width),
                            NonZeroU32::new_unchecked(window.inner_size().height),
                        )
                )
            }.expect("Failed to create Gl surface");

            let context = unsafe {
                display.create_context(
                    &gl_config,
                    &ContextAttributesBuilder::new()
                        .build(Some(
                            window.raw_window_handle()
                        ))
                )
            }.expect("Failed to create Gl context")
                .make_current(&surface)
                .expect("Failed to make Gl context current");

            let canvas = Canvas::new(
                unsafe {
                    OpenGl::new_from_function_cstr(|cstr| {
                        display.get_proc_address(cstr) as *const _
                    })
                }.expect("Failed to create OpenGl renderer")
            ).expect("Failed to create OpenGl canvas");

            (display, context, surface, canvas)
        };

        let mut taffy = TaffyTree::<()>::new();

        let mut info = InteractionInfo {
            keys: Vec::new(),
            cursor: None,
            modifiers: Modifiers::default(),
        };

        let mut update = true;

        let mut dpi = window.scale_factor();

        #[cfg(feature = "default-font")]
        {
            canvas.add_font_mem(
                include_bytes!("../../../assets/data/Roboto-Regular.ttf")
            ).expect("Failed to add default font");
        }

        {
            let mut app_ctx = AppContext {
                window: &window,
                monitor: &monitor,
                commands: Vec::new(),
                dpi,
                update,
                canvas: &mut canvas,
            };

            for commands in app_ctx.commands {
                match commands {
                    AppCommand::Exit => panic!("Command cannot be executed when initializing the first page"),
                    AppCommand::SetControl(_) => panic!("Command cannot be executed when initializing the first page"),
                }
            }
        }

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    window_id,
                    event: window_event
                } if window_id == window.id() => {
                    match window_event {
                        WindowEvent::Resized(new_size) => {
                            canvas.set_size(
                                new_size.width,
                                new_size.height,
                                dpi as f32
                            );
                        }

                        WindowEvent::CloseRequested => {
                            if self.config.window.close_on_request {
                                elwt.exit();
                            }
                        }

                        WindowEvent::DroppedFile(file) => {
                            update = true;
                            // todo
                        }

                        WindowEvent::HoveredFile(file) => {
                            update = true;
                            // todo
                        }

                        WindowEvent::HoveredFileCancelled => {
                            update = true;
                            // todo
                        }

                        WindowEvent::KeyboardInput {
                            event: key_event,
                            ..
                        } => {
                            info.keys.push(key_event);
                            update = true;
                        }

                        WindowEvent::ModifiersChanged(mods) => {
                            info.modifiers = mods;
                            update = true;
                        }

                        WindowEvent::CursorMoved {
                            position,
                            ..
                        } => {
                            info.cursor = Some(position);
                            update = true;
                        }

                        WindowEvent::CursorLeft { .. } => {
                            info.cursor = None;
                            update = true;
                        }

                        WindowEvent::MouseWheel {
                            delta,
                            phase,
                            ..
                        } => {
                            update = true;
                            // todo
                        }

                        WindowEvent::MouseInput {
                            state,
                            button,
                            ..
                        } => {
                            update = true;
                            // todo
                        }

                        WindowEvent::RedrawRequested => {
                            taffy.clear();

                            let size = (
                                canvas.width().clone() as f32,
                                canvas.height().clone() as f32
                            );

                            let sketches = {
                                let mut app_ctx = AppContext {
                                    window: &window,
                                    monitor: &monitor,
                                    commands: Vec::new(),
                                    dpi,
                                    update: false,
                                    canvas: &mut canvas,
                                };

                                let sketches = draw_root_widget(
                                    &mut self.page,
                                    &mut app_ctx,
                                    size,
                                    &mut taffy,
                                    &mut info,
                                    self.config.graphics.antialiasing,
                                );

                                // todo: wrap in single function
                                for commands in app_ctx.commands {
                                    match commands {
                                        AppCommand::Exit => elwt.exit(),
                                        AppCommand::SetControl(ctrl) => {
                                            elwt.set_control_flow(ctrl);
                                        }
                                    }
                                }

                                update = app_ctx.update;

                                sketches
                            };

                            render_sketches(
                                sketches,
                                &mut canvas
                            );

                            canvas.flush();

                            gl_surface.swap_buffers(
                                &gl_ctx
                            ).expect("Failed to swap buffers");
                        }

                        WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            ..
                        } => {
                            dpi = scale_factor;
                            update = true;
                        }

                        _ => (),
                    }
                },

                Event::LoopExiting => {
                    // todo
                }

                Event::MemoryWarning => {
                    // todo
                }

                _ => (),
            }

            if update {
                window.request_redraw();
            }
        }).expect("Failed to run event loop");
    }
}
