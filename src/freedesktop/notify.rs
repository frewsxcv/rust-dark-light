
use tokio::sync::mpsc::Sender;
use ashpd::desktop::settings::{Settings, ColorScheme};

use crate::{Mode, detect};

use super::get_freedesktop_color_scheme;

pub async fn notify(tx: Sender<crate::Mode>) -> anyhow::Result<()> {
    if get_freedesktop_color_scheme().await.is_ok() {
        tokio::spawn(freedesktop_watch(tx));
    } else {
        eprintln!("Unable to start proxy, falling back to legacy...");
        tokio::spawn(non_freedesktop_watch(tx));
    }
    Ok(())
}

async fn freedesktop_watch(tx: Sender<Mode>) -> anyhow::Result<()> {
    let proxy = Settings::new().await?;
    while let Ok(color_scheme) = proxy.receive_color_scheme_changed().await {
        let mode = match color_scheme {
            ColorScheme::NoPreference => Mode::Default,
            ColorScheme::PreferDark => Mode::Dark,
            ColorScheme::PreferLight => Mode::Light,
        };
        tx.send(mode).await?;
    }
    Ok(())
}

async fn non_freedesktop_watch(tx: Sender<Mode>) -> anyhow::Result<()> {
    let mut mode = detect();
    loop {
        let new_mode = detect();
        if mode != new_mode {
            mode = new_mode;
            tx.send(mode).await?;
        }
    }
}