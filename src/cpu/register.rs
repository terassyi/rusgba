use crate::error::*;
use crate::util::*;

pub trait Register {
    fn set(&mut self, num: usize, val: u32) -> GBAResult<()>;
    fn get(&self, num: usize) -> GBAResult<u32>;
}

pub const FIQ: usize = 0;
pub const USR: usize = 1;
pub const SVC: usize = 2;
pub const ABT: usize = 3;
pub const IRQ: usize = 4;
pub const UND: usize = 5;

pub const CPSR_N: usize = 31;
pub const CPSR_Z: usize = 30;
pub const CPSR_C: usize = 29;
pub const CPSR_V: usize = 28;
pub const CPSR_Q: usize = 27;
pub const CPSR_I: usize = 7;
pub const CPSR_F: usize = 6;
pub const CPSR_T: usize = 5;

#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub gen: GeneralRegister,
    pub fiq: FIQRegister,
    pub usr: UsrRegister,
    pub r13_bank: BankRegister,
    pub r14_bank: BankRegister,
    pub spsr_bank: BankRegister,
    pub cpsr: CpsrRegister, // 31-> N(sign), 30-> Z(zero), 29-> C(carry), 28-> V(overflow), 27-> Q(sticky(not used))
                   // 7-> I(IRQ disable), 6-> F(FIQ disable), 5->T(thumb), 4-0-> M4-M0(mode)
                   // https://github.com/pokemium/gba_doc_ja/blob/main/arm7tdmi/cond.md#cpsr
}

#[derive(Debug, Copy, Clone)]
pub struct GeneralRegister {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    r13: u32,
    r14: u32,
    r15: u32,
}

// fast interrupt request
#[derive(Debug, Copy, Clone)]
pub struct FIQRegister {
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct UsrRegister {
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct BankRegister {
    fiq: u32,
    usr: u32,
    svc: u32,
    abt: u32,
    irq: u32,
    und: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct CpsrRegister {
    inner: u32,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            gen: GeneralRegister::new(),
            fiq: FIQRegister::new(),
            usr: UsrRegister::new(),
            r13_bank: BankRegister::new(),
            r14_bank: BankRegister::new(),
            spsr_bank: BankRegister::new(),
            cpsr: CpsrRegister::new(),
        }
    }

}

impl GeneralRegister {
    fn new() -> GeneralRegister {
        GeneralRegister {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        }
    }
}

impl Register for GeneralRegister {
    fn set(&mut self, num: usize, val: u32) -> GBAResult<()> {
        match num {
            0 => self.r0 = val,
            1 => self.r1 = val,
            2 => self.r2 = val,
            3 => self.r3 = val,
            4 => self.r4 = val,
            5 => self.r5 = val,
            6 => self.r6 = val,
            7 => self.r7 = val,
            8 => self.r8 = val,
            9 => self.r9 = val,
            10 => self.r10 = val,
            11 => self.r11 = val,
            12 => self.r12 = val,
            13 => self.r13 = val,
            14 => self.r14 = val,
            15 => self.r15 = val,
            _ => return Err(GBAError::InvalidData),
        }
        Ok(())
    }

    fn get(&self, num: usize) -> GBAResult<u32> {
        match num {
            0 => Ok(self.r0),
            1 => Ok(self.r1),
            2 => Ok(self.r2),
            3 => Ok(self.r3),
            4 => Ok(self.r4),
            5 => Ok(self.r5),
            6 => Ok(self.r6),
            7 => Ok(self.r7),
            8 => Ok(self.r8),
            9 => Ok(self.r9),
            10 => Ok(self.r10),
            11 => Ok(self.r11),
            12 => Ok(self.r12),
            13 => Ok(self.r13),
            14 => Ok(self.r14),
            15 => Ok(self.r15),
            _ => return Err(GBAError::InvalidData),
        }
    }
}

impl FIQRegister {
    fn new() -> FIQRegister {
        FIQRegister {
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
        }
    }

}

impl Register for FIQRegister {
    fn set(&mut self, num: usize, val: u32) -> GBAResult<()> {
        match num {
            8 => self.r8 = val,
            9 => self.r9 = val,
            10 => self.r10 = val,
            11 => self.r11 = val,
            12 => self.r12 = val,
            _ => return Err(GBAError::InvalidData),
        }
        Ok(())
    }

    fn get(&self, num: usize) -> GBAResult<u32> {
        match num {
            8 => Ok(self.r8),
            9 => Ok(self.r9),
            10 => Ok(self.r10),
            11 => Ok(self.r11),
            12 => Ok(self.r12),
            _ => return Err(GBAError::InvalidData),
        }
    }
}

impl UsrRegister {
    fn new() -> UsrRegister {
        UsrRegister {
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
        }
    }
    pub fn set(&mut self, num: usize, val: u32) -> GBAResult<()> {
        match num {
            8 => self.r8 = val,
            9 => self.r9 = val,
            10 => self.r10 = val,
            11 => self.r11 = val,
            12 => self.r12 = val,
            _ => return Err(GBAError::InvalidData),
        }
        Ok(())
    }

    pub fn get(&self, num: usize) -> GBAResult<u32> {
        match num {
            8 => Ok(self.r8),
            9 => Ok(self.r9),
            10 => Ok(self.r10),
            11 => Ok(self.r11),
            12 => Ok(self.r12),
            _ => return Err(GBAError::InvalidData),
        }
    }
}

impl Register for UsrRegister {
    fn set(&mut self, num: usize, val: u32) -> GBAResult<()> {
        match num {
            8 => self.r8 = val,
            9 => self.r9 = val,
            10 => self.r10 = val,
            11 => self.r11 = val,
            12 => self.r12 = val,
            _ => return Err(GBAError::InvalidData),
        }
        Ok(())
    }

    fn get(&self, num: usize) -> GBAResult<u32> {
        match num {
            8 => Ok(self.r8),
            9 => Ok(self.r9),
            10 => Ok(self.r10),
            11 => Ok(self.r11),
            12 => Ok(self.r12),
            _ => return Err(GBAError::InvalidData),
        }
    }
}

impl BankRegister {
    fn new() -> BankRegister {
        BankRegister {
            fiq: 0,
            usr: 0,
            svc: 0,
            abt: 0,
            irq: 0,
            und: 0,
        }
    }
}

impl Register for BankRegister {
    fn set(&mut self, num: usize, val: u32) -> GBAResult<()> {
        match num {
            0 => self.fiq = val,
            1 => self.usr = val,
            2 => self.svc = val,
            3 => self.abt = val,
            4 => self.irq = val,
            5 => self.und = val,
            _ => return Err(GBAError::InvalidData),
        }
        Ok(())
    }

    fn get(&self, num: usize) -> GBAResult<u32> {
        match num {
            0 => Ok(self.fiq),
            1 => Ok(self.usr),
            2 => Ok(self.svc),
            3 => Ok(self.abt),
            4 => Ok(self.irq),
            5 => Ok(self.und),
            _ => return Err(GBAError::InvalidData),
        }
    }
}

impl CpsrRegister {
    fn new() -> CpsrRegister {
        CpsrRegister { inner: 0u32 }
    }

    pub fn set_flag(&mut self, flag: usize, bit: bool) {
        self.inner = set_bit_u32(self.inner, flag, bit)
    }

    fn is_flag_set(&self, flag: usize) -> bool {
        self.inner & (1 << flag) != 0
    }

    pub fn carry(&self) -> u32 {
        if self.is_flag_set(CPSR_C) { 1u32 } else { 0u32 }
    }

    pub fn zero(&self) -> u32 {
        if self.is_flag_set(CPSR_Z) { 1u32 } else { 0u32 }
    }

    pub fn sign(&self) -> u32 {
        if self.is_flag_set(CPSR_N) { 1u32 } else { 0u32 }
    }

    pub fn overflow(&self) -> u32 {
        if self.is_flag_set(CPSR_V) { 1u32 } else { 0u32 }
    }

    
    pub fn sticky(&self) -> u32 {
        if self.is_flag_set(CPSR_Q) { 1u32 } else { 0u32 }
    }
}
impl Register for CpsrRegister {
    fn get(&self, num: usize) -> GBAResult<u32> {
        match num {
            CPSR_N => Ok(self.inner & (1 << CPSR_N)),
            CPSR_Z => Ok(self.inner & (1 << CPSR_Z)),
            CPSR_C => Ok(self.inner & (1 << CPSR_C)),
            CPSR_V => Ok(self.inner & (1 << CPSR_V)),
            CPSR_Q => Ok(self.inner & (1 << CPSR_Q)),
            CPSR_I => Ok(self.inner & (1 << CPSR_I)),
            CPSR_F => Ok(self.inner & (1 << CPSR_F)),
            CPSR_T => Ok(self.inner & (1 << CPSR_T)),
            _ => Err(GBAError::InvalidData),
        }
    }
    
    fn set(&mut self, num: usize, val: u32) -> GBAResult<()> {
        if val > 1 || val < 0 {
            return Err(GBAError::InvalidData);
        }
        let b = if val == 1 { true } else { false };
        self.inner = match num {
            CPSR_N => set_bit_u32(self.inner, CPSR_N, b),
            CPSR_Z => set_bit_u32(self.inner, CPSR_Z, b),
            CPSR_C => set_bit_u32(self.inner, CPSR_C, b),
            CPSR_V => set_bit_u32(self.inner, CPSR_V, b),
            CPSR_Q => set_bit_u32(self.inner, CPSR_Q, b),
            CPSR_I => set_bit_u32(self.inner, CPSR_I, b),
            CPSR_F => set_bit_u32(self.inner, CPSR_F, b),
            CPSR_T => set_bit_u32(self.inner, CPSR_T, b),
            _ => return Err(GBAError::InvalidData),
        };
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::Register;
    #[test]
    fn test_general_register_set() {
        let mut reg = super::GeneralRegister::new();
        let res = reg.set(0usize, 1u32).is_ok();
        assert_eq!(res, true);
    }
    #[test]
    fn test_general_register_get() {
        let mut reg = super::GeneralRegister::new();
        reg.set(0usize, 1u32).unwrap();
        assert_eq!(reg.get(0usize).unwrap(), 1u32);
    }
    #[test]
    fn test_cpsr_regiter_set() {
        let mut cpsr = super::CpsrRegister::new();
        let res = cpsr.set(super::CPSR_N, 1u32).is_ok();
        assert_eq!(res, true);
    }
    #[test]
    fn test_cpsr_register_get() {
        let mut cpsr = super::CpsrRegister::new();
        assert_eq!(cpsr.get(super::CPSR_N).unwrap(), 0u32);
        cpsr.set(super::CPSR_N, 1u32).unwrap();
        assert_eq!(cpsr.get(super::CPSR_N).unwrap(), (1u32 << super::CPSR_N));
    }
}
