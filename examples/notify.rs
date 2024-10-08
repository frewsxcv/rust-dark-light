use futures::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = dark_light::subscribe().await.unwrap();
    while let Some(mode) = stream.next().await {
        println!("System theme changed: {:?}", mode);
    }
}
