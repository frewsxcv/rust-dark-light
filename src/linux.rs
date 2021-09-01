use detect_desktop_environment::DesktopEnvironment;

fn is_dark_mode_enabled() -> bool {
    match DesktopEnvironment::detect() {
        DesktopEnvironment::Unknown => {
            false
        }
        DesktopEnvironment::Cinnamon => {
            false
        }
        DesktopEnvironment::Enlightenment => {
            false
        }
        DesktopEnvironment::Gnome => {
            false
        }
        DesktopEnvironment::Kde => {
            if let Ok(content) = std::fs::read_to_string("/home/eduardo/.config/kdeglobals") {
                let theme = content.lines().filter(|line| line.contains("Name=")).collect::<String>();
                if theme.to_lowercase().contains("dark") {
                    true
                } else {
                    false
                }
            }
        }
        DesktopEnvironment::Lxde => {
            false
        }
        DesktopEnvironment::Lxqt => {
            false
        }
        DesktopEnvironment::MacOs => {
            false
        }
        DesktopEnvironment::Mate => {
            false
        }
        DesktopEnvironment::Unity => {
            false
        }
        DesktopEnvironment::Windows => {
            false
        }
        DesktopEnvironment::Xfce => {
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
