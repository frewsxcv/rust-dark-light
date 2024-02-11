use dark_light::Event;
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    while let Some(event) = dark_light::subscribe().await?.next().await {
        if let Event::ThemeChanged(mode) = event {
            println!("System theme changed: {:?}", mode);
        }
    }

    Ok(())
}
