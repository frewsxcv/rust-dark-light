//! Detect if dark mode or light mode is enabled.
//!
//! # Examples
//!
//! ```
//! let mode = dark_light::detect();
//! 
//! match mode {
//!     dark_light::Mode::Dark => {},
//!     dark_light::Mode::Light => {},
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

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd"
))]
mod freedesktop;
#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use freedesktop as platform;

#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd"
)))]
mod platform {
    pub fn detect() -> crate::Mode {
        super::Mode::Light
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    Dark,
    Light,
}

impl Mode {
    fn from(b: bool) -> Self {
        if b {
            Mode::Dark
        } else {
            Mode::Light
        }
    }
    fn rgb(r: u32, g: u32, b: u32) -> Self {
        let window_background_gray = (r * 11 + g * 16 + b * 5) / 32;
        if window_background_gray < 192 {
            Self::Dark
        } else {
            Self::Light
        }
    }
}

/// Detect if light mode or dark mode is enabled. If the mode can’t be detected, fall back to [`Mode::Light`].
pub fn detect() -> Mode {
    platform::detect()
}
