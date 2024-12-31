use std::error::Error;

use ashpd::desktop::settings::Settings;
use futures_lite::{Stream, StreamExt};

use crate::Mode;

#[cfg(any(feature = "sync", doc))]
pub(crate) mod sync {
    use super::super::Mode;
    use super::color_scheme_stream;
    use futures_lite::StreamExt;

    pub fn subscribe() -> std::sync::mpsc::Receiver<Mode> {
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            futures_lite::future::block_on(async {
                let stream = match color_scheme_stream().await {
                    Ok(stream) => stream.boxed(),
                    Err(err) => {
                        log::error!("Failed to subscribe to color scheme changes: {}", err);
                        futures_lite::stream::empty().boxed()
                    }
                };

                stream
                    .for_each(|mode| {
                        let _ = tx.send(mode);
                    })
                    .await;
            });
        });

        rx
    }
}

pub async fn subscribe() -> impl Stream<Item = Mode> + Send {
    match color_scheme_stream().await {
        Ok(stream) => stream.boxed(),
        Err(err) => {
            log::error!("Failed to subscribe to color scheme changes: {}", err);
            futures_lite::stream::empty().boxed()
        }
    }
}

pub async fn color_scheme_stream() -> Result<impl Stream<Item = Mode> + Send, Box<dyn Error>> {
    let color_scheme_stream = Settings::new()
        .await?
        .receive_color_scheme_changed()
        .await?
        .map(Mode::from);
    Ok(Box::pin(color_scheme_stream))
}
