#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Update(u8);

impl Update {
    pub const DRAW: Update = Update(0b00000001);
    pub const LAYOUT: Update = Update(0b00000010);
    pub const FORCE: Update = Update(0b00000100);
    pub const EVAL: Update = Update(0b00001000);

    pub fn empty() -> Self {
        Update(0)
    }

    pub fn all() -> Self {
        Update(Self::DRAW.0 | Self::LAYOUT.0 | Self::FORCE.0 | Self::EVAL.0)
    }

    pub fn contains(&self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0;
    }

    pub fn remove(&mut self, flag: Self) {
        self.0 &= !flag.0;
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn toggle(&mut self, flag: Self) {
        self.0 ^= flag.0;
    }

    pub fn set(&mut self, flag: Self, value: bool) {
        if value {
            self.insert(flag);
        } else {
            self.remove(flag);
        }
    }

    pub fn is_all(&self) -> bool {
        self.0 == Self::all().0
    }

    pub fn from_bits_truncate(bits: u8) -> Self {
        Update(bits & (Self::DRAW.0 | Self::LAYOUT.0 | Self::FORCE.0 | Self::EVAL.0))
    }

    pub fn from_bits(bits: u8) -> Self {
        Update(bits)
    }

    pub fn bits(&self) -> u8 {
        self.0
    }
}
