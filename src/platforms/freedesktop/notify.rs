use crate::Mode;
use futures::Stream;

#[cfg(not(feature = "zbus"))]
pub async fn subscribe() -> anyhow::Result<impl Stream<Item = Mode> + Send> {
    use futures::stream;
    use std::{
        io::{BufRead, BufReader},
        process::{Command, Stdio},
    };
    let mut process = Command::new("dbus-monitor")
        .arg(
            "type='signal',\
            sender='org.freedesktop.portal.Desktop',\
            path='/org/freedesktop/portal/desktop',\
            interface='org.freedesktop.portal.Settings',\
            member='SettingChanged',\
            arg0='org.freedesktop.appearance',\
            arg1='color-scheme'",
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let stdout = process.stdout.take().unwrap();
    let lines = BufReader::new(stdout).lines();
    Ok(stream::iter(lines.filter_map(
        |line| match line.ok()?.chars().last()? {
            '0' => Some(Mode::NoPreference),
            '1' => Some(Mode::Dark),
            '2' => Some(Mode::Light),
            _ => None,
        },
    )))
}

#[cfg(feature = "zbus")]
pub async fn subscribe() -> anyhow::Result<impl Stream<Item = Mode> + Send> {
    use crate::detect;
    use futures::{stream, StreamExt};
    use std::task::Poll;
    let stream = if get_freedesktop_color_scheme().await.is_ok() {
        let proxy = ashpd::desktop::settings::Settings::new().await?;
        proxy
            .receive_color_scheme_changed()
            .await?
            .map(Mode::from)
            .boxed()
    } else {
        let mut last_mode = detect();
        stream::poll_fn(move |ctx| -> Poll<Option<Mode>> {
            let current_mode = detect();
            if current_mode != last_mode {
                last_mode = current_mode;
                Poll::Ready(Some(current_mode))
            } else {
                ctx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .boxed()
    };

    Ok(stream)
}

#[cfg(feature = "zbus")]
async fn get_freedesktop_color_scheme() -> anyhow::Result<Mode> {
    let proxy = ashpd::desktop::settings::Settings::new().await?;
    let color_scheme = proxy.color_scheme().await?;
    let mode = color_scheme.into();
    Ok(mode)
}
