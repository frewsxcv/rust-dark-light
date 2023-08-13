use anyhow::Ok;
use dark_light::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let action = |mode| {
        match mode {
            Mode::Dark => println!("Dark mode"),
            Mode::Light => println!("Light mode"),
            Mode::Default => println!("Default or unspecified"),
        }
    };
    notify(action).await?;
    Ok(())
}