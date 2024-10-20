use std::error::Error;

use futures_lite::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = dark_light::subscribe().await;
    while let Some(mode) = stream.next().await {
        println!("System theme changed: {:?}", mode);
    }

    Ok(())
}
