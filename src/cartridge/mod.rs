use crate::error::*;
use super::rom::Rom;
use super::rom::gamepak::{ GamePakRom, FlashRom };
use super::ram::gamepak::{ SRam };

#[derive(Debug)]
pub struct GamePak<'a> {
    gamepak1: &'a GamePakRom,
    gamepak2: &'a GamePakRom,
    gamepak3: &'a GamePakRom,
    flash: &'a FlashRom,
    sram: &'a mut SRam,
}

impl<'a> GamePak<'a> {
    pub fn new(gamepak1: &'a GamePakRom, gamepak2: &'a GamePakRom, gamepak3: &'a GamePakRom,
                flash: &'a FlashRom, sram: &'a mut SRam) -> GamePak<'a> {
        GamePak {
            gamepak1: gamepak1,
            gamepak2: gamepak2,
            gamepak3: gamepak3,
            flash: flash,
            sram: sram,
        }
    }
}
