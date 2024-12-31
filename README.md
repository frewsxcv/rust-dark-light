<div align="center">
    <img src="resources/icon/icon.svg" width="200"/>
    <h1>dark-light</h1>
    <p>Rust crate to detect the system theme mode</p>
</div>

Supports macOS, Windows, Linux, BSDs, and WASM. On Linux the XDG Desktop Portal D-Bus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access.

[API Documentation](https://docs.rs/dark-light/)

## Usage

### Detect current theme mode
You can detect the current mode by using the `detect` function. This function returns a `Mode` value.
```rust
use dark_light::Mode;

let mode = dark_light::detect().await;

match mode {
    Mode::Dark => {},
    Mode::Light => {},
    Mode::Default => {},
}
```

### Subscribe to system theme changes
You can subscribe to system theme changes by using the `subscribe` function. This function returns a stream of `Mode` values. The stream will emit a new value whenever the system theme changes.

```rust
use dark_light::Mode;

#[tokio::main]
async fn main() {
    let mut stream = dark_light::subscribe().await;
    while let Some(mode) = stream.next().await {
        match mode {
            Mode::Dark => {},
            Mode::Light => {},
            Mode::Default => {},
        }
    }
}
```

This crate is asynchronous by default, but can be used in a synchronous manner by enabling the `sync` feature and prefixing the methods with `sync::`. 

 ```rust
let mode = dark_light::sync::detect();
let rx = dark_light::sync::subscribe();
```

> If you are upgrading this crate, you may update your code to use the async API or alternatively, you can use the `sync` feature to use the synchronous version of the methods.

## Example

Async:
```
cargo run --example async
```

Sync
```
cargo run --example sync
```

## Test
To test the crate, run the following command:
```
cargo test --doc --all-features
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


