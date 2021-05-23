#[derive(Debug, Copy, Clone)]
pub struct Registers {
    gen: GeneralRegister,
    fiq: FIQRegister,
    usr: UsrRegister,
    r13_bank: BankRegister,
    r14_bank: BankRegister,
    spsr_bank: BankRegister,
    cpsr: u32,
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

impl Registers {
    pub fn new() -> Registers {
        Registers {
            gen: GeneralRegister::new(),
            fiq: FIQRegister::new(),
            usr: UsrRegister::new(),
            r13_bank: BankRegister::new(),
            r14_bank: BankRegister::new(),
            spsr_bank: BankRegister::new(),
            cpsr: 0,
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
