use crate::error::{GBAError, GBAResult};
use crate::util;
use crate::cpu::register::*;
use crate::cpu::bus::*;

const OPCODE_SHIFT: u16 = 0b0000_0000_0000_0000;
const OPCODE_ADD_SUB: u16 = 0b0001_1000_0000_0000;
const OPCODE_MOV_CMP_ADD_SUB_IMM: u16 = 0b0010_0000_0000_0000;
const OPCODE_ALU: u16 = 0b0100_0000_0000_0000;
const OPCODE_HI_REG_BX: u16 = 0b0100_0100_0000_0000;
const OPCODE_LOAD_PC_RELATIVE: u16 = 0b0100_1000_0000_0000;
const OPCODE_LOAD_STORE_WITH_REG_OFF: u16 = 0b0101_0000_0000_0000;
const OPCODE_LOAD_STORE_SBH: u16 = 0b0101_0010_0000_0000;
const OPCODE_LOAD_STORE_WITH_IMM_OFF: u16 = 0b0110_0000_0000_0000;
const OPCODE_LOAD_STORE_H: u16 = 0b1000_0000_0000_0000;
const OPCODE_LOAD_STORE_SP_RELATIVE: u16 = 0b1001_0000_0000_0000;
const OPCODE_PUSH_POP_REG: u16 = 0b1011_0100_0000_0000;
const OPCODE_MUL_LOAD_STORE: u16 = 0b1100_0000_0000_0000;
const OPCODE_GET_RELATIVE_ADDR: u16 = 0b1010_0000_0000_0000;
const OPCODE_ADD_OFF_SP: u16 = 0b1011_0000_0000_0000;
const OPCODE_COND_B: u16 = 0b1101_0000_0000_0000;
const OPCODE_SWI: u16 = 0b1101_1111_0000_0000;
const OPCODE_BKPT: u16 = 0b1011_1110_0000_0000;
const OPCODE_B: u16 = 0b1110_0000_0000_0000;
const OPCODE_LONG_BR_WITH_LINK_1: u16 = 0b1111_0000_0000_0000;
const OPCODE_LONG_BR_WITH_LINK_2: u16 = 0b1111_1000_0000_0000;

const OPCODE_SHIFT_MASK: u16 = 0b0000_0000_0000_0000;
const OPCODE_SHIFT_OP_MASK: u16 = 0b0001_1000_0000_0000;
const OPCODE_ADD_SUB_MASK: u16 = 0b1111_1000_0000_0000;
const OPCODE_MOV_CMP_ADD_SUB_IMM_MASK: u16 = 0b1110_0000_0000_0000;
const OPCODE_ALU_MASK: u16 = 0b1111_1000_0000_0000;
const OPCODE_HI_REG_BX_MASK: u16 = 0b1111_1100_0000_0000;
const OPCODE_LOAD_PC_RELATIVE_MASK: u16 = 0b1111_1000_0000_0000;
const OPCODE_LOAD_STORE_WITH_REG_OFF_MASK: u16 = 0b1111_0010_0000_0000;
const OPCODE_LOAD_STORE_SBH_MASK: u16 = 0b1111_0010_0000_0000;
const OPCODE_LOAD_STORE_WITH_IMM_OFF_MASK: u16 = 0b1110_0000_0000_0000;
const OPCODE_LOAD_STORE_H_MASK: u16 = 0b1111_0000_0000_0000;
const OPCODE_LOAD_STORE_SP_RELATIVE_MASK: u16 = 0b1111_0000_0000_0000;
const OPCODE_PUSH_POP_REG_MASK: u16 = 0b1111_1100_0000_0000;
const OPCODE_MUL_LOAD_STORE_MASK: u16 = 0b1111_0000_0000_0000;
const OPCODE_GET_RELATIVE_ADDR_MASK: u16 = 0b1111_0000_0000_0000;
const OPCODE_ADD_OFF_SP_MASK: u16 = 0b1111_1111_0000_0000;
const OPCODE_COND_B_MASK: u16 = 0b1111_0000_0000_0000;
const OPCODE_SWI_MASK: u16 = 0b1111_1111_0000_0000;
const OPCODE_BKPT_MASK: u16 = 0b1111_1111_0000_0000;
const OPCODE_B_MASK: u16 = 0b1111_1000_0000_0000;
const OPCODE_LONG_BR_WITH_LINK_1_MASK: u16 = 0b1111_1000_0000_0000;
const OPCODE_LONG_BR_WITH_LINK_2_MASK: u16 = 0b1111_1000_0000_0000;

type InstructionFn = fn(inst: u16, reg: &mut Registers, bus: & mut Bus) -> GBAResult<u16>;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    SHIFT,
    ADD_SUB,
    MOV_CMP_ADD_SUB_IMM,
    ALU,
    HI_REG_BX,
    LOAD_PC_RELATIVE,
    LOAD_STORE_WITH_REG_OFF,
    LOAD_STORE_SBH,
    LOAD_STORE_WITH_IMM_OFF,
    LOAD_STORE_H,
    LOAD_STORE_SP_RELATIVE,
    PUSH_POP_REG,
    MUL_LOAD_STORE,
    GET_RELATIVE_ADDR,
    ADD_OFF_SP,
    COND_B,
    SWI,
    BKPT,
    B,
    LONG_BR_WITH_LINK1,
    LONG_BR_WITH_LINK2,
}

impl Instruction {
    pub fn from(inst: u16) -> GBAResult<Instruction> {
        if is_shift(inst) { Ok(Instruction::SHIFT) }
        else if is_add_sub(inst) { Ok(Instruction::ADD_SUB) }
        else if is_mov_cmp_add_sub_imm(inst) { Ok(Instruction::MOV_CMP_ADD_SUB_IMM) }
        else if is_alu(inst) { Ok(Instruction::ALU) }
        else if is_hi_reg_bx(inst) { Ok(Instruction::HI_REG_BX) }
        else if is_load_pc_relative(inst) { Ok(Instruction::LOAD_PC_RELATIVE) }
        else if is_alu(inst) { Ok(Instruction::LOAD_STORE_WITH_REG_OFF) }
        else if is_alu(inst) { Ok(Instruction::LOAD_STORE_SBH) }
        else if is_alu(inst) { Ok(Instruction::LOAD_STORE_WITH_IMM_OFF) }
        else if is_alu(inst) { Ok(Instruction::LOAD_STORE_H) }
        else if is_alu(inst) { Ok(Instruction::LOAD_STORE_SP_RELATIVE) }
        else if is_alu(inst) { Ok(Instruction::PUSH_POP_REG) }
        else if is_alu(inst) { Ok(Instruction::MUL_LOAD_STORE) }
        else if is_alu(inst) { Ok(Instruction::GET_RELATIVE_ADDR) }
        else if is_alu(inst) { Ok(Instruction::ADD_OFF_SP) }
        else if is_alu(inst) { Ok(Instruction::COND_B) }
        else if is_alu(inst) { Ok(Instruction::SWI) }
        else if is_alu(inst) { Ok(Instruction::BKPT) }
        else if is_alu(inst) { Ok(Instruction::B) }
        else if is_alu(inst) { Ok(Instruction::LONG_BR_WITH_LINK1) }
        else if is_alu(inst) { Ok(Instruction::LONG_BR_WITH_LINK2) }
        else { Err(GBAError::InstructionNotFound) }
    }

    pub fn function(&self) -> InstructionFn {
        match *self {
            Instruction::SHIFT => shift,
            Instruction::ADD_SUB => add_sub,
            Instruction::MOV_CMP_ADD_SUB_IMM => mov_cmp_add_sub_imm,
            Instruction::ALU => alu,
            Instruction::HI_REG_BX => hi_reg_bx,
            Instruction::LOAD_PC_RELATIVE => load_pc_relative,
            Instruction::LOAD_STORE_WITH_REG_OFF => load_store_with_reg_offset,
            Instruction::LOAD_STORE_SBH => load_store_sign_ex_byte_halfword,
            Instruction::LOAD_STORE_WITH_IMM_OFF => load_store_with_imm_offset,
            Instruction::LOAD_STORE_H => load_store_halfword,
            Instruction::LOAD_STORE_SP_RELATIVE => load_store_sp_relative,
            Instruction::PUSH_POP_REG => push_pop_reg,
            Instruction::MUL_LOAD_STORE => multiple_load_store,
            Instruction::GET_RELATIVE_ADDR => get_relative_addr,
            Instruction::ADD_OFF_SP => add_offset_sp,
            Instruction::COND_B => cond_b,
            Instruction::SWI => swi,
            Instruction::BKPT => breakpoint,
            Instruction::B => b,
            Instruction::LONG_BR_WITH_LINK1 => long_branch_with_link_1,
            Instruction::LONG_BR_WITH_LINK2 => long_branch_with_link_2,
        }
    }
}

fn shift(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn add_sub(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn mov_cmp_add_sub_imm(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn alu(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn hi_reg_bx(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn load_pc_relative(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn load_store_with_reg_offset(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn load_store_sign_ex_byte_halfword(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn load_store_with_imm_offset(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn load_store_halfword(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn load_store_sp_relative(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn push_pop_reg(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn multiple_load_store(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn get_relative_addr(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn add_offset_sp(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn cond_b(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn swi(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn breakpoint(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn b(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn long_branch_with_link_1(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn long_branch_with_link_2(inst: u16, reg: &mut Registers, bus: &mut Bus) -> GBAResult<u16> {
    Ok(0u16)
}

fn is_shift(inst: u16) -> bool {
    (inst & OPCODE_SHIFT_MASK == OPCODE_SHIFT) && !(inst & OPCODE_SHIFT_OP_MASK == OPCODE_SHIFT_OP_MASK)
}

fn is_add_sub(inst: u16) -> bool {
    inst & OPCODE_ADD_SUB_MASK == OPCODE_ADD_SUB
}

fn is_mov_cmp_add_sub_imm(inst: u16) -> bool {
    inst & OPCODE_MOV_CMP_ADD_SUB_IMM_MASK == OPCODE_MOV_CMP_ADD_SUB_IMM
}

fn is_alu(inst: u16) -> bool {
    inst & OPCODE_ALU_MASK == OPCODE_ALU
}

fn is_hi_reg_bx(inst: u16) -> bool {
    inst & OPCODE_HI_REG_BX_MASK == OPCODE_HI_REG_BX
}

fn is_load_pc_relative(inst: u16) -> bool {
    inst & OPCODE_LOAD_PC_RELATIVE_MASK == OPCODE_LOAD_PC_RELATIVE
}

fn is_load_store_with_reg_offset(inst: u16) -> bool {
    inst & OPCODE_LOAD_STORE_WITH_REG_OFF_MASK== OPCODE_LOAD_STORE_WITH_REG_OFF
}

fn is_load_store_sign_ex_byte_halfword(inst: u16) -> bool {
    inst & OPCODE_LOAD_STORE_SBH_MASK== OPCODE_LOAD_STORE_SBH
}

fn is_load_store_with_imm_offset(inst: u16) -> bool {
    inst & OPCODE_LOAD_STORE_WITH_IMM_OFF_MASK == OPCODE_LOAD_STORE_WITH_IMM_OFF
}

fn is_load_store_halfword(inst: u16) -> bool {
    inst & OPCODE_LOAD_STORE_H_MASK == OPCODE_LOAD_STORE_H
}

fn is_load_store_sp_relative(inst: u16) -> bool {
    inst & OPCODE_LOAD_STORE_SP_RELATIVE_MASK == OPCODE_LOAD_STORE_SP_RELATIVE
}

fn is_load_store_(inst: u16) -> bool {
    inst & OPCODE_LOAD_STORE_SP_RELATIVE_MASK == OPCODE_LOAD_STORE_SP_RELATIVE
}

fn is_push_pop_reg(inst: u16) -> bool {
    inst & OPCODE_PUSH_POP_REG_MASK == OPCODE_PUSH_POP_REG
}

fn is_multiple_load_store(inst: u16) -> bool {
    inst & OPCODE_MUL_LOAD_STORE_MASK== OPCODE_MUL_LOAD_STORE
}

fn is_get_relative_addr(inst: u16) -> bool {
    inst & OPCODE_GET_RELATIVE_ADDR_MASK == OPCODE_GET_RELATIVE_ADDR
}

fn is_add_offset_sp(inst: u16) -> bool {
    inst & OPCODE_ADD_OFF_SP_MASK == OPCODE_ADD_OFF_SP
}

fn is_cond_b(inst: u16) -> bool {
    inst & OPCODE_COND_B_MASK == OPCODE_COND_B
}

fn is_swi(inst: u16) -> bool {
    inst & OPCODE_SWI_MASK == OPCODE_SWI
}

fn is_breakpoint(inst: u16) -> bool {
    inst & OPCODE_BKPT_MASK == OPCODE_BKPT
}

fn is_b(inst: u16) -> bool {
    inst & OPCODE_B_MASK == OPCODE_B
}

fn is_long_branch_with_link_1(inst: u16) -> bool {
    inst & OPCODE_LONG_BR_WITH_LINK_1_MASK == OPCODE_LONG_BR_WITH_LINK_1
}

fn is_long_branch_with_link_2(inst: u16) -> bool {
    inst & OPCODE_LONG_BR_WITH_LINK_2_MASK == OPCODE_LONG_BR_WITH_LINK_2
}
