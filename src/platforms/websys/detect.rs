use crate::Mode;

pub fn detect_mode() -> crate::Mode {
    if let Some(window) = web_sys::window() {
        let query_result = window.match_media("(prefers-color-scheme: dark)");
        if let Ok(Some(mql)) = query_result {
            return Mode::from_bool(mql.matches());
        }
    }
    Mode::Light
}

#[cfg(feature = "sync")]
pub fn detect() -> crate::Mode {
    detect_mode()
}

#[cfg(not(feature = "sync"))]
pub async fn detect() -> crate::Mode {
    detect_mode()
}