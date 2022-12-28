use ashpd::desktop::settings::SettingsProxy;

use crate::Mode;

pub mod detect;
pub mod notify;

async fn get_freedesktop_color_scheme() -> anyhow::Result<Mode> {
    let mode = fetch_color_scheme_from_proxy().await?;
    Ok(mode)
}

async fn fetch_color_scheme_from_proxy() -> anyhow::Result<Mode> {
    let connection = ashpd::zbus::Connection::session().await?;
    let proxy = SettingsProxy::new(&connection).await?;
    let color_scheme = proxy.read::<i32>("org.freedesktop.appearance", "color-scheme").await?;
    let mode = match color_scheme {
        1 => Mode::Dark,
        2 => Mode::Light,
        _ => Mode::Default,
    };
    Ok(mode)
}