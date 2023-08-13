use std::sync::mpsc::Sender;

const duration: std::time::Duration = std::time::Duration::from_secs(1);

pub async fn notify(action: fn(mode: Mode)) -> anyhow::Result<()> {
    todo!()
}
