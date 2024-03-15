use taffy::Layout;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, Modifiers, MouseButton};
use winit::keyboard::{Key, KeyCode};

#[derive(Debug, Clone)]
pub struct InteractionInfo {
    pub cursor: Option<PhysicalPosition<f64>>,
    pub keys: Vec<KeyEvent>,
    pub modifiers: Modifiers,
    pub mouse_buttons: Vec<(ElementState, MouseButton)>,
}

impl InteractionInfo {
    pub fn reset(&mut self) {
        self.keys.clear();
        self.mouse_buttons.clear();
    }
}

pub fn cursor_hovering_over(info: &InteractionInfo, layout: &Layout) -> bool {
    if let Some(cursor) = info.cursor {
        if cursor.x as f32 >= layout.location.x
            && cursor.y as f32 >= layout.location.y
            && cursor.x as f32 <= layout.location.x + layout.size.width
            && cursor.y as f32 <= layout.location.y + layout.size.height
        {
            return true;
        }
    }

    return false;
}

pub fn get_keys_while_hovering(info: &InteractionInfo, layout: &Layout) -> Vec<KeyEvent> {
    return if cursor_hovering_over(info, layout) {
        info.keys.clone()
    } else {
        Vec::new()
    };
}

pub fn is_mouse_button_pressed(info: &InteractionInfo, button: MouseButton, state: ElementState) -> bool {
    info.mouse_buttons
        .iter()
        .any(|mouse_button| mouse_button.0 == state && mouse_button.1 == button)
}

pub fn is_key_pressed(info: &InteractionInfo, key: Key, state: ElementState) -> bool {
    info.keys.iter().any(|key_event| key_event.logical_key == key && key_event.state == state)
}

pub fn is_key_pressed_with(info: &InteractionInfo, key: Key, modifiers: Modifiers, state: ElementState) -> bool {
    info.keys
        .iter()
        .any(|key_event| {
            key_event.logical_key == key && info.modifiers == modifiers && key_event.state == state
        })
}
