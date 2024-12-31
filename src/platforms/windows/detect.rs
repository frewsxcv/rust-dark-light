use crate::Mode;
use winreg::RegKey;

const SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
const VALUE: &str = "AppsUseLightTheme";

pub fn detect_mode() -> Mode {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    if let Ok(subkey) = hkcu.open_subkey(SUBKEY) {
        if let Ok(dword) = subkey.get_value::<u32, _>(VALUE) {
            return Mode::from_bool(dword == 0);
        }
    }
    Mode::Light
}

#[cfg(any(feature = "sync", doc))]
pub mod sync {
    pub fn detect() -> crate::Mode {
        super::detect_mode()
    }
}

pub async fn detect() -> crate::Mode {
    detect_mode()
}
