use crate::app::context::AppContext;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};

pub trait AppHandler<P> {
    fn on_new_event(&mut self, cause: StartCause, context: AppContext<P>);
    fn on_window_event(&mut self, event: WindowEvent, context: AppContext<P>);
    fn on_device_event(&mut self, id: DeviceId, event: DeviceEvent, context: AppContext<P>);
    fn on_proxy_event(&mut self, event: P, context: AppContext<P>);
    fn on_suspended(&mut self, context: AppContext<P>);
    fn on_resumed(&mut self, context: AppContext<P>);
    fn on_about_to_wait(&mut self, context: AppContext<P>);
    fn on_existing(&mut self, context: AppContext<P>);
    fn on_mem_warn(&mut self, context: AppContext<P>);
}
