use std::path::Path;
use detect_desktop_environment::DesktopEnvironment;

use anyhow::{Result, Context};
use zbus::{blocking::Connection};
use zvariant::Value;

use crate::Mode;

fn get_appearance() -> Result<Option<Mode>> {
    let conn = Connection::session()?;
    let reply = conn.call_method(Some("org.freedesktop.portal.Desktop"), "/org/freedesktop/portal/desktop", Some("org.freedesktop.portal.Settings"), "Read", &("org.freedesktop.appearance", "color-scheme"))?;
    let theme = reply.body::<Value>()?;
    let theme = theme.downcast::<u32>().with_context(|| "Failed to parse value")?;
    match theme {
        0 | 2 => Ok(Some(Mode::Light)),
        1 =>  Ok(Some(Mode::Dark)),
        _ => Ok(None),
    }
}

fn check_file(pattern: &str, path: &Path) -> Mode {
    if let Ok(content) = std::fs::read_to_string(path) {
        let theme = content.lines().filter(|line| line.contains(pattern)).collect::<String>();
        if theme.to_lowercase().contains("dark") {
            Mode::Dark
        } else {
            Mode::Light
        }
    } else {
        Mode::Light
    }
}

fn check_config_file(pattern: &str, path: &str) -> Mode {
    if let Some(config_dir) = dirs::config_dir() {
        check_file(pattern, &config_dir.join(path))
    } else {
        Mode::Light
    }
}

fn check_dconf(pattern: &str) -> Mode {
    match dconf_rs::get_string(pattern) {
        Ok(theme) => if theme.contains("dark") {
            Mode::Dark
        } else {
            Mode::Light
        },
        Err(_) => Mode::Light,
    }
}


fn is_dark_mode_enabled() -> Result<crate::Mode> {
    let mode = if get_appearance()?.is_some() {
        get_appearance()?.unwrap()
    } else {
        match DesktopEnvironment::detect() {
            DesktopEnvironment::Unknown => Mode::Light,
            DesktopEnvironment::Cinnamon => check_dconf("/org/cinnamon/desktop/interface/gtk-theme"),
            DesktopEnvironment::Gnome => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
            DesktopEnvironment::Kde => check_config_file("Name=", "kdeglobals"),
            DesktopEnvironment::Mate => check_dconf("/org/mate/desktop/interface/gtk-theme"),
            DesktopEnvironment::Unity => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
            DesktopEnvironment::Xfce => check_config_file("name=\"ThemeName\"", "xfce4/xfconf/xfce-perchannel-xml/xsettings.xml"),
            _ => Mode::Light
        }
    };
    Ok(mode)
}

pub fn detect() -> Result<crate::Mode> {
    is_dark_mode_enabled()
}
