use std::process::Command;

use detect_desktop_environment::DesktopEnvironment;

use crate::Mode;

use super::{dconf_detect, kde_detect, CINNAMON, GNOME, MATE};

pub fn detect() -> Mode {
    NonFreeDesktop::detect()
        .concrete()
        .unwrap_or_else(FreeDesktop::detect)
}

/// Detects the color scheme on a platform.
trait ColorScheme {
    fn detect() -> Mode;
}

/// Represents the FreeDesktop platform.
struct FreeDesktop;

/// Represents non FreeDesktop platforms.
struct NonFreeDesktop;

/// Detects the color scheme on FreeDesktop platforms. It makes use of the DBus interface.
impl ColorScheme for FreeDesktop {
    fn detect() -> Mode {
        let Ok(output) = Command::new("dbus-send")
            .args([
                "--print-reply=literal",
                "--dest=org.freedesktop.portal.Desktop",
                "/org/freedesktop/portal/desktop",
                "org.freedesktop.portal.Settings.Read",
                "string:org.freedesktop.appearance",
                "string:color-scheme",
            ])
            .output()
            .map(|output| output.stdout)
        else {
            return Mode::NoPreference;
        };
        const PREFIX: &[u8] = b"uint32 ";
        if let Some(index) = output
            .windows(PREFIX.len())
            .position(|bytes| bytes == PREFIX)
        {
            match output.get(index + PREFIX.len()) {
                Some(b'0') => Mode::NoPreference,
                Some(b'1') => Mode::Dark,
                Some(b'2') => Mode::Light,
                _ => Mode::NoPreference,
            }
        } else {
            Mode::NoPreference
        }
    }
}

/// Detects the color scheme on non FreeDesktop platforms, having a custom implementation for each desktop environment.
impl ColorScheme for NonFreeDesktop {
    fn detect() -> Mode {
        match DesktopEnvironment::detect() {
            Some(mode) => match mode {
                DesktopEnvironment::Kde => kde_detect(),
                DesktopEnvironment::Cinnamon => dconf_detect(CINNAMON),
                DesktopEnvironment::Gnome => dconf_detect(GNOME),
                DesktopEnvironment::Mate => dconf_detect(MATE),
                DesktopEnvironment::Unity => dconf_detect(GNOME),
                _ => Mode::NoPreference,
            },
            None => Mode::NoPreference,
        }
    }
}
