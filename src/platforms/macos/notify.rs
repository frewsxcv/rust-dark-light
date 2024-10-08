use std::task::Poll;
use std::error::Error;

use futures::{stream, Stream};

use crate::{detect, Mode};

pub async fn subscribe() -> Result<impl Stream<Item = Mode> + Send, Box<dyn Error>> {
    let mut last_mode = detect();

    let stream = stream::poll_fn(move |ctx| -> Poll<Option<Mode>> {
        let current_mode = detect();

        if current_mode != last_mode {
            last_mode = current_mode;
            Poll::Ready(Some(current_mode))
        } else {
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    });

    Ok(stream)
}
