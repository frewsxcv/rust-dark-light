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

mod platforms;
use platforms::platform;

mod utils;

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
}

pub use platform::notify::ThemeWatcher;

/// Detect if light mode or dark mode is enabled. If the mode can’t be detected, fall back to [`Mode::Default`].
pub fn detect() -> Mode {
    platform::detect::detect()
}
