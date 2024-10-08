use crate::Mode;

pub fn detect() -> Mode {
    pollster::block_on(super::get_color_scheme())
}
