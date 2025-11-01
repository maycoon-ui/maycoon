#[cfg(target_arch = "wasm32")]
pub mod time {
    pub use web_time::Instant;
    pub use web_time::SystemTime;
    pub use web_time::SystemTimeError;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod time {
    pub use std::time::Instant;
    pub use std::time::SystemTime;
    pub use std::time::SystemTimeError;
}
