use crate::Mode;

#[cfg(feature = "sync")]
pub fn detect() -> Mode {
    pollster::block_on(super::get_color_scheme())
}

#[cfg(not(feature = "sync"))]
pub async fn detect() -> Mode {
    super::get_color_scheme().await
}
