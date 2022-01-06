use detect_desktop_environment::DesktopEnvironment;
use std::path::Path;

use zbus::blocking::Connection;
use zvariant::Value;

use crate::Mode;

fn get_freedesktop_color_scheme() -> Option<Mode> {
    let conn = Connection::session();
    if let Err(_) = conn {
        return None;
    }
    let reply = conn.unwrap().call_method(
        Some("org.freedesktop.portal.Desktop"),
        "/org/freedesktop/portal/desktop",
        Some("org.freedesktop.portal.Settings"),
        "Read",
        &("org.freedesktop.appearance", "color-scheme"),
    );
    if let Ok(reply) = &reply {
        let theme = reply.body::<Value>();
        if let Err(_) = theme {
            return None;
        }
        let theme = theme.unwrap().downcast::<u32>();
        match theme.unwrap() {
            1 => Some(Mode::Dark),
            2 => Some(Mode::Light),
            _ => None,
        }
    } else {
        return None;
    }
}

fn check_file(pattern: &str, path: &Path) -> Mode {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let theme = content
                .lines()
                .filter(|line| line.contains(pattern))
                .collect::<String>();
            Mode::from(theme.to_lowercase().contains("dark"))
        }
        Err(_) => Mode::Light,
    }
}

fn check_config_file(pattern: &str, path: &str) -> Mode {
    match dirs::config_dir() {
        Some(config_dir) => check_file(pattern, &config_dir.join(path)),
        None => Mode::Light,
    }
}

fn check_dconf(pattern: &str) -> Mode {
    match dconf_rs::get_string(pattern) {
        Ok(theme) => {
            if theme.to_lowercase().contains("dark") {
                Mode::Dark
            } else {
                Mode::Light
            }
        }
        Err(_) => Mode::Light,
    }
}

pub fn detect() -> Mode {
    match get_freedesktop_color_scheme() {
        Some(mode) => mode,
        None => match DesktopEnvironment::detect() {
            DesktopEnvironment::Cinnamon => {
                check_dconf("/org/cinnamon/desktop/interface/gtk-theme")
            }
            DesktopEnvironment::Gnome => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
            DesktopEnvironment::Kde => check_config_file("Name=", "kdeglobals"),
            DesktopEnvironment::Mate => check_dconf("/org/mate/desktop/interface/gtk-theme"),
            DesktopEnvironment::Unity => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
            DesktopEnvironment::Xfce => check_config_file(
                "name=\"ThemeName\"",
                "xfce4/xfconf/xfce-perchannel-xml/xsettings.xml",
            ),
            _ => Mode::Light,
        },
    }
}
