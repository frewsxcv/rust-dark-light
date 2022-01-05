use crate::Mode;
use anyhow::Result;
use std::process;

pub fn detect() -> Result<Mode> {
    let mode = if let Ok(output) = process::Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("AppleInterfaceStyle")
        .output()
    {
        Mode::from(output.stdout.starts_with(b"Dark"))
    } else {
        Mode::Light
    };
    Ok(mode)
}
