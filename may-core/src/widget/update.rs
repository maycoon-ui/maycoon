// NOTE: since a bug/false flag in some editors, we need to impl bitflags

use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Update(u8);

bitflags! {
    impl Update: u8 {
        const DRAW = 0b00000001;
        const LAYOUT = 0b00000010;
        const FORCE = 0b00000100;
        const EVAL = 0b00001000;
    }
}
