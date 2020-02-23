use crate::cpu::cpu::Cpu;
use crate::memory::mmu::{Mmu, Opcode};
use crate::util::binary::bytes_to_word;

pub mod cb_instructions;
mod functions;
pub mod instructions;

pub enum ExecutionType {
    ActionTaken,
    Jumped,
    JumpedActionTaken,
    None,
}

pub struct Instruction {
    pub length: u16,
    pub clock_cycles: u8,
    pub clock_cycles_condition: Option<u8>,
    pub description: &'static str,
    pub handler: fn(cpu: &mut Cpu, mmu: &mut Mmu, op_code: &Opcode) -> ExecutionType,
}

pub fn get_instruction_by_op_code(op_code: &Opcode) -> Option<&Instruction> {
    match op_code {
        Opcode::Regular(value) => instructions::get_instruction(&value),
        Opcode::CB(value) => cb_instructions::get_instruction(&value),
    }
}

fn read_hl_addr(cpu: &Cpu, mmu: &Mmu) -> u8 {
    mmu.read(bytes_to_word(cpu.registers.h, cpu.registers.l))
}

fn write_hl_addr(value: u8, cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write(bytes_to_word(cpu.registers.h, cpu.registers.l), value);
}
