use std::thread;
use async_ashpd::Error;
use async_ashpd::desktop::settings::{SettingsProxy, ColorScheme};
use async_ashpd::zbus::Connection;
use crossbeam::channel::{self, Sender};

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

pub async fn notify(callback: &dyn Fn(Mode)) -> anyhow::Result<()> {
    let (tx, rx) = channel::unbounded();
    let connection = Connection::session().await?;
    let proxy = SettingsProxy::new(&connection).await?;
    let settings = Settings::new(&proxy);
    if get_freedesktop_color_scheme().is_ok() {
        loop {
            let tx = tx.clone();
            freedesktop_watch(tx, &settings).await?;
            match rx.recv() {
                Ok(mode) => callback(mode),
                Err(_) => {},
            }
        }
    } else {
        non_freedesktop_watch(tx)?;
        loop {
            match rx.recv() {
                Ok(mode) => callback(mode),
                Err(_) => {},
            }
        }
    }
}

async fn freedesktop_watch<'a>(tx: Sender<Mode>, settings: &'a Settings<'a>) -> anyhow::Result<()> {
    let color_scheme = match settings.receive().await? {
        ColorScheme::PreferDark => Mode::Dark,
        ColorScheme::PreferLight => Mode::Light,
        ColorScheme::NoPreference => Mode::Default,
    };
    tx.send(color_scheme).unwrap();
    Ok(())
}

fn non_freedesktop_watch(tx: Sender<Mode>) -> anyhow::Result<()> {
    let mut mode = detect();
    thread::spawn(move || {
        // TODO: Remove and replace for something like `notify`.
        loop {
            let new_mode = detect();
            if mode != new_mode {
                mode = new_mode;
                tx.send(mode).unwrap();
            }
        }
    });
    Ok(())
}