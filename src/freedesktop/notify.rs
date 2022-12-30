
use ashpd::desktop::settings::{SettingsProxy, ColorScheme};
use ashpd::zbus::Connection;
use tokio::sync::mpsc::Sender;

use crate::Mode;

use super::detect::detect_async;
use super::get_freedesktop_color_scheme;

pub async fn notify(tx: Sender<crate::Mode>) -> anyhow::Result<()> {
    
    if get_freedesktop_color_scheme().await.is_ok() {
        freedesktop_watch(tx.clone()).await
    } else {
        non_freedesktop_watch(tx).await
    }
}

async fn freedesktop_watch<'a>(tx: Sender<Mode>) -> anyhow::Result<()> {
    tokio::spawn(async move {
        let connection = Connection::session().await.unwrap();
        let proxy = SettingsProxy::new(&connection).await.unwrap();
        loop {
            if let Ok(color_scheme) = proxy.receive_color_scheme_changed().await {
                let mode = match color_scheme {
                    ColorScheme::NoPreference => Mode::Default,
                    ColorScheme::PreferDark => Mode::Dark,
                    ColorScheme::PreferLight => Mode::Light,
                };
                tx.send(mode).await.unwrap()
            }
        }
    });
    Ok(())
}

async fn non_freedesktop_watch(tx: Sender<Mode>) -> anyhow::Result<()> {
    let mut mode = detect_async().await?;
    tokio::spawn(async move {
        loop {
            let new_mode = detect_async().await.unwrap();
            if mode != new_mode {
                mode = new_mode;
                tx.send(mode).await.unwrap();
            }
        }
    });
    Ok(())
}