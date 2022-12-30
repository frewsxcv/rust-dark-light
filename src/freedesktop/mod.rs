use ashpd::desktop::settings::{SettingsProxy, ColorScheme};

use crate::Mode;

pub mod detect;
pub mod notify;

async fn get_freedesktop_color_scheme() -> anyhow::Result<Mode> {
    let connection = ashpd::zbus::Connection::session().await?;
    let proxy = SettingsProxy::new(&connection).await?;
    let color_scheme = proxy.color_scheme().await?;
    let mode = match color_scheme {
        ColorScheme::PreferDark => Mode::Dark,
        ColorScheme::PreferLight => Mode::Light,
        ColorScheme::NoPreference => Mode::Default,
    };
    Ok(mode)
}