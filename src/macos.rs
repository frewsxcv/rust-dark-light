use crate::Mode;
use std::process;

pub fn detect() -> Mode {
    if let Ok(output) = process::Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("AppleInterfaceStyle")
        .output()
    {
        Mode::from(output.stdout.starts_with(b"Dark"))
    } else {
        Mode::Light
    }
}
