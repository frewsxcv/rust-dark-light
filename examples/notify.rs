use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = dark_light::subscribe().await?;
    while let Some(mode) = stream.next().await {
        println!("System theme changed: {:?}", mode);
    }

    Ok(())
}
