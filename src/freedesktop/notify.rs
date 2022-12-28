
use ashpd::Error;
use ashpd::desktop::settings::{SettingsProxy, ColorScheme};
use ashpd::zbus::Connection;
use tokio::sync::mpsc::Sender;

use crate::Mode;

use super::{get_freedesktop_color_scheme, detect::detect};

struct Settings<'a> {
    proxy: &'a SettingsProxy<'a>
}

impl<'a> Settings<'a> {
    fn new(proxy: &'a SettingsProxy<'a>) -> Self {
        Self {
            proxy
        }
    }
    async fn receive(&self) -> std::result::Result<ColorScheme, Error> {
        self.proxy.receive_color_scheme_changed().await
    }
}

pub async fn notify(tx: Sender<crate::Mode>) -> anyhow::Result<()> {
    let connection = Connection::session().await?;
    let proxy = SettingsProxy::new(&connection).await?;
    let settings = Settings::new(&proxy);
    if get_freedesktop_color_scheme().await.is_ok() {
        loop {
            freedesktop_watch(tx.clone(), &settings).await?
        }
    } else {
        non_freedesktop_watch(tx)
    }
}

async fn freedesktop_watch<'a>(tx: Sender<Mode>, settings: &'a Settings<'a>) -> anyhow::Result<()> {
    let color_scheme = match settings.receive().await? {
        ColorScheme::PreferDark => Mode::Dark,
        ColorScheme::PreferLight => Mode::Light,
        ColorScheme::NoPreference => Mode::Default,
    };
    tx.send(color_scheme).await?;
    Ok(())
}

fn non_freedesktop_watch(tx: Sender<Mode>) -> anyhow::Result<()> {
    let mut mode = detect();
    tokio::spawn(async move {
        // TODO: Remove and replace for something like `notify`.
        loop {
            let new_mode = detect();
            if mode != new_mode {
                mode = new_mode;
                tx.send(mode).await.unwrap();
            }
        }
    });
    Ok(())
}