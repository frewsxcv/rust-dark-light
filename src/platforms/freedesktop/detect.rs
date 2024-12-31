#[cfg(any(feature = "sync", doc))]
pub mod sync {
    use crate::Mode;

    pub fn detect() -> Mode {
        pollster::block_on(super::super::get_color_scheme())
    }
}

use crate::Mode;

pub async fn detect() -> Mode {
    super::get_color_scheme().await
}
