use maycoon::core::app::info::AppInfo;
use maycoon::core::app::update::UpdateManager;
use maycoon::core::config::MayConfig;
use maycoon::core::layout::{NodeId, TaffyTree};
use maycoon::core::plugin::{Plugin, PluginManager};
use maycoon::core::vgi::VectorGraphicsInterface;
use maycoon::core::window::{ActiveEventLoop, Window, WindowEvent};
use maycoon::theme::theme::Theme;
use std::sync::Arc;
use std::time::Instant;

pub struct MyPlugin;

impl<'a, T: Theme, V: VectorGraphicsInterface<'a>> Plugin<'a, T, V> for MyPlugin {
    fn name(&self) -> &'static str {
        "my_plugin"
    }

    fn on_register(&mut self, _manager: &mut PluginManager<'a, T, V>) {
        println!("Hello World!");
    }

    fn on_unregister(&mut self, _manager: &mut PluginManager<'a, T, V>) {
        println!("Bye World!");
    }

    fn on_window_event(
        &mut self,
        event: &mut WindowEvent,
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
        if let WindowEvent::DroppedFile(path) = event {
            println!("Dropped file: {}", path.to_string_lossy());
        }
    }
}
