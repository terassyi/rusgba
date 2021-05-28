use crate::error::*;
use crate::rom::bios::{ BiosRom };
use crate::ram::work_ram::{ EWRam, IWRam };
use crate::cartridge::{ GamePak };

#[derive(Debug)]
pub struct Bus<'a> {
    bios: &'a BiosRom,
    ewram: &'a mut EWRam,
    iwram: &'a mut IWRam,
    gamepak: &'a mut GamePak<'a>,
}

impl<'a> Bus<'a> {
    pub fn new(bios: &'a BiosRom, ewram: &'a mut EWRam, iwram: &'a mut IWRam, gamepak: &'a mut GamePak<'a>) -> Bus<'a> {
        Bus {
            bios: bios,
            ewram: ewram,
            iwram: iwram,
            gamepak: gamepak,
        }
    }
}
