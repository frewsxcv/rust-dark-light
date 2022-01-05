use std::path::Path;
use std::time::Duration;
use dbus::arg;
use detect_desktop_environment::DesktopEnvironment;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;

use anyhow::Result;

use crate::Mode;

fn is_dark_mode_enabled() -> Result<crate::Mode> {
    let mode = if get_appearance().is_some() {
        get_freedesktop_color_scheme()
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

fn get_freedesktop_color_scheme() -> Mode {
    let appearance: Option<arg::PropMap> = get_appearance();
    if appearance.is_none() { 
        Mode::Light
    } else {
        let appearance = appearance.unwrap();
        let theme: Option<&i32> = arg::prop_cast(&appearance, "color-scheme");
        match theme.unwrap() {
            1 => Mode::Dark,
            _ => Mode::Light,
        }
    }
}

fn get_appearance() -> Option<arg::PropMap> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy("org.freedesktop.portal.Desktop", "/org/freedesktop/portal/desktop", Duration::from_millis(5000));
    let data = proxy.get("org.freedesktop.portal.Settings", "org.freedesktop.appearance").ok();
    data
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

pub fn detect() -> Result<crate::Mode> {
    is_dark_mode_enabled()
}
