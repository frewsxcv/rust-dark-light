use std::error::Error;

use ashpd::desktop::settings::Settings;
use futures::{stream, Stream, StreamExt};

use crate::Mode;

#[cfg(feature = "sync")]
pub async fn subscribe() -> Result<impl Stream<Item = Mode> + Send, Box<dyn Error>> {
    pollster::block_on(subscribe())
}

#[cfg(not(feature = "sync"))]
pub async fn subscribe() -> Result<impl Stream<Item = Mode> + Send, Box<dyn Error>> {
    let initial = stream::once(super::get_color_scheme()).boxed();
    let later_updates = Settings::new()
        .await?
        .receive_color_scheme_changed()
        .await?
        .map(Mode::from)
        .boxed();
    Ok(initial.chain(later_updates))
}
