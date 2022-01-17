# rust-dark-light

Rust crate to detect if dark mode or light mode is enabled. Supports macOS, Windows, Linux, and BSDs. On Linux and BSDs, first the XDG Desktop Portal dbus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access. If that does not work, fallback methods are used for KDE, GNOME, Cinnamon, MATE, XFCE, and Unity.

[API Documentation](https://docs.rs/dark-light/)

## Usage

```rust
fn main() {
    let mode = dark_light::detect();

    match mode {
        dark_light::Mode::Dark => {},
        dark_light::Mode::Light => {},
    }
}
```

## Example

```
cargo run --example detect
```

