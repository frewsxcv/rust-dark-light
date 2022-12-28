use anyhow::Ok;
use dark_light::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(4);
    notify(tx).await?;
    while let Some(mode) = rx.recv().await {
        println!("{:?}",  mode)
    }
    Ok(())
}