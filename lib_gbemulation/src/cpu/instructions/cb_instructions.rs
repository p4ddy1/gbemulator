use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::{
    functions, read_hl_addr, write_hl_addr, ExecutionType, Instruction,
};
use crate::memory::mmu::{Mmu, Opcode};
use crate::util::binary::{reset_bit_in_byte, set_bit_in_byte};

//TODO: Fix descriptions
macro_rules! bit {
    ($bit: expr) => {
        Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT $bit,(B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                functions::check_bit(cpu, read_by_opcode(op_code, cpu), $bit);
                ExecutionType::None
            },
        })
    };
}

macro_rules! bit_hl {
    ($bit: expr) => {
        Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "BIT $bit,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::check_bit(cpu, read_hl_addr(cpu, mmu), $bit);
                ExecutionType::None
            },
        })
    };
}

macro_rules! res {
    ($bit: expr) => {
        Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RES $bit,(B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let result = reset_bit_in_byte(read_by_opcode(op_code, cpu), $bit);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        })
    };
}

macro_rules! res_hl {
    ($bit: expr) => {
        Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RES $bit,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = reset_bit_in_byte(read_hl_addr(cpu, mmu), $bit);
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        })
    };
}

macro_rules! set {
    ($bit: expr) => {
        Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SET $bit,(B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let result = set_bit_in_byte(read_by_opcode(op_code, cpu), $bit);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        })
    };
}

macro_rules! set_hl {
    ($bit: expr) => {
        Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "SET $bit,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = set_bit_in_byte(read_hl_addr(cpu, mmu), $bit);
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        })
    };
}

pub fn get_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        0x00..=0x05 | 0x07 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RLC (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::rotate_left(cpu, value, true);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x06 => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RLC (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = functions::rotate_left(cpu, read_hl_addr(cpu, mmu), true);
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x08..=0x0D | 0x0F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RRC (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::rotate_right(cpu, value, true);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x0E => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RRC (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = functions::rotate_right(cpu, read_hl_addr(cpu, mmu), true);
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x10..=0x15 | 0x17 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::rotate_left_through_carry(cpu, value, true);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x16 => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RL (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result =
                    functions::rotate_left_through_carry(cpu, read_hl_addr(cpu, mmu), true);
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x18..=0x1D | 0x1F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RR (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::rotate_right_through_carry(cpu, value, true);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x1E => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RR (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result =
                    functions::rotate_right_through_carry(cpu, read_hl_addr(cpu, mmu), true);
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x20..=0x25 | 0x27 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SLA (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::sla(cpu, value);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x26 => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "SLA (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = functions::sla(cpu, read_hl_addr(cpu, mmu));
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x28..=0x2D | 0x2F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRA (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::sra(cpu, value);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x2E => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "SRA (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = functions::sra(cpu, read_hl_addr(cpu, mmu));
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x30..=0x35 | 0x37 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::swap_nibbles(cpu, value);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x36 => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "SWAP (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = functions::swap_nibbles(cpu, read_hl_addr(cpu, mmu));
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x38..=0x3D | 0x3F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL (B..A)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, op_code: &Opcode| {
                let value = read_by_opcode(op_code, cpu);
                let result = functions::srl(cpu, value);
                write_by_opcode(op_code, result, cpu);
                ExecutionType::None
            },
        }),
        0x3E => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "SRL (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = functions::srl(cpu, read_hl_addr(cpu, mmu));
                write_hl_addr(result, cpu, mmu);
                ExecutionType::None
            },
        }),
        0x40..=0x45 | 0x47 => bit!(0),
        0x46 => bit_hl!(0),
        0x48..=0x4D | 0x4F => bit!(1),
        0x4E => bit_hl!(1),
        0x50..=0x55 | 0x57 => bit!(2),
        0x56 => bit_hl!(2),
        0x58..=0x5D | 0x5F => bit!(3),
        0x5E => bit_hl!(3),
        0x60..=0x65 | 0x67 => bit!(4),
        0x66 => bit_hl!(4),
        0x68..=0x6D | 0x6F => bit!(5),
        0x6E => bit_hl!(5),
        0x70..=0x75 | 0x77 => bit!(6),
        0x76 => bit_hl!(6),
        0x78..=0x7D | 0x7F => bit!(7),
        0x7E => bit_hl!(7),
        0x80..=0x85 | 0x87 => res!(0),
        0x86 => res_hl!(0),
        0x88..=0x8D | 0x8F => res!(1),
        0x8E => res_hl!(1),
        0x90..=0x95 | 0x97 => res!(2),
        0x96 => res_hl!(2),
        0x98..=0x9D | 0x9F => res!(3),
        0x9E => res_hl!(3),
        0xA0..=0xA5 | 0xA7 => res!(4),
        0xA6 => res_hl!(4),
        0xA8..=0xAD | 0xAF => res!(5),
        0xAE => res_hl!(5),
        0xB0..=0xB5 | 0xB7 => res!(6),
        0xB6 => res_hl!(6),
        0xB8..=0xBD | 0xBF => res!(7),
        0xBE => res_hl!(7),
        0xC0..=0xC5 | 0xC7 => set!(0),
        0xC6 => set_hl!(0),
        0xC8..=0xCD | 0xCF => set!(1),
        0xCE => set_hl!(1),
        0xD0..=0xD5 | 0xD7 => set!(2),
        0xD6 => set_hl!(2),
        0xD8..=0xDD | 0xDF => set!(3),
        0xDE => set_hl!(3),
        0xE0..=0xE5 | 0xE7 => set!(4),
        0xE6 => set_hl!(4),
        0xE8..=0xED | 0xEF => set!(5),
        0xEE => set_hl!(5),
        0xF0..=0xF5 | 0xF7 => set!(6),
        0xF6 => set_hl!(6),
        0xF8..=0xFD | 0xFF => set!(7),
        0xFE => set_hl!(7),
    }
}

fn read_by_opcode(op_code: &Opcode, cpu: &Cpu) -> u8 {
    match get_lower_nibble_of_opcode(op_code) {
        0x00 | 0x80 => cpu.registers.b,
        0x10 | 0x90 => cpu.registers.c,
        0x20 | 0xA0 => cpu.registers.d,
        0x30 | 0xB0 => cpu.registers.e,
        0x40 | 0xC0 => cpu.registers.h,
        0x50 | 0xD0 => cpu.registers.l,
        0x70 | 0xF0 => cpu.registers.a,
        _ => panic!("Unknown register"),
    }
}

fn write_by_opcode(op_code: &Opcode, value: u8, cpu: &mut Cpu) {
    match get_lower_nibble_of_opcode(op_code) {
        0x00 | 0x80 => cpu.registers.b = value,
        0x10 | 0x90 => cpu.registers.c = value,
        0x20 | 0xA0 => cpu.registers.d = value,
        0x30 | 0xB0 => cpu.registers.e = value,
        0x40 | 0xC0 => cpu.registers.h = value,
        0x50 | 0xD0 => cpu.registers.l = value,
        0x70 | 0xF0 => cpu.registers.a = value,
        _ => panic!("Unknown register"),
    }
}

fn get_lower_nibble_of_opcode(op_code: &Opcode) -> u8 {
    let op_val = match op_code {
        Opcode::CB(value) => value,
        _ => panic!("No CB Opcode"),
    };

    op_val << 4
}
