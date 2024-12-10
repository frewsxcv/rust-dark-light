use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    detect().await;
    subscribe().await;
    Ok(())
}

async fn detect() {
    println!("Current mode: {:?}", dark_light::detect().await);
}

async fn subscribe() {
    let mut stream = dark_light::subscribe().await;
    while let Some(mode) = stream.next().await {
        println!("System mode changed: {:?}", mode);
    }
}
