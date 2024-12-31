fn main() -> Result<(), Box<dyn std::error::Error>> {
    detect();
    subscribe();
    Ok(())
}

fn detect() {
    println!("Current mode: {:?}", dark_light::detect());
}

fn subscribe() {
    let stream = dark_light::subscribe();
    while let Ok(mode) = stream.recv() {
        println!("System theme changed: {:?}", mode);
    }
}
