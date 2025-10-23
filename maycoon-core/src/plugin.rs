use crate::app::info::AppInfo;
use crate::app::update::UpdateManager;
use crate::config::MayConfig;
use crate::vgi::VectorGraphicsInterface;
use indexmap::IndexMap;
use maycoon_theme::theme::Theme;
use std::sync::Arc;
use std::time::Instant;
use taffy::{NodeId, TaffyTree};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes};

/// A plugin interface for maycoon applications.
///
/// Plugins are used to extend functionality and manipulate the inner state of applications.
/// Beware that tampering with the application state may cause crashes or other issues if not done correctly.
///
/// All functions defined in this trait get called before the app handler logic and therefore can control the application flow.
pub trait Plugin<'a, T: Theme, V: VectorGraphicsInterface<'a>> {
    /// The plugin name.
    ///
    /// Should be unique among the ecosystem.
    fn name(&self) -> &'static str;

    /// Called when the plugin is registered using [PluginManager::register].
    fn on_register(&mut self, _manager: &mut PluginManager<'a, T, V>) {}

    /// Called when the plugin is unregistered using [PluginManager::unregister].
    fn on_unregister(&mut self, _manager: &mut PluginManager<'a, T, V>) {}

    /// Called right before initializing the [AppHandler](crate::app::handler::AppHandler) and running the event loop.
    fn init(
        &mut self,
        _event_loop: &mut EventLoop<()>,
        _update: &UpdateManager,
        _window: &mut WindowAttributes,
        _config: &mut MayConfig<'a, T, V>,
    ) {
    }

    /// Called when the application is resumed after being suspended or when it's first started.
    ///
    /// Desktop applications typically don't get suspended and this function is only called once,
    /// while mobile apps can be suspended and resumed.
    fn on_resume(
        &mut self,
        _config: &mut MayConfig<'a, T, V>,
        _scene: &mut V::Scene,
        _taffy: &mut TaffyTree,
        _window_node: NodeId,
        _info: &mut AppInfo,
        _update: &UpdateManager,
        _last_update: &mut Instant,
        _event_loop: &ActiveEventLoop,
    ) {
    }

    /// Called right before the application handler tries to update the application
    /// and figure out what updates to apply.
    fn on_update(
        &mut self,
        _config: &mut MayConfig<'a, T, V>,
        _window: &Arc<Window>,
        _scene: &mut V::Scene,
        _taffy: &mut TaffyTree,
        _window_node: NodeId,
        _info: &mut AppInfo,
        _update: &UpdateManager,
        _last_update: &mut Instant,
        _event_loop: &ActiveEventLoop,
    ) {
    }

    /// Called when a window event is received.
    fn on_window_event(
        &mut self,
        _event: &mut WindowEvent,
        _config: &mut MayConfig<'a, T, V>,
        _window: &Arc<Window>,
        _scene: &mut V::Scene,
        _taffy: &mut TaffyTree,
        _window_node: NodeId,
        _info: &mut AppInfo,
        _update: &UpdateManager,
        _last_update: &mut Instant,
        _event_loop: &ActiveEventLoop,
    ) {
    }

    /// Called when the application is suspended.
    fn on_suspended(
        &mut self,
        _config: &mut MayConfig<'a, T, V>,
        _scene: &mut V::Scene,
        _taffy: &mut TaffyTree,
        _window_node: NodeId,
        _info: &mut AppInfo,
        _update: &UpdateManager,
        _last_update: &mut Instant,
        _event_loop: &ActiveEventLoop,
    ) {
    }
}

/// A plugin manager for maycoon applications.
pub struct PluginManager<'a, T: Theme, V: VectorGraphicsInterface<'a>> {
    plugins: IndexMap<&'static str, Box<dyn Plugin<'a, T, V>>>,
}

impl<'a, T: Theme, V: VectorGraphicsInterface<'a>> PluginManager<'a, T, V> {
    /// Creates a new empty plugin manager.
    pub fn new() -> Self {
        Self {
            plugins: IndexMap::new(),
        }
    }

    /// Registers a new plugin.
    pub fn register(&mut self, mut plugin: impl Plugin<'a, T, V> + 'static) {
        plugin.on_register(self);

        self.plugins.insert(plugin.name(), Box::new(plugin));
    }

    /// Unregisters a plugin.
    pub fn unregister(&mut self, name: &'static str) {
        let mut pl = self.plugins.swap_remove(name).expect("Plugin not found");

        pl.on_unregister(self);
    }

    /// Unregisters all plugins.
    pub fn clear(&mut self) {
        let plugins = self.plugins.keys().cloned().collect::<Vec<_>>();

        for name in plugins {
            self.unregister(name);
        }
    }

    /// Runs a closure on all plugins.
    pub fn run<F>(&mut self, mut op: F)
    where
        F: FnMut(&mut Box<dyn Plugin<'a, T, V>>),
    {
        for pl in self.plugins.values_mut() {
            op(pl);
        }
    }
}

impl<'a, T: Theme, V: VectorGraphicsInterface<'a>> Default for PluginManager<'a, T, V> {
    fn default() -> Self {
        Self::new()
    }
}
