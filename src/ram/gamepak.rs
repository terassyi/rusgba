use crate::error::*;
use crate::util::num::KILO;

#[derive(Debug)]
pub struct SRam {
    inner: Vec<u8>
}

impl SRam {
    pub fn new() -> SRam {
        SRam {
            inner: Vec::with_capacity(64 * KILO)
        }
    }
}
