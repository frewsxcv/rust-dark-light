# rust-dark-light

Rust crate to detect if dark mode or light mode is enabled. Supports macOS and Windows.

[API Documentation](https://docs.rs/dark-light/)

## Usage

```rust
fn main() {
    let mode = dark_light::detect();
}
```

## Example

```
cargo run --example detect
```

