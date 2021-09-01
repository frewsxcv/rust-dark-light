use detect_desktop_environment::DesktopEnvironment;

fn is_dark_mode_enabled() -> bool {
    match DesktopEnvironment::detect() {
        DesktopEnvironment::Unknown => false,
        DesktopEnvironment::Cinnamon => check_dconf("/org/cinnamon/desktop/interface/gtk-theme"),
        DesktopEnvironment::Enlightenment => false,
        DesktopEnvironment::Gnome => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
        DesktopEnvironment::Kde => check_file("Name=", "/home/eduardo/.config/kdeglobals"),
        DesktopEnvironment::Lxde => false,
        DesktopEnvironment::Lxqt => false,
        DesktopEnvironment::MacOs => false,
        DesktopEnvironment::Mate => check_dconf("/org/mate/desktop/interface/gtk-theme"),
        DesktopEnvironment::Unity => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
        DesktopEnvironment::Windows => false,
        DesktopEnvironment::Xfce => check_file("name=\"ThemeName\"", "/home/eduardo/.config/xfce4/xfconf/xfce-perchannel-xml/xsettings.xml")
    }
}

fn check_file(pattern: &str, path: &str) -> bool {
    if let Ok(content) = std::fs::read_to_string(path) {
        let theme = content.lines().filter(|line| line.contains(pattern)).collect::<String>();
        if theme.to_lowercase().contains("dark") {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn check_dconf(pattern: &str) -> bool {
    let theme = match dconf_rs::get_string(pattern) {
        Ok(theme) => theme,
        Err(_) => String::new()
    };
    if theme.contains("dark") {
        true
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
