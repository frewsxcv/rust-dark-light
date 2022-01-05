use crate::Mode;
use anyhow::Result;
use winreg::RegKey;

pub fn detect() -> Result<Mode> {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let mode = if let Ok(subkey) =
        hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
    {
        if let Ok(dword) = subkey.get_value::<u32, _>("AppsUseLightTheme") {
            if dword == 0 {
                Mode::Dark
            } else {
                Mode::Light
            }
        } else {
            Mode::Light
        }
    } else {
        Mode::Light
    };
    Ok(mode)
}
