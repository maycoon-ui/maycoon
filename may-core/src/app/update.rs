use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Update: u8 {
        const EVAL   = 0b00000001;
        const DRAW   = 0b00000010;
        const LAYOUT = 0b00000100;
        const FORCE  = 0b00001000;
    }
}
