use winreg::RegKey;

fn is_dark_mode_enabled() -> bool {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    if let Ok(subkey) =
        hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
    {
        if let Ok(dword) = subkey.get_value::<u32, _>("AppsUseLightTheme") {
            (dword == 0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn detect() -> crate::Mode {
    if is_dark_mode_enabled() {
        crate::Mode::Dark
    } else {
        crate::Mode::Light
    }
}
