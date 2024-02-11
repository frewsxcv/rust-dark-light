use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    while let Some(mode) = dark_light::subscribe().await?.next().await {
        println!("System theme changed: {:?}", mode);
    }

    Ok(())
}
