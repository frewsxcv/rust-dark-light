use std::process;

fn is_dark_mode_enabled() -> bool {
    if let Ok(output) = process::Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("AppleInterfaceStyle")
        .output()
    {
        output.stdout.starts_with(b"Dark")
    } else {
        false
    }
}

pub fn detect() -> crate::Mode {
    if is_dark_mode_enabled() {
        crate::Mode::Dark
    } else {
        crate::Mode::Light
    }
}
