use std::sync::Arc;
use std::time::{Duration, Instant};

use nalgebra::Vector2;
use taffy::{
    AvailableSpace, Dimension, NodeId, PrintTree, Size, Style, TaffyResult, TaffyTree,
    TraversePartialTree,
};
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::app::context::AppContext;
use crate::app::font_ctx::FontContext;
use crate::app::info::AppInfo;
use crate::app::update::{Update, UpdateManager};
use crate::config::MayConfig;
use crate::layout::{LayoutNode, StyleNode};
use crate::widget::Widget;
use maycoon_theme::theme::Theme;

/// The core application handler. You should use [MayApp](crate::app::MayApp) instead for running applications.
pub struct AppHandler<'a, T, W>
where
    T: Theme,
    W: Widget,
{
    config: MayConfig<T>,
    attrs: WindowAttributes,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    scene: Scene,
    surface: Option<RenderSurface<'a>>,
    taffy: TaffyTree,
    window_node: NodeId,
    widget: W,
    info: AppInfo,
    render_ctx: Option<RenderContext>,
    update: UpdateManager,
    last_update: Instant,
}

impl<T, W> AppHandler<'_, T, W>
where
    T: Theme,
    W: Widget,
{
    /// Create a new handler with given window attributes, config, widget and state.
    pub fn new(
        attrs: WindowAttributes,
        config: MayConfig<T>,
        widget: W,
        font_context: FontContext,
        update: UpdateManager,
    ) -> Self {
        let mut taffy = TaffyTree::with_capacity(16);

        // gets configured on resume
        let window_node = taffy
            .new_leaf(Style::default())
            .expect("Failed to create window node");

        let size = config.window.size;

        Self {
            attrs,
            window: None,
            renderer: None,
            config,
            scene: Scene::new(),
            surface: None,
            taffy,
            widget,
            info: AppInfo {
                font_context,
                size,
                ..Default::default()
            },
            window_node,
            render_ctx: None,
            update,
            last_update: Instant::now(),
        }
    }

    /// Get the application context.
    pub fn context(&self) -> AppContext {
        AppContext::new(self.update.clone())
    }

    /// Add the parent node and its children to the layout tree.
    fn layout_widget(&mut self, parent: NodeId, style: &StyleNode) -> TaffyResult<()> {
        log::trace!("Laying out widget: {:?}", parent);

        let node = self.taffy.new_leaf(style.style.clone().into())?;

        self.taffy.add_child(parent, node)?;

        for child in &style.children {
            self.layout_widget(node, child)?;
        }

        Ok(())
    }

    /// Compute the layout of the root node and its children.
    fn compute_layout(&mut self) -> TaffyResult<()> {
        log::trace!("Computing root layout.");

        self.taffy.compute_layout(
            self.window_node,
            Size::<AvailableSpace> {
                width: AvailableSpace::Definite(
                    self.window.as_ref().unwrap().inner_size().width as f32,
                ),
                height: AvailableSpace::Definite(
                    self.window.as_ref().unwrap().inner_size().height as f32,
                ),
            },
        )?;
        Ok(())
    }

    /// Collect the computed layout of the given node and its children. Make sure to call [AppHandler::compute_layout] before, to not get dirty results.
    fn collect_layout(&mut self, node: NodeId, style: &StyleNode) -> TaffyResult<LayoutNode> {
        log::trace!("Collecting layout for node: {:?}", node);

        let mut children = Vec::with_capacity(style.children.capacity());

        for (i, child) in style.children.iter().enumerate() {
            children.push(self.collect_layout(self.taffy.child_at_index(node, i)?, child)?);
        }

        Ok(LayoutNode {
            layout: *self.taffy.get_final_layout(node),
            children,
        })
    }

    /// Request a window redraw.
    fn request_redraw(&self) {
        log::trace!("Requesting redraw...");

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    /// Update the app and process events.
    fn update(&mut self, _: &ActiveEventLoop) {
        // completely layout widgets if taffy is not set up yet (e.g. during first update)
        if self.taffy.child_count(self.window_node) == 0 {
            log::trace!("Setting up layout...");

            let style = self.widget.layout_style();

            self.layout_widget(self.window_node, &style)
                .expect("Failed to layout window");

            self.compute_layout().expect("Failed to compute layout");

            self.update.insert(Update::FORCE);
        }

        let style = self.widget.layout_style();

        let mut layout_node = self
            .collect_layout(
                self.taffy.child_at_index(self.window_node, 0).unwrap(),
                &style,
            )
            .expect("Failed to collect layout");

        // update call to check if app should re-evaluate
        log::trace!("Updating root widget...");
        self.update
            .insert(self.widget.update(&layout_node, self.context(), &self.info));

        // check if app should re-evaluate layout
        if self.update.get().intersects(Update::LAYOUT | Update::FORCE) {
            log::trace!("Layout update detected!");

            // clear all nodes (except root window node)
            self.taffy
                .set_children(self.window_node, &[])
                .expect("Failed to set children");

            let style = self.widget.layout_style();

            self.layout_widget(self.window_node, &style)
                .expect("Failed to layout window");

            self.compute_layout().expect("Failed to compute layout");

            layout_node = self
                .collect_layout(
                    self.taffy.child_at_index(self.window_node, 0).unwrap(),
                    &style,
                )
                .expect("Failed to collect layout");
        }

        // check if app should redraw
        if self.update.get().intersects(Update::FORCE | Update::DRAW) {
            log::trace!("Draw update detected!");

            // clear scene
            self.scene.reset();

            let context = self.context();

            log::trace!("Rendering root widget...");
            self.widget.render(
                &mut self.scene,
                &mut self.config.theme,
                &layout_node,
                &self.info,
                context,
            );

            let renderer = self.renderer.as_mut().expect("Renderer not initialized");
            let render_ctx = self
                .render_ctx
                .as_ref()
                .expect("Render context not initialized");
            let surface = self.surface.as_ref().expect("Surface not initialized");
            let window = self.window.as_ref().expect("Window not initialized");

            let device_handle = render_ctx.devices.first().expect("No devices available");

            // check surface validity
            if window.inner_size().width != 0 && window.inner_size().height != 0 {
                let surface_texture = surface
                    .surface
                    .get_current_texture()
                    .expect("Failed to get surface texture");

                // make sure winit knows that the surface texture is ready to be presented
                window.pre_present_notify();

                // TODO: this panics if canvas didn't change (no operation was done) in debug mode
                renderer
                    .render_to_surface(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.scene,
                        &surface_texture,
                        &RenderParams {
                            base_color: self.config.theme.window_background(),
                            width: window.inner_size().width,
                            height: window.inner_size().height,
                            antialiasing_method: self.config.render.antialiasing,
                        },
                    )
                    .expect("Failed to render to surface");

                surface_texture.present();
            } else {
                log::trace!("Surface invalid. Skipping render.");
            }
        }

        // check if app should re-evaluate
        if self.update.get().intersects(Update::EVAL | Update::FORCE) {
            log::trace!("Evaluation update detected!");

            if let Some(window) = self.window.as_ref() {
                window.request_redraw();
            }
        }

        // reset AppInfo and update states
        self.info.reset();
        self.update.clear();

        // update diagnostics
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.last_update = Instant::now();

            // calc avg updates per sec through updates per sec NOW divided by 2
            self.info.diagnostics.updates_per_sec =
                (self.info.diagnostics.updates_per_sec + self.info.diagnostics.updates) / 2;

            // reset current updates per seconds
            self.info.diagnostics.updates = 0;
        } else {
            // increase updates per sec NOW by 1
            self.info.diagnostics.updates += 1;
        }

        log::trace!("Updates per sec: {}", self.info.diagnostics.updates_per_sec);
    }
}

impl<T, W> ApplicationHandler for AppHandler<'_, T, W>
where
    T: Theme,
    W: Widget,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Resuming/Starting app execution...");

        let mut render_ctx = RenderContext::new();

        log::trace!("Creating window...");
        self.window = Some(Arc::new(
            event_loop
                .create_window(self.attrs.clone())
                .expect("Failed to create window"),
        ));

        self.taffy
            .set_style(
                self.window_node,
                Style {
                    size: Size::<Dimension> {
                        width: Dimension::length(
                            self.window.as_ref().unwrap().inner_size().width as f32,
                        ),
                        height: Dimension::length(
                            self.window.as_ref().unwrap().inner_size().height as f32,
                        ),
                    },
                    ..Default::default()
                },
            )
            .expect("Failed to set window node style");

        self.surface = Some(
            crate::tasks::block_on(async {
                log::trace!("Creating surface...");

                render_ctx
                    .create_surface(
                        self.window.clone().unwrap(),
                        self.window.as_ref().unwrap().inner_size().width,
                        self.window.as_ref().unwrap().inner_size().height,
                        self.config.render.present_mode,
                    )
                    .await
            })
            .expect("Failed to create surface"),
        );

        log::trace!("Requesting device handle via selector...");
        let device_handle = (self.config.render.device_selector)(&render_ctx.devices);

        log::trace!("Creating renderer...");
        self.renderer = Some(
            Renderer::new(
                &device_handle.device,
                RendererOptions {
                    surface_format: Some(self.surface.as_ref().unwrap().format),
                    use_cpu: self.config.render.cpu,
                    antialiasing_support: match self.config.render.antialiasing {
                        AaConfig::Area => AaSupport::area_only(),

                        AaConfig::Msaa8 => AaSupport {
                            area: false,
                            msaa8: true,
                            msaa16: false,
                        },

                        AaConfig::Msaa16 => AaSupport {
                            area: false,
                            msaa8: false,
                            msaa16: true,
                        },
                    },
                    num_init_threads: self.config.render.init_threads,
                },
            )
            .expect("Failed to create renderer"),
        );

        self.render_ctx = Some(render_ctx);
        self.update.set(Update::FORCE);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &self.window {
            if window.id() == window_id {
                match event {
                    WindowEvent::Resized(new_size) => {
                        if new_size.width != 0 && new_size.height != 0 {
                            log::info!("Window resized to {}x{}", new_size.width, new_size.height);

                            if let Some(ctx) = &self.render_ctx {
                                if let Some(surface) = &mut self.surface {
                                    ctx.resize_surface(surface, new_size.width, new_size.height);
                                }
                            }

                            self.taffy
                                .set_style(
                                    self.window_node,
                                    Style {
                                        size: Size::<Dimension> {
                                            width: Dimension::length(new_size.width as f32),
                                            height: Dimension::length(new_size.height as f32),
                                        },
                                        ..Default::default()
                                    },
                                )
                                .expect("Failed to set window node style");

                            self.info.size =
                                Vector2::new(new_size.width as f64, new_size.height as f64);

                            self.request_redraw();

                            self.update.insert(Update::DRAW | Update::LAYOUT);
                        } else {
                            log::trace!("Window size is 0x0, ignoring resize event.");
                        }
                    },

                    WindowEvent::CloseRequested => {
                        log::info!("Window Close requested...");

                        log::trace!("Destroying device handles...");
                        if let Some(render_ctx) = self.render_ctx.as_mut() {
                            for handle in &render_ctx.devices {
                                handle.device.destroy();
                            }
                        }

                        if self.config.window.close_on_request {
                            event_loop.exit();
                        }
                    },

                    WindowEvent::RedrawRequested => {
                        self.update(event_loop);
                    },

                    WindowEvent::CursorLeft { .. } => {
                        self.info.cursor_pos = None;
                        self.request_redraw();
                    },

                    WindowEvent::CursorMoved { position, .. } => {
                        self.info.cursor_pos = Some(Vector2::new(position.x, position.y));
                        self.request_redraw();
                    },

                    WindowEvent::KeyboardInput {
                        event,
                        device_id,
                        is_synthetic,
                    } => {
                        if !is_synthetic {
                            self.info.keys.push((device_id, event));
                            self.request_redraw();
                        }
                    },

                    WindowEvent::MouseInput {
                        device_id,
                        button,
                        state,
                    } => {
                        self.info.buttons.push((device_id, button, state));
                        self.request_redraw();
                    },

                    WindowEvent::MouseWheel { delta, .. } => {
                        self.info.mouse_scroll_delta = Some(delta);
                        self.request_redraw();
                    },

                    WindowEvent::Destroyed => log::info!("Window destroyed! Exiting..."),

                    _ => (),
                }
            }
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        log::info!("Suspending application...");

        self.window = None;
        self.surface = None;
        self.render_ctx = None;
        self.renderer = None;
        self.info.reset();
    }
}
