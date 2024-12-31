use std::error::Error;

use ashpd::desktop::settings::Settings;
use futures::{stream, Stream, StreamExt};

use crate::Mode;

#[cfg(feature = "sync")]
pub fn subscribe() -> std::sync::mpsc::Receiver<Mode> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        pollster::block_on(async {
            let stream = match color_scheme_stream().await {
                Ok(stream) => stream,
                Err(err) => {
                    log::error!("Failed to subscribe to color scheme changes: {}", err);
                    Box::pin(Box::new(stream::empty()))
                }
            };

            stream
                .for_each(|mode| {
                    let _ = tx.send(mode);
                    async {}
                })
                .await;
        });
    });

    rx
}

#[cfg(not(feature = "sync"))]
pub async fn subscribe() -> impl Stream<Item = Mode> + Send {
    match color_scheme_stream().await {
        Ok(stream) => stream,
        Err(err) => {
            log::error!("Failed to subscribe to color scheme changes: {}", err);
            panic!("Failed to subscribe to color scheme changes: {}", err);
        }
    }
}

pub async fn color_scheme_stream() -> Result<impl Stream<Item = Mode> + Send, Box<dyn Error>> {
    let initial = stream::once(super::get_color_scheme()).boxed();
    let later_updates = Settings::new()
        .await?
        .receive_color_scheme_changed()
        .await?
        .map(Mode::from)
        .boxed();
    Ok(Box::pin(initial.chain(later_updates)))
}
