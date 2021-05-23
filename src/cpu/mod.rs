mod arm;
mod register;
mod thumb;

use super::error::{GBAError, GBAResult};
use register::*;

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
    register: Registers,
}

#[derive(Debug)]
pub enum Mode {
    USR,
    FIQ,
    IRQ,
    SWI,
    ABT,
    UND,
    SYS,
}

impl Mode {
    pub fn value(&self) -> u8 {
        match *self {
            Mode::USR => 0b10000,
            Mode::FIQ => 0b10001,
            Mode::IRQ => 0b10010,
            Mode::SWI => 0b10011,
            Mode::ABT => 0b10111,
            Mode::UND => 0b11011,
            Mode::SYS => 0b11111,
        }
    }

    pub fn from(mode: u8) -> GBAResult<Mode> {
        match mode {
            0b10000 => Ok(Mode::USR),
            0b10001 => Ok(Mode::FIQ),
            0b10010 => Ok(Mode::IRQ),
            0b10011 => Ok(Mode::SWI),
            0b10111 => Ok(Mode::ABT),
            0b11011 => Ok(Mode::UND),
            0b11111 => Ok(Mode::SYS),
            _ => Err(GBAError::InvalidData),
        }
    }
}
