use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use ashpd::desktop::settings::{ColorScheme, Settings};
use zbus::export::futures_util::StreamExt;

use crate::{detect, Mode};

pub struct ThemeWatcher {
    sender: broadcast::Sender<Mode>,
    current_mode: Mutex<Mode>,
}

/// Theme watcher implementation for subscriptions.
impl ThemeWatcher {
    pub fn new() -> Arc<Mutex<Self>> {
        let (sender, _) = broadcast::channel::<Mode>(256);

        let theme_watcher = ThemeWatcher {
            sender,
            current_mode: Mutex::new(detect()),
        };

        let arc_watcher = Arc::new(Mutex::new(theme_watcher));

        tokio::spawn({
            let arc_watcher = Arc::clone(&arc_watcher);
            async move {
                arc_watcher.lock().await.monitor_theme_changes().await;
            }
        });

        arc_watcher
    }

    /// Method to get the current theme mode
    pub async fn get_current_mode(&self) -> Mode {
        let current_mode = self.current_mode.lock().await;
        current_mode.clone()
    }

    /// Method to subscribe to theme change events
    pub fn subscribe(&self) -> broadcast::Receiver<Mode> {
        self.sender.subscribe()
    }

    /// The asynchronous method to monitor theme changes
    async fn monitor_theme_changes(&self) {
        if get_freedesktop_color_scheme().await.is_ok() {
            let proxy = Settings::new().await.unwrap();
            if let Ok(mut color_scheme) = proxy.receive_color_scheme_changed().await {
                while let Some(color_scheme) = color_scheme.next().await {
                    // Compare the current value with the stored value
                    let mut current_mode = self.current_mode.lock().await;

                    let mode = match color_scheme {
                        ColorScheme::NoPreference => Mode::Default,
                        ColorScheme::PreferDark => Mode::Dark,
                        ColorScheme::PreferLight => Mode::Light,
                    };

                    if *current_mode != mode {
                        *current_mode = mode;
                        let _ = self.sender.send(current_mode.clone());
                    }
                }
            }
        } else {
            eprintln!("Unable to start freedesktop proxy, falling back to legacy...");
            loop {
                let mut current_mode = self.current_mode.lock().await;
                let new_mode = detect();
                if *current_mode != new_mode {
                    *current_mode = new_mode;
                    let _ = self.sender.send(current_mode.clone());
                }
            }
        }
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
