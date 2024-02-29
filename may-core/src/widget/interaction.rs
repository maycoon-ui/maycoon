use winit::dpi::PhysicalPosition;
pub use winit::event::ElementState;
use winit::event::{KeyEvent, Modifiers};
pub use winit::keyboard;
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
