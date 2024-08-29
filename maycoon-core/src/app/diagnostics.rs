/// Contains diagnostics data for the application.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Diagnostics {
    /// The updates since the last second. Use `updates_per_sec` for the average updates per second.
    pub updates: usize,
    /// The average updates per second.
    pub updates_per_sec: usize,
}
