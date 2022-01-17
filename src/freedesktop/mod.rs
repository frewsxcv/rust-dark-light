use anyhow::Context;
use ashpd::desktop::settings::SettingsProxy;

use crate::Mode;

pub mod detect;
pub mod notify;

fn get_freedesktop_color_scheme() -> anyhow::Result<Mode> {
    let connection = ashpd::zbus::Connection::new_session()?;
    let proxy = SettingsProxy::new(&connection)?;
    let color_scheme = *proxy.read("org.freedesktop.appearance", "color-scheme")?
        .downcast_ref::<ashpd::zvariant::Value>().with_context(|| "Failed to downcast OwnedValue to Value")?
        .downcast_ref::<u32>().with_context(|| "Failed to downcast Value to u32")?;
    let mode = match color_scheme {
        1 => Mode::Dark,
        2 => Mode::Light,
        _ => Mode::Default,
    };
    Ok(mode)
}