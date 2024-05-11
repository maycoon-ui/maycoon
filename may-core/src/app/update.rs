use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Update {
    flags: u8,
}

impl Update {
    pub const DRAW: Update = Update::from(0b0001);
    pub const LAYOUT: Update = Update::from(0b0010);
    pub const FORCE: Update = Update::from(0b0100);
    pub const EVAL: Update = Update::from(0b1000);

    pub fn empty() -> Self {
        Update { flags: 0 }
    }

    pub fn all() -> Self {
        Update {
            flags: (Self::DRAW | Self::LAYOUT | Self::FORCE | Self::EVAL).flags,
        }
    }

    pub fn insert(&mut self, flag: Update) {
        self.flags |= flag.flags;
    }

    pub fn remove(&mut self, flag: Update) {
        self.flags &= !flag.flags;
    }

    pub fn contains(&self, flag: Update) -> bool {
        self.flags & flag.flags != 0
    }

    pub fn any(&self) -> bool {
        self.flags != 0
    }

    pub fn none(&self) -> bool {
        self.flags == 0
    }

    pub fn clear(&mut self) {
        self.flags = 0;
    }

    pub fn toggle(&mut self, flag: Update) {
        self.flags ^= flag.flags;
    }

    pub const fn from(flags: u8) -> Self {
        Update { flags }
    }
}

impl BitOr for Update {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Update {
            flags: self.flags | rhs.flags,
        }
    }
}

impl BitAnd for Update {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Update {
            flags: self.flags & rhs.flags,
        }
    }
}

impl BitXor for Update {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Update {
            flags: self.flags ^ rhs.flags,
        }
    }
}

impl BitAndAssign for Update {
    fn bitand_assign(&mut self, rhs: Self) {
        self.flags &= rhs.flags;
    }
}

impl BitOrAssign for Update {
    fn bitor_assign(&mut self, rhs: Self) {
        self.flags |= rhs.flags;
    }
}
