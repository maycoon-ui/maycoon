pub struct Diagnostics {
    pub updates: usize,
    pub updates_per_sec: usize,
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self {
            updates: 0,
            updates_per_sec: 0,
        }
    }
}
