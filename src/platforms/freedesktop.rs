use std::str::FromStr;

use anyhow::Context;
use ini::Ini;

use crate::{utils::rgb::Rgb, Mode};

pub mod detect;
pub mod notify;

const MATE: &str = "/org/mate/desktop/interface/gtk-theme";
const GNOME: &str = "/org/gnome/desktop/interface/gtk-theme";
const CINNAMON: &str = "/org/cinnamon/desktop/interface/gtk-theme";

fn dconf_detect(path: &str) -> Mode {
    match dconf_rs::get_string(path) {
        Ok(theme) => Mode::from(theme.to_lowercase().contains("dark")),
        Err(_) => Mode::Default,
    }
}

fn kde_detect() -> anyhow::Result<Mode> {
    let xdg = xdg::BaseDirectories::new()?;
    let path = xdg
        .find_config_file("kdeglobals")
        .context("Path not found")?;
    let cfg = Ini::load_from_file(path)?;
    let properties = cfg
        .section(Some("Colors:Window"))
        .context("Failed to get section Colors:Window")?;
    let background = properties
        .get("BackgroundNormal")
        .context("Failed to get BackgroundNormal inside Colors:Window")?;
    let rgb = Rgb::from_str(background)?;
    Ok(Mode::from_rgb(rgb))
}
