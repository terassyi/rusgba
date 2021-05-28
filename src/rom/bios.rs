use crate::error::*;
use crate::util::num::{ KILO };

#[derive(Debug, Clone)]
pub struct BiosRom {
    inner: Vec<u8>
}

impl BiosRom {
    pub fn new() -> BiosRom {
        BiosRom {
            inner: Vec::with_capacity(16 * KILO),
        }
    }

    pub fn read(&self, addr: u32) -> GBAResult<u32> {
        Ok(0u32)
    }
}
