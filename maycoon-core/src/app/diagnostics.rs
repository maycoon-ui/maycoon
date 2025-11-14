use crate::platform::time::Instant;

/// Contains diagnostics data for the application.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Diagnostics {
    /// The last time the diagnostics got updated. Resets every second.
    pub last_tick: Instant,

    /// The updates since the last second. Use `updates_per_sec` for the average updates per second.
    pub updates: usize,
    /// The average updates per second.
    pub updates_per_sec: usize,

    /// The frames since the last second. Use `frames_per_sec` for the average frames per second.
    pub frames: usize,
    /// The average frames per second.
    pub frames_per_sec: usize,

    /// Whether this is the first run of the application.
    pub first_run: bool,
}

impl Diagnostics {
    /// Ticks the diagnostics.
    ///
    /// This computes frames and updates per second
    /// and is called every frame by the event loop.
    #[inline(always)]
    pub fn tick(&mut self) {
        if self.last_tick.elapsed().as_secs() >= 1 {
            self.last_tick = Instant::now();

            self.frames_per_sec = (self.frames_per_sec + self.frames) / 2;
            self.updates_per_sec = (self.updates_per_sec + self.updates) / 2;

            self.updates = 0;
            self.frames = 0;
        }
    }

    /// Increases the update count.
    #[inline(always)]
    pub fn do_update(&mut self) {
        self.updates += 1;
    }

    /// Increases the frame count.
    #[inline(always)]
    pub fn do_frame(&mut self) {
        self.frames += 1;
    }
}

impl Default for Diagnostics {
    #[inline(always)]
    fn default() -> Self {
        Self {
            last_tick: Instant::now(),
            updates: 0,
            updates_per_sec: 0,
            frames: 0,
            frames_per_sec: 0,
            first_run: true,
        }
    }
}
