use anyhow::Context;
use detect_desktop_environment::DesktopEnvironment;
use ini::Ini;

use crate::Mode;

use super::{CINNAMON, GNOME, MATE};

pub fn detect() -> Mode {
    match legacy_detect() {
        Ok(mode) => mode,
        Err(_) => Mode::Default,
    }
}

fn legacy_detect() -> anyhow::Result<Mode> {
    let mode = match DesktopEnvironment::detect() {
        DesktopEnvironment::Kde => kde_detect()?,
        DesktopEnvironment::Cinnamon => dconf_detect(CINNAMON),
        DesktopEnvironment::Gnome => dconf_detect(GNOME),
        DesktopEnvironment::Mate => dconf_detect(MATE),
        DesktopEnvironment::Unity => dconf_detect(GNOME),
        _ => Mode::Default,
    };
    Ok(mode)
}

fn dconf_detect(path: &str) -> Mode {
    match dconf_rs::get_string(path) {
        Ok(theme) => Mode::from(Some(theme.to_lowercase().contains("dark"))),
        Err(_) => Mode::Default,
    }
}

fn kde_detect() -> anyhow::Result<Mode> {
    let xdg = xdg::BaseDirectories::new()?;
    let path = xdg.find_config_file("kdeglobals")
        .context("Path not found")?;
    let cfg = Ini::load_from_file(path)?;
    let properties = cfg.section(Some("Colors:Window"))
        .context("Failed to get section Colors:Window")?;
    let background = properties.get("BackgroundNormal")
        .context("Failed to get BackgroundNormal inside Colors:Window")?;
    let rgb = rgb_from_string(background)?;
    Ok(Mode::from_rgb(&rgb))
}

fn rgb_from_string(rgb: &str) -> anyhow::Result<Vec<u32>> {
    rgb.split(',')
        .map(|s| s.parse::<u32>().unwrap_or_else(|_| 255))
        .try_fold(vec![255, 255, 255], |mut acc, x| {
            if acc.len() < 3 {
                acc.push(x);
                Ok(acc)
            } else {
                Err(anyhow::anyhow!("Too many elements"))
            }
        })
}