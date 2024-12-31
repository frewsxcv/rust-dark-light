//! This crate is designed to facilitate the development of applications that support both light and dark themes. It provides a simple API to detect the current theme mode and subscribe to changes in the system theme mode.
//!
//! It supports macOS, Windows, Linux, BSDs, and WASM.
//!
//! On Linux the [XDG Desktop Portal](https://flatpak.github.io/xdg-desktop-portal/) D-Bus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access.
//! Features:
//! - [`sync`] - Enables the synchronous API.
mod mode;
mod platforms;

pub use mode::Mode;

/// Detects the system theme mode. If the mode can’t be detected, it fallbacks to [`Mode::Default`].
///
/// # Example
///
/// ``` no_run
/// use dark_light::Mode;
///
/// #[tokio::main]
/// async fn main() {
///     let mode = dark_light::detect().await;
///
///     match mode {
///         Mode::Dark => {},
///         Mode::Light => {},
///         Mode::Default => {},
///     }
/// }
/// ```
pub use platforms::platform::detect::detect;

/// Notifies the user if the system theme has been changed.
///
/// This function returns a stream of `Mode` values. The stream will emit a new value whenever the system theme changes.
///
/// # Example
///
/// ``` no_run
/// use dark_light::Mode;
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let mut stream = dark_light::subscribe().await;
///
///     while let Some(mode) = stream.next().await {
///         match mode {
///             Mode::Dark => {},
///             Mode::Light => {},
///             Mode::Default => {},
///         }
///     }
/// }
/// ```
pub use platforms::platform::subscribe::subscribe;

#[cfg(any(feature = "sync", doc))]
/// The synchronous API of this crate.
///
/// If you are upgrading this crate, you may update your code to use the async API or alternatively, you can enable the `sync` feature to use this module.
pub mod sync {
    /// Detects the system theme mode. If the mode can’t be detected, it fallbacks to [`Mode::Default`](crate::Mode::Default).
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use dark_light::Mode;
    ///     
    /// let mode = dark_light::sync::detect();
    ///
    /// match mode {
    ///     Mode::Dark => {},
    ///     Mode::Light => {},
    ///     Mode::Default => {},
    /// }
    /// ```
    pub use super::platforms::platform::detect::sync::detect;

    /// Notifies the user if the system theme has been changed.
    ///
    /// This function returns a stream of `Mode` values. The stream will emit a new value whenever the system theme changes.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use dark_light::Mode;
    ///
    /// let stream = dark_light::sync::subscribe();
    /// while let Ok(mode) = stream.recv() {
    ///     match mode {
    ///         Mode::Dark => {},
    ///         Mode::Light => {},
    ///         Mode::Default => {},
    ///     }
    /// }
    /// ```
    pub use super::platforms::platform::subscribe::sync::subscribe;
}
