//! Detect if dark mode or light mode is enabled.
//!
//! # Examples
//!
//! ```no-exec
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
    #[allow(dead_code)]
    fn from_bool(b: bool) -> Self {
        if b {
            Mode::Dark
        } else {
            Mode::Light
        }
    }
}

/// Detect if light mode or dark mode is enabled. If the mode can’t be detected, fall back to [`Mode::Default`].
pub use platform::detect::detect;
/// Notifies the user if the system theme has been changed.
pub use platform::notify::subscribe;
