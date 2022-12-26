use dark_light::*;

#[tokio::main]
async fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::spawn(async move {
        notify(tx).await.unwrap();
    });
    while let Ok(mode) = rx.recv() {
        println!("{:?}",  mode)
    }
}