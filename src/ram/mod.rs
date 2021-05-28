pub mod gamepak;
pub mod work_ram;

use super::error::*;

pub trait Ram {
    fn read(&self, addr: u32) -> GBAResult<u32>;
    fn write(&mut self, addr: u32, val: u32) -> GBAResult<()>;
}
