use super::initial_value;
use crate::Mode;

pub fn detect() -> Mode {
    pollster::block_on(initial_value())
}
