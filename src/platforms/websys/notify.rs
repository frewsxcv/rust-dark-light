use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

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
        todo!("Implement theme monitoring for websys")
    }
}
