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

mod mode;
mod platforms;

pub use mode::Mode;

/// Detect if light mode or dark mode is enabled. If the mode can’t be detected, fall back to [`Mode::Default`].
pub use platforms::platform::detect::detect;
/// Notifies the user if the system theme has been changed.
pub use platforms::platform::notify::subscribe;
