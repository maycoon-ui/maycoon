use winit::dpi::PhysicalPosition;
use winit::event::{KeyEvent, Modifiers};
pub use winit::event::ElementState;
pub use winit::keyboard as keyboard;
use winit::keyboard::Key;

#[derive(Debug)]
pub enum Action<'a> {
    Hover,
    Key(&'a Key, &'a ElementState),
    Modifiers(&'a Modifiers),
}

pub struct InteractionInfo {
    pub cursor: Option<PhysicalPosition<f64>>,
    pub keys: Vec<KeyEvent>,
    pub modifiers: Modifiers,
}
