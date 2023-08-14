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
//!     // Unspecified
//!     dark_light::Mode::Default => {},
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

mod rgb;
use rgb::Rgb;

/// Enum representing dark mode, light mode, or unspecified.
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
    /// Convert a boolean to [`Mode`]. `true` is [`Mode::Dark`], `false` is [`Mode::Light`].
    fn from(mode: bool) -> Self {
        if mode {
            Mode::Dark
        } else {
            Mode::Light
        }
    }

    /// Convert an RGB color to [`Mode`]. The color is converted to grayscale, and if the grayscale value is less than 192, [`Mode::Dark`] is returned. Otherwise, [`Mode::Light`] is returned.
    fn from_rgb(rgb: Rgb) -> Self {
        let window_background_gray = (rgb.0 * 11 + rgb.1 * 16 + rgb.2 * 5) / 32;
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

/// Watch for changes in light mode or dark mode. If the mode can’t be detected, fall back to [`Mode::Default`].
pub async fn notify(action: fn(mode: Mode)) -> anyhow::Result<()> {
    platform::notify::notify(action).await
}
