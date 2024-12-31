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
    pub fn from_bool(b: bool) -> Self {
        if b {
            Mode::Dark
        } else {
            Mode::Light
        }
    }
}
