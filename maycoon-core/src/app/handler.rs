use nalgebra::Vector2;
use std::sync::Arc;
use std::time::{Duration, Instant};
use taffy::{
    AvailableSpace, Dimension, NodeId, PrintTree, Size, Style, TaffyResult, TaffyTree,
    TraversePartialTree,
};
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
use crate::plugin::PluginManager;
use crate::vgi::{Scene, VectorGraphicsInterface};
use crate::widget::Widget;
use maycoon_theme::theme::Theme;

/// The core application handler. You should use [MayApp](crate::app::MayApp) instead for running applications.
pub struct AppHandler<T, W, S, F, V>
where
    T: Theme,
    W: Widget,
    F: Fn(AppContext, S) -> W,
    V: VectorGraphicsInterface,
{
    config: MayConfig<T, V>,
    attrs: WindowAttributes,
    window: Option<Arc<Window>>,
    scene: V::Scene,
    taffy: TaffyTree,
    window_node: NodeId,
    builder: F,
    state: Option<S>,
    widget: Option<W>,
    info: AppInfo,
    update: UpdateManager,
    last_update: Instant,
    plugins: PluginManager<T, V>,
    graphics: V,
}

impl<T, W, S, F, V> AppHandler<T, W, S, F, V>
where
    T: Theme,
    W: Widget,
    F: Fn(AppContext, S) -> W,
    V: VectorGraphicsInterface,
{
    /// Create a new handler with given window attributes, config, widget and state.
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn new(
        attrs: WindowAttributes,
        config: MayConfig<T, V>,
        builder: F,
        state: S,
        font_context: FontContext,
        update: UpdateManager,
        plugins: PluginManager<T, V>,
    ) -> Self {
        tracing::trace!("creating taffy tree");
        let mut taffy = TaffyTree::with_capacity(16);

        // gets configured on resume
        let window_node = taffy
            .new_leaf(Style::default())
            .expect("Failed to create window node");

        let size = config.window.size;

        let graphics = config.graphics.clone();

        Self {
            attrs,
            window: None,
            config,
            scene: Scene::new(),
            taffy,
            widget: None,
            info: AppInfo {
                font_context,
                size,
                ..Default::default()
            },
            window_node,
            builder,
            state: Some(state),
            update,
            last_update: Instant::now(),
            plugins,
            graphics: V::new(graphics).expect("Failed to create vector graphics interface"),
        }
    }

    /// Get the application context.
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn context(&self) -> AppContext {
        AppContext::new(self.update.clone(), self.info.diagnostics)
    }

    /// Add the parent node and its children to the layout tree.
    #[tracing::instrument(level = "trace", skip(self, style))]
    fn layout_widget(&mut self, parent: NodeId, style: &StyleNode) -> TaffyResult<()> {
        let node = self.taffy.new_leaf(style.style.clone().into())?;

        self.taffy.add_child(parent, node)?;

        for child in &style.children {
            self.layout_widget(node, child)?;
        }

        Ok(())
    }

    /// Compute the layout of the root node and its children.
    #[tracing::instrument(level = "trace", skip(self))]
    fn compute_layout(&mut self) -> TaffyResult<()> {
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
    #[tracing::instrument(level = "trace", skip(self, style))]
    fn collect_layout(&mut self, node: NodeId, style: &StyleNode) -> TaffyResult<LayoutNode> {
        tracing::trace!("collecting node layout {node:?}");

        let mut children = Vec::with_capacity(style.children.capacity());

        for (i, style) in style.children.iter().enumerate() {
            let child = self.taffy.child_at_index(node, i)?;

            tracing::trace!("collecting layout of child {child:?}");

            children.push(self.collect_layout(child, style)?);
        }

        Ok(LayoutNode {
            layout: self.taffy.get_final_layout(node),
            children,
        })
    }

    /// Request a window redraw.
    #[tracing::instrument(level = "trace", skip(self))]
    fn request_redraw(&self) {
        tracing::trace!("requesting redraw");
        self.window.as_ref().unwrap().request_redraw();
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn render(
        &mut self,
        window: Arc<Window>,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), V::Error> {
        tracing::trace!("rendering via vector graphics interface");
        self.graphics.render(
            window.clone(),
            event_loop,
            &self.scene,
            self.config.theme.window_background(),
        )?;

        Ok(())
    }

    /// Update the app and process events.
    #[tracing::instrument(level = "trace", skip_all)]
    fn update(&mut self, event_loop: &ActiveEventLoop) {
        // update plugins
        tracing::trace!("updating plugins");
        self.plugins.run(|pl| {
            pl.on_update(
                &mut self.config,
                self.window.as_ref().expect("Window not initialized"),
                &mut self.scene,
                &mut self.taffy,
                self.window_node,
                &mut self.info,
                &self.update,
                &mut self.last_update,
                event_loop,
            )
        });

        // completely layout widgets if taffy is not set up yet (e.g. during first update)
        if self.taffy.child_count(self.window_node) == 0 {
            tracing::trace_span!("complete layout").in_scope(|| {
                let style = self.widget.as_ref().unwrap().layout_style();

                self.layout_widget(self.window_node, &style)
                    .expect("Failed to layout window");

                self.compute_layout().expect("Failed to compute layout");

                self.update.insert(Update::FORCE);
            });
        }

        let style = self.widget.as_ref().unwrap().layout_style();

        let mut layout_node = self
            .collect_layout(
                self.taffy.child_at_index(self.window_node, 0).unwrap(),
                &style,
            )
            .expect("Failed to collect layout");

        // update call to check if app should re-evaluate
        let context = self.context();

        tracing::trace!("updating widget");
        self.update.insert(
            self.widget
                .as_mut()
                .unwrap()
                .update(&layout_node, context, &self.info),
        );

        // check if app should re-evaluate layout
        if self.update.get().intersects(Update::LAYOUT | Update::FORCE) {
            tracing::trace_span!("layout").in_scope(|| {
                // clear all nodes (except root window node)
                self.taffy
                    .set_children(self.window_node, &[])
                    .expect("Failed to set children");

                let style = self.widget.as_ref().unwrap().layout_style();

                self.layout_widget(self.window_node, &style)
                    .expect("Failed to layout window");

                self.compute_layout().expect("Failed to compute layout");

                layout_node = self
                    .collect_layout(
                        self.taffy.child_at_index(self.window_node, 0).unwrap(),
                        &style,
                    )
                    .expect("Failed to collect layout");
            });
        }

        // check if app should redraw
        if self.update.get().intersects(Update::FORCE | Update::DRAW) {
            tracing::trace_span!("draw").in_scope(|| {
                // clear scene
                tracing::trace!("resetting vector graphics interface scene");
                self.scene.reset();

                let context = self.context();

                tracing::trace!("drawing root widget");
                self.widget.as_mut().unwrap().render(
                    &mut self.scene,
                    &mut self.config.theme,
                    &layout_node,
                    &self.info,
                    context,
                );

                let window = self.window.clone().expect("Window not initialized");

                // check surface validity
                if window.inner_size().width != 0 && window.inner_size().height != 0 {
                    self.render(window, event_loop)
                        .expect("Failed rendering process");
                } else {
                    tracing::debug!("skipping render due to invalid surface");
                }
            });
        }

        // check if app should re-evaluate
        if self.update.get().intersects(Update::EVAL | Update::FORCE) {
            tracing::trace!("re-evaluating application state");
            self.request_redraw();
        }

        // update the app if requested
        if self.update.get().intersects(Update::EXIT) {
            tracing::trace!("exiting event loop");
            event_loop.exit();
            return;
        }

        // reset AppInfo and update states
        tracing::trace!("resetting app info and update states");
        self.info.reset();
        self.update.clear();

        // update diagnostics
        tracing::trace!("updating diagnostics");

        self.info.diagnostics.first_run = false;

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

        tracing::debug!("updates per sec: {}", self.info.diagnostics.updates_per_sec);
    }
}

impl<T, W, S, F, V> ApplicationHandler for AppHandler<T, W, S, F, V>
where
    T: Theme,
    W: Widget,
    F: Fn(AppContext, S) -> W,
    V: VectorGraphicsInterface,
{
    #[tracing::instrument(level = "trace", skip_all)]
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::debug!("resuming plugins");
        self.plugins.run(|pl| {
            pl.on_resume(
                &mut self.config,
                &mut self.scene,
                &mut self.taffy,
                self.window_node,
                &mut self.info,
                &self.update,
                &mut self.last_update,
                event_loop,
            )
        });

        tracing::info!("creating window");
        let window = Arc::new(
            event_loop
                .create_window(self.attrs.clone())
                .expect("Failed to create window"),
        );

        tracing::info!("initializing layout");
        let size = window.inner_size();

        self.taffy
            .set_style(
                self.window_node,
                Style {
                    size: Size::<Dimension> {
                        width: Dimension::length(size.width as f32),
                        height: Dimension::length(size.height as f32),
                    },
                    ..Default::default()
                },
            )
            .expect("Failed to set window node style");

        tracing::info!("initializing vector graphics interface");
        self.graphics
            .init(window.clone(), event_loop)
            .expect("Failed to initialize vector graphics interface");

        tracing::info!("building root widget");
        self.widget = Some((self.builder)(
            AppContext::new(self.update.clone(), self.info.diagnostics),
            self.state.take().unwrap(),
        ));

        self.window = Some(window);
    }

    #[tracing::instrument(level = "trace", skip_all, fields(event =
        ?event))]
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        mut event: WindowEvent,
    ) {
        tracing::trace!("running plugin window event");
        self.plugins.run(|pl| {
            pl.on_window_event(
                &mut event,
                &mut self.config,
                self.window.as_ref().unwrap(),
                &mut self.scene,
                &mut self.taffy,
                self.window_node,
                &mut self.info,
                &self.update,
                &mut self.last_update,
                event_loop,
            )
        });

        if let Some(window) = &self.window
            && window.id() == window_id
        {
            match event {
                WindowEvent::Resized(new_size) => {
                    tracing::debug!("resizing window to {new_size:?}");

                    if new_size.width != 0 && new_size.height != 0 {
                        tracing::trace!("resizing vector graphics interface");
                        self.graphics
                            .resize(
                                window.clone(),
                                event_loop,
                                Vector2::new(new_size.width, new_size.height),
                            )
                            .expect("Failed to resize vector graphics interface");

                        tracing::trace!("resizing root layout node");
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
                        tracing::trace!("window size is 0x0, ignoring resize event");
                    }
                },

                WindowEvent::CloseRequested => {
                    tracing::trace!("close requested");

                    self.graphics
                        .destroy(window.clone(), event_loop)
                        .expect("Failed to destroy vector graphics interface");

                    if self.config.window.close_on_request {
                        tracing::info!("exiting event loop");
                        event_loop.exit();
                    }
                },

                WindowEvent::RedrawRequested => {
                    window.request_redraw();
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
                        tracing::trace!("keyboard input {event:?}");

                        self.info.keys.push((device_id, event));
                        self.request_redraw();
                    }
                },

                WindowEvent::MouseInput {
                    device_id,
                    button,
                    state,
                } => {
                    tracing::trace!("mouse input {button:?} {state:?}");

                    self.info.buttons.push((device_id, button, state));
                    self.request_redraw();
                },

                WindowEvent::MouseWheel { delta, .. } => {
                    tracing::trace!("mouse wheel {delta:?}");
                    self.info.mouse_scroll_delta = Some(delta);
                    self.request_redraw();
                },

                WindowEvent::Destroyed => tracing::info!("window destroyed"),

                _ => (),
            }
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        tracing::trace!("destroying vector graphics interface");
        let window = self.window.clone().unwrap();
        self.graphics
            .uninit(window, event_loop)
            .expect("Failed to destroy vector graphics interface");

        tracing::trace!("destroying window");
        self.window = None;

        tracing::trace!("running plugin suspensions");
        self.plugins.run(|pl| {
            pl.on_suspended(
                &mut self.config,
                &mut self.scene,
                &mut self.taffy,
                self.window_node,
                &mut self.info,
                &self.update,
                &mut self.last_update,
                event_loop,
            )
        });

        self.info.reset();
    }
}
