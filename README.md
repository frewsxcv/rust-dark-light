# rust-dark-light

Rust crate to detect if dark mode or light mode is enabled.

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

