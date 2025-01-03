use crate::Mode;

#[cfg(any(feature = "sync", doc))]
pub(crate) mod sync {
    use crate::Mode;

    pub fn subscribe() -> std::sync::mpsc::Receiver<Mode> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut last_mode = crate::sync::detect();

        tx.send(last_mode).unwrap();

        std::thread::spawn(move || loop {
            let current_mode = crate::sync::detect();

            if current_mode != last_mode {
                if tx.send(current_mode).is_err() {
                    break;
                }
                last_mode = current_mode;
            }
        });

        rx
    }
}

pub async fn subscribe() -> impl futures_lite::Stream<Item = Mode> {
    Box::pin(futures_lite::stream::unfold(
        crate::detect().await,
        |last_mode| async move {
            loop {
                let current_mode = crate::detect().await;

                if current_mode != last_mode {
                    return Some((current_mode, current_mode));
                }
            }
        },
    ))
}
