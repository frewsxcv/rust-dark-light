use ashpd::desktop::settings::Settings;
use futures::{Stream, StreamExt};

use crate::Mode;

pub async fn subscribe() -> anyhow::Result<impl Stream<Item = Mode> + Send> {
    Ok(Settings::new()
        .await?
        .receive_color_scheme_changed()
        .await?
        .map(Mode::from)
        .boxed())
}
