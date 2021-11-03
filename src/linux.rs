use std::path::Path;

use detect_desktop_environment::DesktopEnvironment;

fn is_dark_mode_enabled() -> bool {
    match DesktopEnvironment::detect() {
        DesktopEnvironment::Unknown => false,
        DesktopEnvironment::Cinnamon => check_dconf("/org/cinnamon/desktop/interface/gtk-theme"),
        DesktopEnvironment::Enlightenment => false,
        DesktopEnvironment::Gnome => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
        DesktopEnvironment::Kde => check_config_file("Name=", "kdeglobals"),
        DesktopEnvironment::Lxde => false,
        DesktopEnvironment::Lxqt => false,
        DesktopEnvironment::MacOs => false,
        DesktopEnvironment::Mate => check_dconf("/org/mate/desktop/interface/gtk-theme"),
        DesktopEnvironment::Unity => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
        DesktopEnvironment::Windows => false,
        DesktopEnvironment::Xfce => check_config_file("name=\"ThemeName\"", "xfce4/xfconf/xfce-perchannel-xml/xsettings.xml")
    }
}

fn check_file(pattern: &str, path: &Path) -> bool {
    if let Ok(content) = std::fs::read_to_string(path) {
        let theme = content.lines().filter(|line| line.contains(pattern)).collect::<String>();
        theme.to_lowercase().contains("dark")
    } else {
        false
    }
}

fn check_config_file(pattern: &str, path: &str) -> bool {
    if let Some(config_dir) = dirs::config_dir() {
        check_file(pattern, &config_dir.join(path))
    } else {
        false
    }
}

fn check_dconf(pattern: &str) -> bool {
    match dconf_rs::get_string(pattern) {
        Ok(theme) => theme.contains("dark"),
        Err(_) => false,
    }
}

pub fn detect() -> crate::Mode {
    if is_dark_mode_enabled() {
        crate::Mode::Dark
    } else {
        crate::Mode::Light
    }
}
