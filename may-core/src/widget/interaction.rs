use taffy::Layout;
use winit::dpi::PhysicalPosition;
use winit::event::{KeyEvent, Modifiers};

#[derive(Debug, Clone)]
pub struct InteractionInfo {
    pub cursor: Option<PhysicalPosition<f64>>,
    pub keys: Vec<KeyEvent>,
    pub modifiers: Modifiers,
}

impl InteractionInfo {
    pub fn reset(&mut self) {
        self.keys.clear();
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
