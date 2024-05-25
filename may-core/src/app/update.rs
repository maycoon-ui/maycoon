#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Update {
    bits: u32,
}

impl Update {
    pub const EVAL: Update = Self { bits: 0b0001 };
    pub const DRAW: Update = Self { bits: 0b0010 };
    pub const LAYOUT: Update = Self { bits: 0b0100 };
    pub const FORCE: Update = Self { bits: 0b1000 };

    pub fn empty() -> Self {
        Update { bits: 0 }
    }

    pub fn all() -> Self {
        Update {
            bits: Self::EVAL.bits() | Self::DRAW.bits() | Self::LAYOUT.bits() | Self::FORCE.bits(),
        }
    }

    pub fn bits(&self) -> u32 {
        self.bits
    }

    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    pub fn remove(&mut self, other: Self) {
        self.bits &= !other.bits;
    }

    pub fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    pub fn intersects(&self, other: Self) -> bool {
        (self.bits & other.bits) != 0
    }

    pub fn from_bits(bits: u32) -> Self {
        Update { bits }
    }
}

// Implementing BitOr for combining flags using the | operator
impl std::ops::BitOr for Update {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Update {
            bits: self.bits | rhs.bits,
        }
    }
}

// Implementing BitAnd for combining flags using the & operator
impl std::ops::BitAnd for Update {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Update {
            bits: self.bits & rhs.bits,
        }
    }
}

// Implementing BitXor for combining flags using the ^ operator
impl std::ops::BitXor for Update {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Update {
            bits: self.bits ^ rhs.bits,
        }
    }
}

// Implementing Not for negating flags using the ! operator
impl std::ops::Not for Update {
    type Output = Self;

    fn not(self) -> Self::Output {
        Update { bits: !self.bits }
    }
}
