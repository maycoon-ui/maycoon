use winit::event_loop::EventLoopWindowTarget;
use winit::monitor::{MonitorHandle, VideoMode};

pub fn get_monitor(elwt: &EventLoopWindowTarget<()>) -> Option<MonitorHandle> {
    if let Some(monitor) = elwt.primary_monitor() {
        Some(monitor)
    } else {
        elwt.available_monitors().next()
    }
}

pub fn get_video_mode(elwt: &EventLoopWindowTarget<()>) -> Option<VideoMode> {
    get_monitor(elwt)?.video_modes().next()
}
