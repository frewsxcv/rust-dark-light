use std::sync::mpsc::Sender;

const duration: std::time::Duration = std::time::Duration::from_secs(1);

pub async fn notify(tx: Sender<crate::Mode>) -> anyhow::Result<()> {
    tx.send(crate::Mode::Default)?;
    std::thread::sleep(duration);
    tx.send(crate::Mode::Light)?;
    std::thread::sleep(duration);
    tx.send(crate::Mode::Dark)?;
    Ok(())
}
