pub fn detect_mode() -> crate::Mode {
    if let Some(window) = web_sys::window() {
        let query_result = window.match_media("(prefers-color-scheme: dark)");
        if let Ok(Some(mql)) = query_result {
            return crate::Mode::from_bool(mql.matches());
        }
    }
    crate::Mode::Light
}

#[cfg(any(feature = "sync", doc))]
pub mod sync {
    pub fn detect() -> crate::Mode {
        detect_mode()
    }
}

pub async fn detect() -> crate::Mode {
    detect_mode()
}
