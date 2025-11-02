/// Contains [Instant](Instant) and [SystemTime](SystemTime) types for the web (using [web_time]).
#[cfg(web)]
pub mod time {
    pub use web_time::Instant;
    pub use web_time::SystemTime;
    pub use web_time::SystemTimeError;
}

/// Contains native [Instant] and [SystemTime] types.
#[cfg(native)]
pub mod time {
    pub use std::time::Instant;
    pub use std::time::SystemTime;
    pub use std::time::SystemTimeError;
}
