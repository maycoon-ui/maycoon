use std::path::Path;
use femtovg::{Canvas, FontId};
use femtovg::renderer::OpenGl;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::ExternalError;
use winit::event_loop::ControlFlow;
use winit::monitor::MonitorHandle;
use winit::platform::windows::WindowExtWindows;
use winit::window::{CursorGrabMode, ResizeDirection, Window};

use crate::config::Fullscreen;

pub struct AppContext<'a> {
    pub(crate) window: &'a Window,
    pub(crate) monitor: &'a MonitorHandle,
    pub(crate) commands: Vec<AppCommand>,
    pub(crate) dpi: f64,
    pub(crate) update: bool,
    pub(crate) canvas: &'a Canvas<OpenGl>,
}

impl<'a> AppContext<'_> {
    pub fn is_decorated(&self) -> bool {
        self.window.is_decorated()
    }

    pub fn is_resizable(&self) -> bool {
        self.window.is_resizable()
    }

    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    pub fn fullscreen(&self) -> Option<Fullscreen> {
        self.window.fullscreen()
            .map(|fs| {
                match fs {
                    winit::window::Fullscreen::Exclusive(_) => Fullscreen::Exclusive,
                    winit::window::Fullscreen::Borderless(_) => Fullscreen::Borderless,
                }
            })
    }

    pub fn is_visible(&self) -> Option<bool> {
        self.window.is_visible()
    }

    pub fn is_minimized(&self) -> Option<bool> {
        self.window.is_minimized()
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn dpi(&self) -> f64 {
        self.dpi
    }

    pub fn position(&self) -> Option<PhysicalPosition<i32>> {
        self.window.inner_position().ok()
    }

    pub fn push(&mut self, command: AppCommand) {
        self.commands.push(command);
    }

    pub fn pop(&mut self) -> Option<AppCommand> {
        self.commands.pop()
    }

    pub fn commands(&self) -> &Vec<AppCommand> {
        &self.commands
    }

    pub fn commands_mut(&mut self) -> &mut Vec<AppCommand> {
        &mut self.commands
    }

    pub fn exit(&mut self) {
        self.commands.push(AppCommand::Exit);
    }

    pub fn set_fullscreen(&mut self, fullscreen: Option<Fullscreen>) -> Option<()> {
        if let Some(fs) = fullscreen {
            match fs {
                Fullscreen::Exclusive => {
                    if let Some(mode) = self.monitor.video_modes().next() {
                        self.window.set_fullscreen(
                            Some(
                                winit::window::Fullscreen::Exclusive(mode)
                            )
                        );
                    } else {
                        return None;
                    }
                },
                Fullscreen::Borderless => {
                    self.window.set_fullscreen(
                        Some(
                            winit::window::Fullscreen::Borderless(Some(self.monitor.clone()))
                        )
                    )
                }
            }
        } else {
            self.window.set_fullscreen(None);
        }

        Some(())
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.window.set_visible(visible);
    }

    pub fn set_minimized(&mut self, minimized: bool) {
        self.window.set_minimized(minimized);
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.window.set_maximized(maximized);
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    pub fn set_decorations(&mut self, decorations: bool) {
        self.window.set_decorations(decorations);
    }

    pub fn set_position(&mut self, position: PhysicalPosition<i32>) {
        self.window.set_outer_position(position);
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_min_inner_size(&mut self, size: Option<PhysicalSize<u32>>) {
        self.window.set_min_inner_size(size);
    }

    pub fn set_max_inner_size(&mut self, size: Option<PhysicalSize<u32>>) {
        self.window.set_max_inner_size(size);
    }

    pub fn set_window_level(&mut self, level: winit::window::WindowLevel) {
        self.window.set_window_level(level);
    }

    pub fn drag_window(&mut self) -> Result<(), ExternalError> {
        self.window.drag_window()?;

        Ok(())
    }

    pub fn drag_resize(&mut self, dir: ResizeDirection) -> Result<(), ExternalError> {
        self.window.drag_resize_window(dir)?;

        Ok(())
    }

    pub fn set_skip_taskbar(&mut self, skip: bool) {
        self.window.set_skip_taskbar(skip);
    }

    pub fn set_transparent(&mut self, transparent: bool) {
        self.window.set_transparent(transparent);
    }

    pub fn set_update(&mut self, update: bool) {
        self.update = update;
    }

    pub fn set_blur(&mut self, blur: bool) {
        self.window.set_blur(blur);
    }

    pub fn focus(&mut self) {
        self.window.focus_window();
    }

    pub fn set_cursor_grab_mode(&mut self, mode: CursorGrabMode) -> Result<(), ExternalError> {
        self.window.set_cursor_grab(mode)?;

        Ok(())
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.window.set_cursor_visible(visible);
    }
}

pub enum AppCommand {
    Exit,
    SetControl(ControlFlow),
}
