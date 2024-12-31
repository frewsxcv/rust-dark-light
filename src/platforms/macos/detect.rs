// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

#[cfg(any(feature = "sync", doc))]
pub mod sync {
    pub fn detect() -> crate::Mode {
        crate::Mode::from_bool(super::super::is_dark_mode())
    }
}

pub async fn detect() -> crate::Mode {
    crate::Mode::from_bool(super::is_dark_mode())
}
