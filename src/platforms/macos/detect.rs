// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

use crate::Mode;

#[cfg(feature = "sync")]
pub fn detect() -> crate::Mode {
    Mode::from_bool(super::is_dark_mode())
}

#[cfg(not(feature = "sync"))]
pub async fn detect() -> crate::Mode {
    Mode::from_bool(super::is_dark_mode())
}
