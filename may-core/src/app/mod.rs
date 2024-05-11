use std::num::NonZeroU32;

use crate::app::context::Context;
use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::config::{AppConfig, Fullscreen};
use crate::gl::create_gl;
use crate::render::RenderCommand;
use crate::state::State;
use crate::widget::{Widget, WidgetLayoutNode, WidgetStyleNode};
use glutin::surface::GlSurface;
use may_math::point::Point;
use taffy::{Dimension, NodeId, Size, Style, TaffyTree};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::platform::windows::{EventLoopBuilderExtWindows, WindowBuilderExtWindows};
use winit::window::WindowBuilder;

pub mod context;
pub mod info;
pub mod update;

pub struct MayApp {
    config: AppConfig,
}

impl MayApp {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn run<W: Widget<S>, S: State>(&mut self, mut widget: W, mut state: S) {
        let event_loop = EventLoopBuilder::new()
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
            .with_position(PhysicalPosition::<f64>::new(
                self.config
                    .window
                    .position
                    .x
                    .as_fixed()
                    .expect("Window position must be fixed") as f64,
                self.config
                    .window
                    .position
                    .y
                    .as_fixed()
                    .expect("Window position must be fixed") as f64,
            ))
            .with_window_level(self.config.window.level)
            .with_blur(self.config.window.blur)
            .with_visible(self.config.window.visible)
            .with_active(self.config.window.active)
            .with_skip_taskbar(self.config.window.skip_taskbar)
            .with_taskbar_icon(self.config.window.taskbar_icon.clone())
            .with_window_icon(self.config.window.window_icon.clone())
            .build(&event_loop)
            .expect("Failed to create window");

        window.set_min_inner_size(if let Some(size) = self.config.window.min_size {
            Some(PhysicalSize::new(
                size.width.as_fixed().expect("Window size must be fixed"),
                size.height.as_fixed().expect("Window size must be fixed"),
            ))
        } else {
            None
        });

        window.set_max_inner_size(if let Some(size) = self.config.window.max_size {
            Some(PhysicalSize::new(
                size.width.as_fixed().expect("Window size must be fixed"),
                size.height.as_fixed().expect("Window size must be fixed"),
            ))
        } else {
            None
        });

        let (gl_ctx, gl_surface, mut canvas) = unsafe { create_gl(&window, &self.config.graphics) };

        let mut info = AppInfo { cursor_pos: None };

        let mut taffy = TaffyTree::<()>::with_capacity(64);

        let mut window_node = taffy
            .new_leaf(Style {
                size: Size::<Dimension> {
                    width: Dimension::Length(window.inner_size().width as f32),
                    height: Dimension::Length(window.inner_size().height as f32),
                },
                ..Style::default()
            })
            .expect("Failed to create window node");

        let mut update = Update::FORCE;

        #[cfg(feature = "roboto-font")]
        {
            canvas
                .add_font_mem(include_bytes!("../../../assets/data/Roboto-Regular.ttf"))
                .expect("Failed to add font");
        }

        event_loop
            .run(move |event, elwt| {
                match event {
                    Event::WindowEvent { event, window_id } if window.id() == window_id => {
                        match event {
                            WindowEvent::CursorLeft { .. } => {
                                info.cursor_pos = None;
                                update.insert(Update::EVAL);
                            }

                            WindowEvent::CursorMoved { position, .. } => {
                                info.cursor_pos =
                                    Some(Point::from((position.x as f32, position.y as f32)));
                                update.insert(Update::EVAL);
                            }

                            WindowEvent::Resized(new_size) => {
                                canvas.set_size(
                                    new_size.width,
                                    new_size.height,
                                    window.scale_factor() as f32,
                                );

                                gl_surface.resize(
                                    &gl_ctx,
                                    NonZeroU32::new(new_size.width)
                                        .unwrap_or(NonZeroU32::new(1).unwrap()),
                                    NonZeroU32::new(new_size.height)
                                        .unwrap_or(NonZeroU32::new(1).unwrap()),
                                );

                                update.insert(Update::FORCE);
                            }

                            WindowEvent::CloseRequested => elwt.exit(),

                            WindowEvent::RedrawRequested => {
                                update.insert(Update::EVAL);
                            }

                            _ => (),
                        }
                    }

                    _ => (),
                }

                if update.contains(Update::EVAL) || update.contains(Update::FORCE) {
                    let ctx = Context {
                        window: &window,
                        app_info: &info,
                    };

                    update.insert(widget.update(&mut state, &ctx));

                    if update.contains(Update::LAYOUT) || update.contains(Update::FORCE) {
                        taffy.clear();

                        window_node = taffy
                            .new_leaf(Style {
                                size: Size::<Dimension> {
                                    width: Dimension::Length(window.inner_size().width as f32),
                                    height: Dimension::Length(window.inner_size().height as f32),
                                },
                                ..Style::default()
                            })
                            .expect("Failed to create window node");

                        Self::layout_compute(widget.style_node(), &mut taffy, window_node);
                    }

                    if update.contains(Update::DRAW) || update.contains(Update::FORCE) {
                        canvas.clear_rect(
                            0,
                            0,
                            canvas.width(),
                            canvas.height(),
                            self.config.theme.window_background(),
                        );

                        let node = taffy
                            .child_at_index(window_node, 0)
                            .expect("Failed to get window node");

                        let commands = widget.render(
                            self.config.theme.scheme_of(widget.id()).unwrap_or(
                                self.config.theme.default_scheme_of(widget.widget_type()),
                            ),
                            Self::layout_of(widget.style_node(), &mut taffy, node),
                        );

                        for command in commands {
                            match command {
                                RenderCommand::None => (),
                                RenderCommand::Fill { path, paint } => {
                                    canvas.fill_path(&path, &paint);
                                }
                                RenderCommand::FillText { text, paint, x, y } => {
                                    canvas
                                        .fill_text(x, y, &text, &paint)
                                        .expect("Failed to fill text");
                                }
                            }
                        }

                        canvas.flush();
                        gl_surface
                            .swap_buffers(&gl_ctx)
                            .expect("Failed to swap buffers");
                    }

                    info.reset();
                    update.clear();
                }
            })
            .expect("Failed to run event loop");
    }

    fn layout_compute(style: WidgetStyleNode, taffy: &mut TaffyTree, parent: NodeId) {
        let node = Self::layout_node(&style, taffy, parent);

        for c in style.children {
            Self::layout_of(c, taffy, node);
        }

        taffy
            .compute_layout(parent, Size::max_content())
            .expect("Failed to compute layout");
    }

    fn layout_of(style: WidgetStyleNode, taffy: &mut TaffyTree, node: NodeId) -> WidgetLayoutNode {
        let children = style
            .children
            .into_iter()
            .map(|c| Self::layout_of(c, taffy, node))
            .collect();

        WidgetLayoutNode {
            children,
            layout: taffy.layout(node).expect("Failed to get layout").clone(),
        }
    }

    fn layout_node(style: &WidgetStyleNode, taffy: &mut TaffyTree, parent: NodeId) -> NodeId {
        let node = taffy
            .new_leaf(style.style.clone())
            .expect("Failed to create widget node");

        taffy
            .add_child(parent, node)
            .expect("Failed to add widget to window");

        for child in style.children.clone() {
            Self::layout_node(&child, taffy, node);
        }

        node
    }
}
