pub mod update;

use crate::app::update::Update;
use crate::config::{DevicePreference, Fullscreen, MayConfig, RenderLevel};
use crate::layout::LayoutStyle;
use crate::state::State;
use crate::widget::{LayoutNode, StyleNode, Widget};
use euclid::Size2D;
use may_theme::theme::Theme;
use pathfinder_canvas::{Canvas, CanvasFontContext, FillRule, FillStyle, Path2D};
use pathfinder_color::ColorF;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use pathfinder_gl::GLDevice;
use pathfinder_renderer::concurrent::executor::Executor;
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{
    DestFramebuffer, RendererLevel, RendererMode, RendererOptions,
};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_renderer::scene::Scene;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use std::time::Instant;
use surfman::{
    Adapter, Connection, ContextAttributeFlags, ContextAttributes, Device, GLVersion,
    SurfaceAccess, SurfaceType,
};
use taffy::{AvailableSpace, Dimension, NodeId, Size, Style, TaffyTree, TraversePartialTree};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::windows::WindowBuilderExtWindows;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::{Window, WindowBuilder};

// without this, surfman won't be able to find a suitable device
surfman::declare_surfman!();

pub struct MayApp<T: Theme> {
    config: MayConfig<T>,
}

impl<T: Theme> MayApp<T> {
    pub fn new(config: MayConfig<T>) -> Self {
        Self { config }
    }

    pub fn run<S: State, W: Widget<S>>(mut self, mut state: S, mut widget: W) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");

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
                self.config.window.position.x,
                self.config.window.position.y,
            ))
            .with_inner_size(LogicalSize::new(
                self.config.window.size.width,
                self.config.window.size.height,
            ))
            .with_window_level(self.config.window.level)
            .with_blur(self.config.window.blur)
            .with_visible(self.config.window.visible)
            .with_active(self.config.window.active)
            .with_skip_taskbar(self.config.window.skip_taskbar)
            // TODO: .with_taskbar_icon(self.config.window.taskbar_icon.clone())
            // TODO: .with_window_icon(self.config.window.window_icon.clone())
            .build(&event_loop)
            .expect("Failed to create window");

        let connection = Connection::from_display_handle(
            window
                .display_handle()
                .expect("Failed to get display handle"),
        )
        .expect("Failed to create connection");

        let adapter = match self.config.render.preference {
            DevicePreference::LowPower => connection.create_low_power_adapter(),
            DevicePreference::HighPerformance => connection.create_hardware_adapter(),
            DevicePreference::Software => connection.create_software_adapter(),
        }
        .expect("Failed to create adapter");

        let mut device = connection
            .create_device(&adapter)
            .expect("Failed to create device");

        let mut context = device
            .create_context(
                &device
                    .create_context_descriptor(&ContextAttributes {
                        version: match self.config.render.mode {
                            RenderLevel::Auto => GLVersion::new(3, 0),
                            RenderLevel::Primary => GLVersion::new(4, 3),
                            RenderLevel::Secondary => GLVersion::new(3, 0),
                        },
                        flags: ContextAttributeFlags::ALPHA,
                    })
                    .expect("Failed to create context descriptor"),
                None,
            )
            .expect("Failed to create context");

        let mut surface = device
            .create_surface(
                &context,
                SurfaceAccess::GPUOnly,
                SurfaceType::Widget {
                    native_widget: connection
                        .create_native_widget_from_window_handle(
                            window.window_handle().expect("Failed to get window handle"),
                            Size2D::new(
                                window.inner_size().width as i32,
                                window.inner_size().height as i32,
                            ),
                        )
                        .expect("Failed to create native widget"),
                },
            )
            .expect("Failed to create surface");

        device
            .bind_surface_to_context(&mut context, surface)
            .expect("Failed to bind surface to context");

        device
            .make_context_current(&context)
            .expect("Failed to make context current");

        gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));

        let pathfinder_device = GLDevice::new(
            pathfinder_gl::GLVersion::GL4,
            device
                .context_surface_info(&context)
                .expect("Failed to get context surface info")
                .expect("Failed to get surface info")
                .framebuffer_object,
        );

        // TODO: add filesystem loader
        let resource_loader = EmbeddedResourceLoader::new();

        let render_level = match self.config.render.mode {
            RenderLevel::Auto => RendererLevel::default_for_device(&pathfinder_device),
            RenderLevel::Primary => RendererLevel::D3D11,
            RenderLevel::Secondary => RendererLevel::D3D9,
        };

        let mut renderer = Renderer::new(
            pathfinder_device,
            &resource_loader,
            RendererMode {
                level: render_level,
            },
            RendererOptions {
                dest: DestFramebuffer::full_window(Vector2I::new(
                    window.inner_size().width as i32,
                    window.inner_size().height as i32,
                )),
                background_color: Some(ColorF::white()),
                show_debug_ui: false,
            },
        );

        let canvas_font_context = CanvasFontContext::from_system_source(); // TODO: add custom fonts

        let mut canvas = Canvas::new(Vector2F::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        ))
        .get_context_2d(canvas_font_context.clone());

        let mut update = Update::FORCE;

        let mut taffy: TaffyTree<()> = TaffyTree::with_capacity(32);

        let window_node = taffy
            .new_leaf(LayoutStyle {
                size: Size::<Dimension> {
                    width: Dimension::Length(window.inner_size().width as f32),
                    height: Dimension::Length(window.inner_size().height as f32),
                },
                ..Default::default()
            })
            .expect("Failed to create window node");

        canvas.set_fill_style(FillStyle::Color(self.config.theme.window_background()));
        canvas.clear_rect(RectF::new(
            Vector2F::zero(),
            Vector2F::new(
                window.inner_size().width as f32,
                window.inner_size().height as f32,
            ),
        ));

        event_loop
            .run(move |event, elwt| {
                match event {
                    Event::WindowEvent { event, window_id } => {
                        if window_id == window.id() {
                            match event {
                                WindowEvent::Resized(new_size) => {
                                    {
                                        let canvas_mut = canvas.canvas_mut();

                                        canvas_mut.set_size(Vector2I::new(
                                            new_size.width as i32,
                                            new_size.height as i32,
                                        ));
                                    }

                                    canvas.set_fill_style(FillStyle::Color(
                                        self.config.theme.window_background(),
                                    ));

                                    canvas.clear_rect(RectF::new(
                                        Vector2F::zero(),
                                        Vector2F::new(
                                            new_size.width as f32,
                                            new_size.height as f32,
                                        ),
                                    ));

                                    taffy
                                        .set_style(
                                            window_node,
                                            Style {
                                                size: Size::<Dimension> {
                                                    width: Dimension::Length(new_size.width as f32),
                                                    height: Dimension::Length(
                                                        new_size.height as f32,
                                                    ),
                                                },
                                                ..Default::default()
                                            },
                                        )
                                        .expect("Failed to set window style");

                                    // update.insert(Update::EVAL); // gets automatically inserted once RedrawRequest even is processed
                                    update.insert(Update::LAYOUT);
                                    update.insert(Update::DRAW);
                                }

                                WindowEvent::CloseRequested => elwt.exit(),

                                WindowEvent::RedrawRequested => {
                                    update.insert(Update::EVAL);
                                }

                                _ => (),
                            }
                        }
                    }

                    Event::LoopExiting => {
                        let mut surface = device
                            .unbind_surface_from_context(&mut context)
                            .expect("Failed to unbind surface from context")
                            .expect("Failed to get surface");

                        device
                            .destroy_surface(&mut context, &mut surface)
                            .expect("Failed to destroy surface");
                    }

                    Event::Resumed => {}

                    _ => (),
                }

                if update.contains(Update::EVAL) || update.contains(Update::FORCE) {
                    // if true, layout of widgets is not built yet
                    if taffy.child_count(window_node) == 0 {
                        let node =
                            self.layout_widget(&mut taffy, window_node, widget.layout_style()); // TODO: make `node` one variable and mutate it

                        taffy
                            .add_child(window_node, node)
                            .expect("Failed to add child");

                        taffy
                            .compute_layout(window_node, Size::<AvailableSpace>::max_content())
                            .expect("Failed to compute layout");
                    }

                    let node = {
                        let children = taffy.children(window_node).expect("Failed to get children");
                        children.first().expect("Failed to get first child").clone()
                    };

                    let mut layout = self.collect_widget_layout(&mut taffy, node.clone());

                    update.insert(widget.update(&layout, &mut state));

                    if update.contains(Update::LAYOUT) || update.contains(Update::FORCE) {
                        taffy
                            .set_children(window_node, &[])
                            .expect("Failed to set children");

                        self.layout_widget(&mut taffy, window_node, widget.layout_style());

                        taffy
                            .compute_layout(window_node, Size::<AvailableSpace>::max_content())
                            .expect("Failed to compute layout");

                        layout = self.collect_widget_layout(&mut taffy, node.clone());
                    }

                    if update.contains(Update::DRAW) || update.contains(Update::FORCE) {
                        canvas.clear();

                        widget.render(&mut canvas, &layout, &mut self.config.theme);

                        SceneProxy::from_scene(
                            canvas.canvas().scene().clone(), // TODO: remove clone
                            render_level,
                            RayonExecutor, // TODO: config
                        )
                        .build_and_render(
                            &mut renderer,
                            BuildOptions {
                                transform: Default::default(),
                                dilation: Default::default(),
                                subpixel_aa_enabled: false, // TODO: config
                            },
                        );
                    }

                    update = Update::empty();
                }
            })
            .expect("Failed to run event loop");
    }

    fn layout_widget(&self, taffy: &mut TaffyTree, parent: NodeId, style: StyleNode) -> NodeId {
        let node = taffy.new_leaf(style.style).expect("Failed to create node");

        for child in style.children {
            let child = self.layout_widget(taffy, node, child);

            taffy.add_child(node, child).expect("Failed to add child");
        }

        node
    }

    fn collect_widget_layout(&self, taffy: &mut TaffyTree, node: NodeId) -> LayoutNode {
        // TODO: remove unnecessary clones

        let children = taffy
            .children(node)
            .expect("Failed to get children")
            .into_iter()
            .map(|el| self.collect_widget_layout(taffy, el))
            .collect::<Vec<LayoutNode>>();

        let layout = taffy.layout(node).expect("Failed to get layout").clone();

        LayoutNode { layout, children }
    }
}
