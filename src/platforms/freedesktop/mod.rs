pub mod detect;
pub mod subscribe;

use crate::Mode;

use ashpd::desktop::settings::ColorScheme as PortalColorScheme;
use ashpd::desktop::settings::Settings as XdgPortalSettings;

impl From<PortalColorScheme> for Mode {
    fn from(value: PortalColorScheme) -> Self {
        match value {
            PortalColorScheme::NoPreference => Mode::Default,
            PortalColorScheme::PreferDark => Mode::Dark,
            PortalColorScheme::PreferLight => Mode::Light,
        }
    }
}

pub(crate) async fn get_color_scheme() -> Mode {
    let Ok(settings) = XdgPortalSettings::new().await else {
        log::error!("Failed to create a new XDG Desktop Portal settings instance.");
        return Mode::Default;
    };

    let Ok(color_scheme) = settings.color_scheme().await else {
        log::error!("Failed to get the current color scheme from XDG Desktop Portal, defaulting to Mode::Default.");
        return Mode::Default;
    };

    color_scheme.into()
}
