//! Detect if dark mode or light mode is enabled.
//!
//! # Examples
//!
//! ```
//! let mode = dark_light::detect();
//!
//! match mode {
//!     // Dark mode
//!     dark_light::Mode::Dark => {},
//!     // Light mode
//!     dark_light::Mode::Light => {},
<<<<<<< HEAD
//!     // Unspecified
=======
>>>>>>> a9e10e2 (Callback functions)
//!     dark_light::Mode::Default => {},
//! }
//! ```

#[cfg(target_os = "macos")]
mod macos;
use std::sync::mpsc::Sender;

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

#[cfg(target_arch = "wasm32")]
mod websys;
#[cfg(target_arch = "wasm32")]
use websys as platform;

#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_arch = "wasm32"
)))]
mod platform {
    pub fn detect() -> crate::Mode {
        super::Mode::Light
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Dark mode
    Dark,
    /// Light mode
    Light,
    /// Unspecified
    Default,
}

impl Mode {
    fn from(b: bool) -> Self {
        if b {
            Mode::Dark
        } else {
            Mode::Light
        }
    }
    fn from_rgb(r: u32, g: u32, b: u32) -> Self {
        let window_background_gray = (r * 11 + g * 16 + b * 5) / 32;
        if window_background_gray < 192 {
            Self::Dark
        } else {
            Self::Light
        }
    }
}

/// Detect if light mode or dark mode is enabled. If the mode can’t be detected, fall back to [`Mode::Default`].
pub fn detect() -> Mode {
    platform::detect::detect()
}

pub async fn notify(tx: Sender<crate::Mode>) -> anyhow::Result<()> {
    platform::notify::notify(tx).await
}