use crate::Mode;
use super::initial_value;

pub fn detect() -> Mode {
    pollster::block_on(initial_value())
}
