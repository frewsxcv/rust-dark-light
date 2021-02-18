//! Detect if dark mode or light mode is enabled.
//!
//! # Examples
//!
//! ```
//! fn main() {
//!     let mode = dark_light::detect();
//! }
//! ```

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as platform;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod platform {
    pub fn detect() -> crate::Mode {
        Mode::Light
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    Dark,
    Light,
}

/// Detect if light mode or dark mode is enabled. If the mode can’t be detected, fall back to [`Mode::Light`].
pub fn detect() -> Mode {
    platform::detect()
}
