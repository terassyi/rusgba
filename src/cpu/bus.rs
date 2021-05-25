use crate::error::*;

pub trait Bus {
    fn read(&self, addr: u32) -> GBAResult<u32>;
}

#[derive(Debug, Clone, Copy)]
pub struct CpuBus {

}

impl CpuBus {
    pub fn new() -> CpuBus {
        CpuBus {}
    }
}

impl Bus for CpuBus {
    fn read(&self, addr: u32) -> GBAResult<u32> {
        Ok(0u32)
    }
}
