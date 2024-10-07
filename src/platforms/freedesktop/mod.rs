pub mod detect;
pub mod notify;

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

pub(crate) async fn initial_value() -> Mode {
    XdgPortalSettings::new()
        .await
        .unwrap()
        .color_scheme()
        .await
        .unwrap()
        .into()
}
