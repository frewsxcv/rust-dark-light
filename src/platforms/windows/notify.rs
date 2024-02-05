use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_READ},
    RegKey,
};

use crate::Mode;

use super::detect::detect;
const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);

pub struct ThemeWatcher {
    sender: broadcast::Sender<Mode>,
    current_mode: Mutex<Mode>,
}

impl ThemeWatcher {
    pub fn new() -> Arc<Mutex<Self>> {
        let (sender, _) = broadcast::channel::<Mode>(256);

        let theme_watcher = ThemeWatcher {
            sender,
            current_mode: Mutex::new(detect()),
        };

        // Wrap the ThemeWatcher in an Arc<Mutex<ThemeWatcher>> for cloning
        let arc_watcher = Arc::new(Mutex::new(theme_watcher));

        // Spawn the asynchronous task
        tokio::spawn({
            let arc_watcher = Arc::clone(&arc_watcher);
            async move {
                arc_watcher.lock().await.monitor_theme_changes().await;
            }
        });

        arc_watcher
    }

    // Method to get the current theme mode
    pub async fn get_current_mode(&self) -> Mode {
        let current_mode = self.current_mode.lock().await;
        current_mode.clone()
    }

    // Method to subscribe to theme change events
    pub fn subscribe(&self) -> broadcast::Receiver<Mode> {
        self.sender.subscribe()
    }

    // The asynchronous method to monitor theme changes
    async fn monitor_theme_changes(&self) {
        // Specify the registry key path you want to monitor
        let key_path = r"Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";

        // Open the registry key with read access
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let key = hklm
            .open_subkey_with_flags(key_path, KEY_READ)
            .expect("Failed to open registry key");

        loop {
            // Get the current value of the registry key
            let current_value = key
                .get_value::<u32, _>("SystemUsesLightTheme")
                .expect("Failed to get initial value");

            // Compare the current value with the stored value
            let mut current_mode = self.current_mode.lock().await;
            let stored_value = *current_mode;

            if current_value != stored_value as u32 {
                // Update the current mode
                *current_mode = if current_value != 0 {
                    Mode::Light
                } else {
                    Mode::Dark
                };

                // Notify subscribers about the theme change
                let _ = self.sender.send(current_mode.clone());
            }

            // Sleep for a specified interval before checking again
            tokio::time::sleep(TIMEOUT).await;
        }
    }
}
