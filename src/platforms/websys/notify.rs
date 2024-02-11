use std::task::Poll;

use futures::{stream, Stream};

use crate::{detect, platforms::Event, Mode};

pub async fn subscribe() -> anyhow::Result<impl Stream<Item = Event<Mode>> + Send> {
    let mut last_mode = detect();

    let stream = stream::poll_fn(move |_| -> Poll<Option<Event<Mode>>> {
        let current_mode = detect();

        if current_mode != last_mode {
            last_mode = current_mode;
            Poll::Ready(Some(Event::ThemeChanged(current_mode)))
        } else {
            Poll::Ready(Some(Event::Waiting))
        }
    });

    Ok(stream)
}
