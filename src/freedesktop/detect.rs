use anyhow::Context;
use detect_desktop_environment::DesktopEnvironment;
use ini::Ini;
use std::path::{Path, PathBuf};

use crate::Mode;

use super::get_freedesktop_color_scheme;

const XDG_KDEGLOBALS: &str = "/etc/xdg/kdeglobals";

fn detect_gtk(pattern: &str) -> Mode {
    match dconf_rs::get_string(pattern) {
        Ok(theme) => Mode::from(theme.to_lowercase().contains("dark")),
        Err(_) => Mode::Default,
    }
}

fn detect_kde(path: &str) -> anyhow::Result<Mode> {
    let cfg = Ini::load_from_file(path)?;
    let section = cfg.section(Some("Colors:Window")).with_context(|| "Failed to get section Colors:Window")?;
    let values = section.get("BackgroundNormal").with_context(|| "Failed to get BackgroundNormal inside Colors:Window")?;
    let rgb = values
        .split(',')
        .map(|s| s.parse::<u32>().unwrap_or(255))
        .collect::<Vec<u32>>();
    let rgb = if rgb.len() >= 3 {
        rgb
    } else {
        vec![255, 255, 255]
    };
    let (r, g, b) = (rgb[0], rgb[1], rgb[2]);
    Ok(Mode::from_rgb(r, g, b))
}

fn legacy_detect() -> anyhow::Result<Mode> {
    let mode = match DesktopEnvironment::detect() {
        DesktopEnvironment::Kde => {
            let path = if Path::new(XDG_KDEGLOBALS).exists() {
                PathBuf::from(XDG_KDEGLOBALS)
            } else {
                dirs::home_dir().unwrap().join(".config/kdeglobals")
            };
            detect_kde(path.to_str().unwrap())?
        }
        DesktopEnvironment::Cinnamon => detect_gtk("/org/cinnamon/desktop/interface/gtk-theme"),
        DesktopEnvironment::Gnome => detect_gtk("/org/gnome/desktop/interface/gtk-theme"),
        DesktopEnvironment::Mate => detect_gtk("/org/mate/desktop/interface/gtk-theme"),
        DesktopEnvironment::Unity => detect_gtk("/org/gnome/desktop/interface/gtk-theme"),
        _ => Mode::Default,
    };
    Ok(mode)
}

pub fn detect() -> Mode {
    match get_freedesktop_color_scheme() {
        Ok(mode) => mode,
        Err(_) => match legacy_detect() {
            Ok(mode) => mode,
            Err(_) => Mode::Default,
        },
    }
}

