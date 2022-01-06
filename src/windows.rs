use crate::Mode;
use winreg::RegKey;

pub fn detect() -> Mode {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    if let Ok(subkey) =
        hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
    {
        if let Ok(dword) = subkey.get_value::<u32, _>("AppsUseLightTheme") {
            Mode::from(dword == 0)
        } else {
            Mode::Light
        }
    } else {
        Mode::Light
    }
}
