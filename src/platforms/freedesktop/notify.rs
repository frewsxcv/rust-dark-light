use ashpd::desktop::settings::{ColorScheme, Settings};
use futures::{stream, Stream, StreamExt};
use std::task::Poll;

use crate::{detect, platforms::Event, Mode};

pub async fn subscribe() -> anyhow::Result<impl Stream<Item = Event<Mode>> + Send> {
    let mut last_mode = detect();

    let stream = if get_freedesktop_color_scheme().await.is_ok() {
        let proxy = Settings::new().await?;
        proxy
            .receive_color_scheme_changed()
            .await?
            .map(Mode::from)
            .map(|mode| Event::ThemeChanged(mode))
            .boxed()
    } else {
        stream::poll_fn(move |_| -> Poll<Option<Event<Mode>>> {
            let current_mode = detect();

            if current_mode != last_mode {
                last_mode = current_mode;
                Poll::Ready(Some(Event::ThemeChanged(current_mode)))
            } else {
                Poll::Ready(Some(Event::Waiting))
            }
        })
        .boxed()
    };

    Ok(stream)
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
