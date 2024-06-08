use std::sync::Arc;
use std::time::{Duration, Instant};

use nalgebra::Vector2;
use taffy::{
    AvailableSpace, Dimension, NodeId, Style, TaffyResult, TaffyTree, TraversePartialTree,
};
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowAttributes, WindowId};

use may_theme::theme::Theme;

use crate::app::info::AppInfo;
use crate::app::update::Update;
use crate::config::MayConfig;
use crate::layout::{LayoutNode, StyleNode};
use crate::state::State;
use crate::widget::Widget;

pub struct AppHandler<'a, T: Theme, W: Widget<S>, S: State> {
    config: MayConfig<T>,
    attrs: WindowAttributes,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    scene: Scene,
    surface: Option<RenderSurface<'a>>,
    taffy: TaffyTree,
    window_node: NodeId,
    widget: W,
    state: S,
    info: AppInfo,
    render_ctx: Option<RenderContext>,
    update: Update,
    last_update: Instant,
}

impl<'a, T: Theme, W: Widget<S>, S: State> AppHandler<'a, T, W, S> {
    pub fn new(attrs: WindowAttributes, config: MayConfig<T>, widget: W, state: S) -> Self {
        let mut taffy = TaffyTree::with_capacity(16);
        let window_node = taffy
            .new_leaf(Style {
                size: taffy::Size::<Dimension> {
                    width: Dimension::Length(config.window.size.x as f32),
                    height: Dimension::Length(config.window.size.y as f32),
                },
                ..Default::default()
            })
            .expect("Failed to create window node");

        Self {
            attrs,
            window: None,
            renderer: None,
            config,
            scene: Scene::new(),
            surface: None,
            taffy,
            state,
            widget,
            info: AppInfo::default(),
            window_node,
            render_ctx: None,
            update: Update::empty(),
            last_update: Instant::now(),
        }
    }

    fn layout_widget(&mut self, parent: NodeId, style: &StyleNode) -> TaffyResult<()> {
        let node = self.taffy.new_leaf(style.style.clone())?;

        self.taffy.add_child(parent, node)?;

        for child in &style.children {
            self.layout_widget(node, child)?;
        }

        Ok(())
    }

    fn compute_layout(&mut self) -> TaffyResult<()> {
        self.taffy.compute_layout(
            self.window_node,
            taffy::Size::<AvailableSpace> {
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

    fn collect_layout(&mut self, node: NodeId, style: &StyleNode) -> TaffyResult<LayoutNode> {
        let mut children = Vec::new();

        for child in &style.children {
            children.push(self.collect_layout(node, child)?);
        }

        Ok(LayoutNode {
            layout: self.taffy.layout(node)?.clone(),
            children,
        })
    }

    fn request_redraw(&self) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn update(&mut self, event_loop: &ActiveEventLoop) {
        // completely layout widgets if taffy is not set up yet (e.g. during first update)
        if self.taffy.child_count(self.window_node) == 0 {
            self.layout_widget(self.window_node, &self.widget.layout_style())
                .expect("Failed to layout window");

            self.compute_layout().expect("Failed to compute layout");

            self.update.set(Update::FORCE);
        }

        let mut layout_node = self
            .collect_layout(
                self.taffy
                    .child_at_index(self.window_node, 0)
                    .expect("Failed to get window node"),
                &self.widget.layout_style(),
            )
            .expect("Failed to collect layout");

        // update call to check if app should re-evaluate
        self.update.set(
            self.widget
                .update(&layout_node, &mut self.state, &self.info),
        );

        // check if app should re-evaluate layout
        if self.update.has_flag(Update::FORCE) || self.update.has_flag(Update::LAYOUT) {
            // clear all nodes (except root window node)
            self.taffy
                .set_children(self.window_node, &[])
                .expect("Failed to set children");

            self.layout_widget(self.window_node, &self.widget.layout_style())
                .expect("Failed to layout window");

            self.compute_layout().expect("Failed to compute layout");

            layout_node = self
                .collect_layout(self.window_node, &self.widget.layout_style())
                .expect("Failed to collect layout");
        }

        // check if app should redraw
        if self.update.has_flag(Update::FORCE) || self.update.has_flag(Update::DRAW) {
            // clear scene
            self.scene.reset();

            self.widget.render(
                &mut self.scene,
                &mut self.config.theme,
                &self.info,
                &layout_node,
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
            }
        }

        // check if app should re-evaluate
        if self.update.has_flag(Update::EVAL) || self.update.has_flag(Update::FORCE) {
            if let Some(window) = self.window.as_ref() {
                window.request_redraw();
            }
        }

        // reset AppInfo and update states
        self.info.reset();
        self.update = Update::empty();

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
    }
}

impl<'a, T: Theme, W: Widget<S>, S: State> ApplicationHandler for AppHandler<'a, T, W, S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut render_ctx = RenderContext::new();

        self.window = Some(Arc::new(
            event_loop
                .create_window(self.attrs.clone())
                .expect("Failed to create window"),
        ));

        self.surface = Some(
            futures_lite::future::block_on(async {
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

        let device_handle = render_ctx.devices.first().expect("Failed to select device");

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
                    num_init_threads: None,
                },
            )
            .expect("Failed to create renderer"),
        );

        self.render_ctx = Some(render_ctx);
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
                            if let Some(ctx) = &self.render_ctx {
                                if let Some(surface) = &mut self.surface {
                                    ctx.resize_surface(surface, new_size.width, new_size.height);
                                }
                            }

                            self.request_redraw();

                            self.update.set(Update::DRAW | Update::LAYOUT);
                        }
                    }

                    WindowEvent::CloseRequested => {
                        if let Some(render_ctx) = self.render_ctx.as_mut() {
                            for handle in &render_ctx.devices {
                                handle.device.destroy();
                            }
                        }

                        if self.config.window.close_on_request {
                            event_loop.exit();
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        self.update(event_loop);
                    }

                    WindowEvent::CursorLeft { .. } => {
                        self.info.cursor_pos = None;
                        self.request_redraw();
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        self.info.cursor_pos = Some(Vector2::new(position.x, position.y));
                        self.request_redraw();
                    }

                    WindowEvent::KeyboardInput {
                        event,
                        device_id,
                        is_synthetic,
                    } => {
                        if !is_synthetic {
                            self.info.keys.push((device_id, event));
                            self.request_redraw();
                        }
                    }

                    WindowEvent::MouseInput {
                        device_id,
                        button,
                        state,
                    } => {
                        self.info.buttons.push((device_id, button, state));
                        self.request_redraw();
                    }

                    _ => (),
                }
            }
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.window = None;
        self.surface = None;
        self.render_ctx = None;
        self.renderer = None;
        self.info.reset();
    }
}
