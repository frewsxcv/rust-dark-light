use dark_light::ThemeWatcher;

#[tokio::main]
async fn main() {
    let mut receiver = ThemeWatcher::new().lock().await.subscribe();

    while let Ok(mode) = receiver.recv().await {
        println!("New mode: {:?}", mode);
    }
}
