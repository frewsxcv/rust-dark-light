
use ashpd::desktop::settings::{Settings, ColorScheme};
use zbus::export::futures_util::StreamExt;

use crate::{Mode, detect};

pub async fn notify(action: fn(mode: Mode)) -> anyhow::Result<()> {
    if get_freedesktop_color_scheme().await.is_ok() {
        freedesktop_watch(action).await
    } else {
        eprintln!("Unable to start freedesktop proxy, falling back to legacy...");
        non_freedesktop_watch(action).await
    }
}

async fn get_freedesktop_color_scheme() -> anyhow::Result<Mode> {
    let proxy = Settings::new().await?;
    let color_scheme = proxy.color_scheme().await?;
    let mode = match color_scheme {
        ColorScheme::PreferDark => Mode::Dark,
        ColorScheme::PreferLight => Mode::Light,
        ColorScheme::NoPreference => Mode::Default,
    };
    Ok(mode)
}

async fn freedesktop_watch(action: fn(mode: Mode)) -> anyhow::Result<()> {
    let proxy = Settings::new().await?;
    if let Ok(mut color_scheme) = proxy.receive_color_scheme_changed().await {
        while let Some(color_scheme) = color_scheme.next().await {
            let mode = match color_scheme {
                ColorScheme::NoPreference => Mode::Default,
                ColorScheme::PreferDark => Mode::Dark,
                ColorScheme::PreferLight => Mode::Light,
            };
            action(mode);
        }
    }
    Ok(())
}

async fn non_freedesktop_watch(action: fn(mode: Mode)) -> anyhow::Result<()> {
    let mut mode = detect();
    loop {
        let new_mode = detect();
        if mode != new_mode {
            mode = new_mode;
            action(mode);
        }
    }
}