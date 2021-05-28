pub mod bios;
pub mod gamepak;

use super::error::*;

pub trait Rom {
    fn read(&self, addr: u32) -> GBAResult<u32>;
}
