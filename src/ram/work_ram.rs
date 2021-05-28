use crate::error::*;
use crate::util::num::KILO;

#[derive(Debug, Clone)]
pub struct IWRam {
    inner: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct EWRam {
    inner: Vec<u8>
}

impl IWRam {
    pub fn new() -> IWRam {
        IWRam {
            inner: Vec::with_capacity(32 * KILO)
        }
    }
}

impl EWRam {
    pub fn new() -> EWRam {
        EWRam {
            inner: Vec::with_capacity(256 * KILO)
        }
    }
}
