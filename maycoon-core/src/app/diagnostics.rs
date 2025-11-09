/// Contains diagnostics data for the application.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Diagnostics {
    /// The updates since the last second. Use `updates_per_sec` for the average updates per second.
    pub updates: usize,
    /// The average updates per second.
    pub updates_per_sec: usize,
    /// Whether this is the first run of the application.
    pub first_run: bool,
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self {
            updates: 0,
            updates_per_sec: 0,
            first_run: true,
        }
    }
}
