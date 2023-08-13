use crate::Mode;

const duration: std::time::Duration = std::time::Duration::from_secs(1);

pub async fn notify(_action: fn(mode: Mode)) -> anyhow::Result<()> {
    todo!()
}
