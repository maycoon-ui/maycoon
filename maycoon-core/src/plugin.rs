use crate::app::info::AppInfo;
use crate::app::update::UpdateManager;
use crate::config::MayConfig;
use crate::vgi::VectorGraphicsInterface;
use maycoon_theme::theme::Theme;
use rpds::HashTrieMap;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
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
pub trait Plugin<T: Theme, V: VectorGraphicsInterface>: 'static {
    /// The plugin name.
    ///
    /// Should be unique among the ecosystem.
    fn name(&self) -> &'static str;

    /// Called when the plugin is registered using [PluginManager::register].
    fn on_register(&mut self, _manager: &mut PluginManager<T, V>) {}

    /// Called when the plugin is unregistered using [PluginManager::unregister].
    fn on_unregister(&mut self, _manager: &mut PluginManager<T, V>) {}

    /// Called right before initializing the [AppHandler](crate::app::handler::AppHandler) and running the event loop.
    #[inline(always)]
    fn init(
        &mut self,
        _event_loop: &mut EventLoop<()>,
        _update: &UpdateManager,
        _window: &mut WindowAttributes,
        _config: &mut MayConfig<T, V>,
    ) {
    }

    /// Called when the application is resumed after being suspended or when it's first started.
    ///
    /// Desktop applications typically don't get suspended and this function is only called once,
    /// while mobile apps can be suspended and resumed.
    #[inline(always)]
    fn on_resume(
        &mut self,
        _config: &mut MayConfig<T, V>,
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
    #[inline(always)]
    fn on_update(
        &mut self,
        _config: &mut MayConfig<T, V>,
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
    #[inline(always)]
    fn on_window_event(
        &mut self,
        _event: &mut WindowEvent,
        _config: &mut MayConfig<T, V>,
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
    #[cold]
    fn on_suspended(
        &mut self,
        _config: &mut MayConfig<T, V>,
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
pub struct PluginManager<T: Theme, V: VectorGraphicsInterface> {
    plugins: HashTrieMap<&'static str, Rc<RefCell<dyn Plugin<T, V>>>>,
}

impl<T: Theme, V: VectorGraphicsInterface> PluginManager<T, V> {
    /// Creates a new empty plugin manager.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            plugins: HashTrieMap::new(),
        }
    }

    /// Registers a new plugin and returns itself.
    #[inline(always)]
    pub fn register(mut self, mut plugin: impl Plugin<T, V>) -> Self {
        plugin.on_register(&mut self);

        Self {
            plugins: self
                .plugins
                .insert(plugin.name(), Rc::new(RefCell::new(plugin))),
        }
    }

    /// Unregisters a plugin and returns itself.
    #[cold]
    pub fn unregister(mut self, name: &'static str) -> Self {
        let plugins = self.plugins.clone();

        plugins
            .get(name)
            .expect("Plugin not found")
            .borrow_mut()
            .on_unregister(&mut self);

        Self {
            plugins: self.plugins.remove(name),
        }
    }

    /// Unregisters all plugins.
    #[cold]
    pub fn clear(&mut self) {
        let plugins = self.plugins.clone();

        for (_, pl) in plugins.iter() {
            pl.borrow_mut().on_unregister(self);
        }

        self.plugins = HashTrieMap::new();
    }

    /// Runs a closure on all plugins.
    #[inline(always)]
    pub fn run<F>(&mut self, mut op: F)
    where
        F: FnMut(&mut dyn Plugin<T, V>),
    {
        for (_, plugin) in self.plugins.iter() {
            let mut plugin = plugin.borrow_mut();

            op(plugin.deref_mut());
        }
    }
}

impl<T: Theme, V: VectorGraphicsInterface> Default for PluginManager<T, V> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
