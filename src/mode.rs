/// Enum representing dark mode, light mode, or unspecified.
///
/// If `Mode::Default` is returned, it is expected that the user decides which theme mode to use for their specific use case.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Represents the dark mode option.
    Dark,
    /// Represents the light mode option.
    Light,
    /// Used when the system theme mode can’t be detected.
    Default,
}

impl Mode {
    #[allow(dead_code)]
    /// Creates a `Mode` value from a boolean value.
    pub fn from_bool(b: bool) -> Self {
        if b {
            Mode::Dark
        } else {
            Mode::Light
        }
    }
}
