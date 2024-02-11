use dark_light::{subscribe, Event};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = subscribe().await?;

    while let Some(event) = stream.next().await {
        if let Event::ThemeChanged(mode) = event {
            println!("System theme changed: {:?}", mode);
        }
    }

    Ok(())
}
