use detect_desktop_environment::DesktopEnvironment;

use crate::Mode;

use super::{dconf_detect, kde_detect, CINNAMON, GNOME, MATE};

pub fn detect() -> Mode {
    match DesktopEnvironment::detect() {
        Some(mode) => match mode {
            DesktopEnvironment::Kde => match kde_detect() {
                Ok(mode) => mode,
                Err(_) => Mode::Default,
            },
            DesktopEnvironment::Cinnamon => dconf_detect(CINNAMON),
            DesktopEnvironment::Gnome => dconf_detect(GNOME),
            DesktopEnvironment::Mate => dconf_detect(MATE),
            DesktopEnvironment::Unity => dconf_detect(GNOME),
            _ => Mode::Default,
        },
        None => Mode::Default,
    }
}
