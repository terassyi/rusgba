use crate::error::*;
use crate::util::num::{ KILO, MEGA };

#[derive(Debug)]
pub struct GamePakRom {
    inner: Vec<u8>
}

#[derive(Debug)]
pub struct FlashRom {
    inner: Vec<u8>
}

impl GamePakRom {
    pub fn new() -> GamePakRom {
        GamePakRom {
            inner: Vec::with_capacity(32 * MEGA)
        }
    }
}

impl FlashRom {
    pub fn new() -> FlashRom {
        FlashRom {
            inner: Vec::with_capacity(128 * KILO)
        }
    }
}
