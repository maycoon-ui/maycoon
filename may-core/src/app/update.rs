pub struct Update(u8);

impl Update {
    pub const DRAW: Update = Update(1 << 0);
    pub const LAYOUT: Update = Update(1 << 1);
    pub const FORCE: Update = Update(1 << 2);

    pub fn empty() -> Self {
        Update(0)
    }

    pub fn set(&mut self, flag: Update) {
        self.0 |= flag.0;
    }

    pub fn unset(&mut self, flag: Update) {
        self.0 &= !flag.0;
    }

    pub fn has_flag(&self, flag: Update) -> bool {
        (self.0 & flag.0) != 0
    }

    pub fn combine(updates: &[Update]) -> Update {
        let mut flags = 0;
        for update in updates {
            flags |= update.0;
        }
        Update(flags)
    }
}

impl std::ops::BitOr for Update {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Update(self.0 | other.0)
    }
}

impl std::ops::BitAnd for Update {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Update(self.0 & other.0)
    }
}

impl std::ops::BitXor for Update {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Update(self.0 ^ other.0)
    }
}

impl std::ops::Not for Update {
    type Output = Self;

    fn not(self) -> Self {
        Update(!self.0)
    }
}
