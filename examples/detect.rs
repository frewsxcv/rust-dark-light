use anyhow::Result;

fn main() -> Result<()> {
    println!("Current mode: {:?}", dark_light::detect()?);
    Ok(())
}
