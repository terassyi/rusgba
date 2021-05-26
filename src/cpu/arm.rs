use crate::error::{GBAError, GBAResult};
use crate::util;
use super::register::*;
use super::bus::*;

const OPCODE_ALU: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
const OPCODE_SWI: u32 = 0b0000_1111_0000_0000_0000_0000_0000_0000; // 27-26 bits == 00
const OPCODE_UND: u32 = 0b0000_0110_0000_0000_0000_0000_0000_0000; // 27-25 bits == 011
const OPCODE_B: u32 = 0b0000_1010_0000_0000_0000_0000_0000_0000; // 27-24 bits == 1010
const OPCODE_BL: u32 = 0b0000_1011_0000_0000_0000_0000_0000_0000; // 27-24 bits == 1011
const OPCODE_BX: u32 = 0b0000_0001_0010_1111_1111_1111_0001_0000; // 27-4 bits == 0001_0010_1111_1111_1111_0001
const OPCODE_MPY: u32 = 0b0000_1010_0000_0000_0000_0000_0000_0000; // 27-25 bits == 000, 7-4 bits == 1001
const OPCODE_LDR: u32 = 0b0000_0100_0001_0000_0000_0000_0000_0000; // 27-26 bits == 01
const OPCODE_STR: u32 = 0b0000_0100_0000_0000_0000_0000_0000_0000; // 27-26 bits == 01
const OPCODE_LDRH: u32 = 0b0000_0000_0001_0000_0000_0000_1011_0000; // 27-25 bits == 000, 20 bit == 1, 7-4 bits == 1011
const OPCODE_LDRSB: u32 = 0b0000_0000_0001_0000_0000_0000_1101_0000; // 27-25 bits == 000, 20 bit == 1, 7-4 bits == 1101
const OPCODE_LDRSH: u32 = 0b0000_0000_0001_0000_0000_0000_1111_0000; // 27-25 bits == 000, 20 bit == 1, 7-4 bits == 1111
const OPCODE_STRH: u32 = 0b0000_0000_0000_0000_0000_0000_1011_0000; // 27-25 bits == 000, 20 bit == 0, 7-4 bits == 1011
const OPCODE_LDM: u32 = 0b0000_1000_0001_0000_0000_0000_1011_0000; // 27-25 bits == 100, 20 bit == 1
const OPCODE_STM: u32 = 0b0000_1000_0000_0000_0000_0000_1011_0000; // 27-25 bits == 100, 20 bit == 0
const OPCODE_MRS: u32 = 0b0000_0001_0000_1111_0000_0000_0000_0000; // 27-23 bits == 0001, 21-20 bit == 00
const OPCODE_MSR: u32 = 0b0000_0001_0010_0000_1111_0000_0000_0000; // 27-26 bits == 00, 24-23 bit == 10, 15-12 bits == 1111

const OPCODE_ALU_MASK: u32 = 0b0000_1100_0000_0000_0000_0000_0000_0000;
const OPCODE_SWI_MASK: u32 = 0b0000_1111_0000_0000_0000_0000_0000_0000;
const OPCODE_UND_MASK: u32 = 0b0000_1110_0000_0000_0000_0000_0000_0000;
const OPCODE_B_MASK: u32 = 0b0000_1111_0000_0000_0000_0000_0000_0000;
const OPCODE_BL_MASK: u32 = 0b0000_1111_0000_0000_0000_0000_0000_0000;
const OPCODE_BX_MASK: u32 = 0b0000_1111_1111_1111_1111_1111_1111_0000;
const OPCODE_MPY_MASK: u32 = 0b0000_1110_0000_0000_0000_0000_1111_0000;
const OPCODE_LDR_MASK: u32 = 0b0000_1100_0001_0000_0000_0000_0000_0000;
const OPCODE_STR_MASK: u32 = 0b0000_1100_0001_0000_0000_0000_1111_0000;
const OPCODE_LDRH_MASK: u32 = 0b0000_1100_0001_0000_0000_0000_1111_0000;
const OPCODE_LDRSB_MASK: u32 = 0b0000_1110_0001_0000_0000_0000_1111_0000;
const OPCODE_LDRSH_MASK: u32 = 0b0000_1110_0001_0000_0000_0000_1111_0000;
const OPCODE_STRH_MASK: u32 = 0b0000_1110_0001_0000_0000_0000_1111_0000;
const OPCODE_LDM_MASK: u32 = 0b0000_1110_0001_0000_0000_0000_0000_0000;
const OPCODE_STM_MASK: u32 = 0b0000_1110_0001_0000_0000_0000_0000_0000;
const OPCODE_MRS_MASK: u32 = 0b0000_1111_1011_1111_0000_1111_1111_1111;
const OPCODE_MSR_MASK: u32 = 0b0000_1101_1011_0000_1111_0000_0000_0000;

type InstructionFn = fn(inst: u32, reg: &mut Registers, bus: & mut CpuBus) -> GBAResult<u32>;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    SWI,   // software interruption
    B,     // branch
    BL,    // branch
    BX,    // special branch
    ALU,   // alu related
    UND,   // undifined instruction exception
    LDR,   // load
    STR,   // store
    LDRH,  // load half word
    LDRSB, // load signed byte
    LDRSH, // load signed half word
    STRH,  // store half word
    LDM,   // load stack
    STM,   // store stack
    MSR,   //
    MRS,
    MPY, // multi related
}

impl Instruction {
    pub fn from(inst: u32) -> GBAResult<Instruction> {
        if is_swi(inst) {
            Ok(Instruction::SWI)
        } else if is_b(inst) {
            Ok(Instruction::B)
        } else if is_bl(inst) {
            Ok(Instruction::BL)
        } else if is_bx(inst) {
            Ok(Instruction::BX)
        } else if is_alu(inst) {
            Ok(Instruction::ALU)
        } else if is_und(inst) {
            Ok(Instruction::UND)
        } else if is_ldr(inst) {
            Ok(Instruction::LDR)
        } else if is_store(inst) {
            Ok(Instruction::STR)
        } else if is_ldrh(inst) {
            Ok(Instruction::LDRH)
        } else if is_ldrsb(inst) {
            Ok(Instruction::LDRSB)
        } else if is_ldrsh(inst) {
            Ok(Instruction::LDRSH)
        } else if is_strh(inst) {
            Ok(Instruction::STRH)
        } else if is_ldm(inst) {
            Ok(Instruction::LDM)
        } else if is_stm(inst) {
            Ok(Instruction::STM)
        } else if is_msr(inst) {
            Ok(Instruction::MSR)
        } else if is_mrs(inst) {
            Ok(Instruction::MRS)
        } else if is_mpy(inst) {
            Ok(Instruction::MPY)
        } else {
            Err(GBAError::NotFound)
        }
    }

    pub fn function(&self) -> InstructionFn {
        match *self {
            Instruction::SWI => swi,
            Instruction::B => b,
            Instruction::BL => bl,
            Instruction::BX => bx,
            Instruction::ALU => alu,
            Instruction::UND => und,
            Instruction::LDR => ldr,
            Instruction::STR => store,
            Instruction::LDRH => ldrh,
            Instruction::LDRSB => ldrsb,
            Instruction::LDRSH => ldrsh,
            Instruction::STRH => strh,
            Instruction::LDM => ldm,
            Instruction::STM => stm,
            Instruction::MSR => msr,
            Instruction::MRS => mrs,
            Instruction::MPY => mpy,
        }
    }
}

pub fn swi(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn b(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}
pub fn bl(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn bx(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn alu(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    if is_and(inst) {
        and(inst, reg, bus)
    } else if is_eor(inst) {
        eor(inst, reg, bus)
    } else if is_sub(inst) {
        sub(inst, reg, bus)
    } else if is_rsb(inst) {
        rsb(inst, reg, bus)
    } else if is_add(inst) {
        add(inst, reg, bus)
    } else if is_adc(inst) {
        adc(inst, reg, bus)
    } else if is_sbc(inst) {
        sbc(inst, reg, bus)
    } else if is_rsc(inst) {
        rsc(inst, reg, bus)
    } else if is_tst(inst) {
        tst(inst, reg, bus)
    } else if is_teq(inst) {
        teq(inst, reg, bus)
    } else if is_cmp(inst) {
        cmp(inst, reg, bus)
    } else if is_cmn(inst) {
        cmn(inst, reg, bus)
    } else if is_orr(inst) {
        orr(inst, reg, bus)
    } else if is_mov(inst) {
        mov(inst, reg, bus)
    } else if is_bic(inst) {
        bic(inst, reg, bus)
    } else if is_mvn(inst) {
        mvn(inst, reg, bus)
    } else {
        Err(GBAError::NotFound)
    }
}

// aru function
pub fn and(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn eor(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn sub(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn rsb(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn add(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn adc(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn sbc(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn rsc(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn tst(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn teq(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn cmp(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn cmn(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn orr(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn mov(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn bic(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn mvn(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn und(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0 as u32)
}

pub fn ldr(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn store(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn ldrh(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn ldrsb(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn ldrsh(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn strh(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn ldm(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn stm(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn msr(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn mrs(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn mpy(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    if is_mul(inst) {
        mul(inst, reg, bus)
    } else if is_mla(inst) {
        mla(inst, reg, bus)
    } else if is_umull(inst) {
        umull(inst, reg, bus)
    } else if is_umlal(inst) {
        umlal(inst, reg, bus)
    } else if is_smull(inst) {
        smull(inst, reg, bus)
    } else if is_smlal(inst) {
        smlal(inst, reg, bus)
    } else {
        Err(GBAError::NotFound)
    }
}

pub fn mul(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn mla(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn umull(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn umlal(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn smull(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

pub fn smlal(inst: u32, reg: &mut Registers, bus: &mut CpuBus) -> GBAResult<u32> {
    Ok(0)
}

fn rd(inst: u32) -> usize {
    // register number for storing the result of compute
    ((inst >> 12) & 0b1111) as usize
}

fn rn(inst: u32) -> usize {
    // register number for first operand
    ((inst >> 16) & 0b1111) as usize
}

fn op2(inst: u32, reg: &mut Registers) -> GBAResult<u32> {
    let is_carry = util::bit_u32(inst, 20);
    match is_imm(inst) {
        true => {
            // use immediate
            let nn = inst & 0b1111_1111;
            let ls = ((inst >> 8) & 0b1111) * 2;
            // ror
            Ok(ror(nn, ls, is_carry, false, reg))
        },
        false => {
            // use register
            let rm = inst & 0b1111;
            let mut shift_val = 0;
            let shift_type = (inst >> 5) & 0b11;
            let mut salt: u32= 0; // TODO: I'm not sure, why this variable is necessary.
            let is_reg = util::bit_u32(inst, 4);
            if is_reg {
                // 11-8 bits is register number that has the shift value.
                let reg_num = (inst >> 8) & 0b1111;
                shift_val = reg.gen.get(reg_num as usize)? & 0b1111_1111;
                if rm == 15u32 {
                    salt = 4
                }
            } else {
                // 11-7 bits is immediate show the shift value
                shift_val = (inst >> 7) & 0b1111;
            }
            let r = reg.gen.get(rm as usize)? + salt;
            match shift_type {
                LSL => Ok(lsl(r, shift_val, is_carry, !is_reg, reg)),
                LSR => Ok(lsr(r, shift_val, is_carry, !is_reg, reg)),
                ASR => Ok(asr(r, shift_val, is_carry, !is_reg, reg)),
                ROR => Ok(ror(r, shift_val, is_carry, !is_reg, reg)),
                _ => Ok(r),
            }
        },
    }
}

fn ror(val: u32, shift_val: u32, is_carry: bool, imm: bool, reg: &mut Registers) -> u32 {
    // https://github.com/pokemium/gba_doc_ja/blob/main/arm7tdmi/arm/alu.md#-%E3%82%B7%E3%83%95%E3%83%88%E9%87%8F%E3%81%8C0%E3%81%AE%E3%81%A8%E3%81%8D
    if shift_val == 0 && imm {
        let carried = reg.cpsr.carry();
        reg.cpsr.set_flag(CPSR_C, util::bit_u32(val, 0));
        util::ror((val & !1u32) | carried, 1)
    } else {
        let c = val >> (shift_val - 1) & 0b1 > 0;
        if shift_val > 0 && is_carry {
            reg.cpsr.set_flag(CPSR_C, c);
        }
        util::ror(val, shift_val as usize)
    }
}

fn lsl(val: u32, shift_val: u32, is_carry: bool, imm: bool, reg: &mut Registers) -> u32 {
    if shift_val == 0 && imm {
        val
    } else if shift_val > 32 {
        if is_carry {
            reg.cpsr.set_flag(CPSR_C, false);
        }
        0u32
    } else {
        let c = val & (1 << (32 - shift_val)) > 0;
        if shift_val > 0 && is_carry {
            reg.cpsr.set_flag(CPSR_C, c);
        }
        util::lsl(val, shift_val as usize)
    }
}

fn lsr(val: u32, shift_val: u32, is_carry: bool, imm: bool, reg: &mut Registers) -> u32 {
    let mut s_v = shift_val;
    if shift_val == 0 && imm {
        s_v = 32;
    }
    let c = val & (1 << (s_v - 1)) > 0;
    if s_v > 0 && is_carry {
        reg.cpsr.set_flag(CPSR_C, c);
    }
    util::lsr(val, s_v as usize)
}

fn asr(val: u32, shift_val: u32, is_carry: bool, imm: bool, reg: &mut Registers) -> u32 {
    let mut s_v = shift_val;
    if shift_val == 0 && imm {
        s_v = 32;
    }
    let c = val & (1 << (s_v - 1)) > 0;
    if s_v > 0 && is_carry {
        reg.cpsr.set_flag(CPSR_C, c);
    }
    util::asr(val, s_v as usize)
}

const LSL: u32 = 0;
const LSR: u32 = 1;
const ASR: u32 = 2;
const ROR: u32 = 3;

fn is_imm(inst: u32) -> bool {
    util::bit_u32(inst, 25)
}

fn set_flag_logic_instruction(rd: usize, cond: bool, res: u32, is_tst_teq: bool, reg: &mut Registers) -> GBAResult<()> {
    if rd == 15 {
        if cond {
            // return to user mode from other mode.
        } else if !is_tst_teq {
            // https://github.com/pokemium/gba_doc_ja/blob/main/arm7tdmi/arm/alu.md#s1-with-unused-rd-bits1111b-%E3%82%AA%E3%83%9A%E3%82%B3%E3%83%BC%E3%83%89%E3%81%8C-cmppcmnptstpteqp-%E3%81%AE%E3%81%A8%E3%81%8D
        }
    } else if cond {
        reg.cpsr.set_flag(CPSR_Z, res == 0);
        reg.cpsr.set_flag(CPSR_N, util::bit_u32(res, 31));
    };
    Ok(())
}

fn set_flag_arith_instruction(rd: usize, cond: bool, lhs: u32, rhs: u32, res: u64, is_cmp: bool, reg: &mut Registers) -> GBAResult<()> {
    if rd == 15 {
        if cond {
            // return to user mode from other mode.
        } else if !is_cmp {
            // https://github.com/pokemium/gba_doc_ja/blob/main/arm7tdmi/arm/alu.md#s1-with-unused-rd-bits1111b-%E3%82%AA%E3%83%9A%E3%82%B3%E3%83%BC%E3%83%89%E3%81%8C-cmppcmnptstpteqp-%E3%81%AE%E3%81%A8%E3%81%8D
        }
    } else if cond {
        reg.cpsr.set_flag(CPSR_Z, res == 0);
        reg.cpsr.set_flag(CPSR_N, util::bit_u32(res as u32, 31));
        reg.cpsr.set_flag(CPSR_C, util::is_add_carry(res));
        reg.cpsr.set_flag(CPSR_V, util::is_add_overflow(lhs, rhs, res as u32));
    };
    Ok(())
}

fn is_swi(inst: u32) -> bool {
    inst & OPCODE_SWI_MASK == OPCODE_SWI
}

fn is_b(inst: u32) -> bool {
    inst & OPCODE_B_MASK == OPCODE_B
}
fn is_bl(inst: u32) -> bool {
    inst & OPCODE_BL_MASK == OPCODE_BL
}

fn is_bx(inst: u32) -> bool {
    inst & OPCODE_BX_MASK == OPCODE_BX
}

fn is_alu(inst: u32) -> bool {
    inst & OPCODE_ALU_MASK == OPCODE_ALU
}

// aru function
fn is_and(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x0
}

fn is_eor(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x1
}

fn is_sub(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x2
}

fn is_rsb(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x3
}

fn is_add(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x4
}

fn is_adc(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x5
}

fn is_sbc(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x6
}

fn is_rsc(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x7
}

fn is_tst(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x8
}

fn is_teq(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0x9
}

fn is_cmp(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0xa
}

fn is_cmn(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0xb
}

fn is_orr(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0xc
}

fn is_mov(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0xd
}

fn is_bic(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0xe
}

fn is_mvn(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0xf
}

fn is_und(inst: u32) -> bool {
    inst & OPCODE_UND_MASK == OPCODE_UND
}

fn is_ldr(inst: u32) -> bool {
    inst & OPCODE_LDR_MASK == OPCODE_LDR
}

fn is_store(inst: u32) -> bool {
    inst & OPCODE_STR_MASK == OPCODE_STR
}

fn is_ldrh(inst: u32) -> bool {
    inst & OPCODE_LDRH_MASK == OPCODE_LDRH
}

fn is_ldrsb(inst: u32) -> bool {
    inst & OPCODE_LDRSB_MASK == OPCODE_LDRSB
}

fn is_ldrsh(inst: u32) -> bool {
    inst & OPCODE_LDRSH_MASK == OPCODE_LDRSH
}

fn is_strh(inst: u32) -> bool {
    inst & OPCODE_STRH_MASK == OPCODE_STRH
}

fn is_ldm(inst: u32) -> bool {
    inst & OPCODE_LDM_MASK == OPCODE_LDM
}

fn is_stm(inst: u32) -> bool {
    inst & OPCODE_STM_MASK == OPCODE_STM
}

fn is_msr(inst: u32) -> bool {
    inst & OPCODE_MSR_MASK == OPCODE_MSR
}

fn is_mrs(inst: u32) -> bool {
    inst & OPCODE_MRS_MASK == OPCODE_MRS
}

fn is_mpy(inst: u32) -> bool {
    inst & OPCODE_MPY_MASK == OPCODE_MPY
}

fn is_mul(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0b0000
}

fn is_mla(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0b0001
}

fn is_umull(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0b0100
}

fn is_umlal(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0b0101
}

fn is_smull(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0b0110
}

fn is_smlal(inst: u32) -> bool {
    inst >> 21 & 0b1111 == 0b0111
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    #[test]
    fn test_instfuction_function() {
        let mut reg = super::super::register::Registers::new();
        let mut bus = super::super::bus::CpuBus::new();
        let inst = Instruction::UND;
        let func = inst.function();
        assert_eq!(func(0 as u32, &mut reg, &mut bus).unwrap(), 0 as u32);
    }
    #[test]
    fn test_instfuction_from() {
        let inst_b = super::OPCODE_B | 0b0000_0000_0000_0000_0000_0000_0000_1111;
        let inst = Instruction::from(inst_b).unwrap();
        assert_eq!(inst, Instruction::B);
    }
    #[test]
    fn test_instfuction_from_err() {
        let invalid_inst = super::OPCODE_MSR | 0b0000_1000_0000_0000_0000_0000_0000_0000;
        assert_eq!(Instruction::from(invalid_inst).is_ok(), false);
    }
    #[test]
    fn test_rd() {
        let inst = super::OPCODE_ALU | 0b0000_0000_0000_0000_0101_0000_0000_0000;
        assert_eq!(super::rd(inst), 5usize);
    }
    #[test]
    fn test_rn() {
        let inst = super::OPCODE_ALU | 0b0000_0000_0000_1100_0101_0000_0000_0000;
        assert_eq!(super::rn(inst), 12usize);
    }
    #[test]
    fn test_ror() {
        // TODO: I wannt to test...        
    }
    #[test]
    fn test_lsl() {
        // TODO: I wannt to test...        
    }
    #[test]
    fn test_lsr() {
        // TODO: I wannt to test...        
    }
    #[test]
    fn test_asr() {
        // TODO: I wannt to test...        
    }
}
