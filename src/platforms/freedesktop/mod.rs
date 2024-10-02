use std::str::FromStr;

use ini::Ini;

use crate::{utils::rgb::Rgb, Mode};

pub mod detect;
pub mod notify;

const MATE: &str = "/org/mate/desktop/interface/gtk-theme";
const GNOME: &str = "/org/gnome/desktop/interface/gtk-theme";
const CINNAMON: &str = "/org/cinnamon/desktop/interface/gtk-theme";

fn dconf_detect(path: &str) -> Mode {
    match dconf_rs::get_string(path) {
        Ok(theme) => {
            if theme.to_lowercase().contains("dark") {
                Mode::Dark
            } else {
                Mode::Light
            }
        }
        Err(_) => Mode::NoPreference,
    }
}

fn kde_detect() -> Mode {
    fn kde_detect() -> Option<Mode> {
        let xdg = xdg::BaseDirectories::new().ok()?;
        let path = xdg.find_config_file("kdeglobals")?;
        let cfg = Ini::load_from_file(path).ok()?;
        let properties = cfg.section(Some("Colors:Window"))?;
        let background = properties.get("BackgroundNormal")?;
        let rgb = Rgb::from_str(background).ok()?;
        Some(Mode::from_rgb(rgb))
    }
    kde_detect().unwrap_or_default()
}

#[cfg(feature = "zbus")]
impl From<ashpd::desktop::settings::ColorScheme> for Mode {
    fn from(value: ashpd::desktop::settings::ColorScheme) -> Self {
        match value {
            ashpd::desktop::settings::ColorScheme::NoPreference => Mode::NoPreference,
            ashpd::desktop::settings::ColorScheme::PreferDark => Mode::Dark,
            ashpd::desktop::settings::ColorScheme::PreferLight => Mode::Light,
        }
    }
}
