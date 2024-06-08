/// Defines the update mode of the app.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Update(u8);

impl Update {
    /// (Re-)draw widgets.
    pub const DRAW: Update = Update(1 << 0);
    /// (Re-)layout widgets and compute layout tree. Recommended to use alongside [Update::DRAW].
    pub const LAYOUT: Update = Update(1 << 1);
    /// Force re-evaluation, re-layout and re-draw.
    pub const FORCE: Update = Update(1 << 2);
    /// Re-evaluate update mode, even if no events are being processed.
    pub const EVAL: Update = Update(1 << 3);

    /// Return an empty update mode.
    pub fn empty() -> Self {
        Update(0)
    }

    /// Set the given update flag.
    pub fn set(&mut self, flag: Update) {
        self.0 |= flag.0;
    }

    /// Unset the given update flag.
    pub fn unset(&mut self, flag: Update) {
        self.0 &= !flag.0;
    }

    /// Check if the given flag is enabled.
    pub fn has_flag(&self, flag: Update) -> bool {
        (self.0 & flag.0) != 0
    }

    /// Combine the given update modes.
    pub fn combine(updates: &[Update]) -> Update {
        let mut flags = 0;
        for update in updates {
            flags |= update.0;
        }
        Update(flags)
    }

    /// Enable all update modes.
    pub fn all() -> Update {
        Update::DRAW | Update::LAYOUT | Update::FORCE | Update::EVAL
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
