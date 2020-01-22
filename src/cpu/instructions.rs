use crate::cpu::registers::Flag;
use crate::memory::mmu::Opcode;
use crate::util::binary;
use crate::util::binary::{
    bytes_to_word, is_bit_set, reset_bit_in_byte, set_bit_in_byte, word_to_bytes,
};
use crate::Cpu;

pub enum Result {
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
    pub handler: fn(cpu: &mut Cpu) -> Result,
}

pub fn get_instruction_by_op_code(op_code: &Opcode) -> Option<&Instruction> {
    match op_code {
        Opcode::Regular(value) => get_regular_instruction(value),
        Opcode::CB(value) => get_cb_instruction(value),
    }
}

fn get_regular_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        0x00 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "NOP",
            handler: |_: &mut Cpu| Result::None,
        }),
        0x01 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD BC,nn",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.get_attribute_for_op_code(1);
                cpu.registers.c = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x03 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC BC",
            handler: |cpu: &mut Cpu| {
                let mut value = binary::bytes_to_word(cpu.registers.b, cpu.registers.c);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.b = byte1;
                cpu.registers.c = byte2;
                Result::None
            },
        }),
        0x04 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = increment_byte(cpu, cpu.registers.b);
                Result::None
            },
        }),
        0x05 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = decrement_byte(cpu, cpu.registers.b);
                Result::None
            },
        }),
        0x06 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD B,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x07 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "RLCA",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = rotate_left(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x08 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD aa,SP",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(
                    cpu.get_attribute_for_op_code(1),
                    cpu.get_attribute_for_op_code(0),
                );
                cpu.mmu.write_word(addr, cpu.registers.sp);
                Result::None
            },
        }),
        0x09 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,BC",
            handler: |cpu: &mut Cpu| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let b_c = bytes_to_word(cpu.registers.b, cpu.registers.c);

                let result = add_words(cpu, h_l, b_c);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                Result::None
            },
        }),
        0x0A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(BC)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu
                    .mmu
                    .read(bytes_to_word(cpu.registers.b, cpu.registers.c));
                Result::None
            },
        }),
        0x0B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC BC",
            handler: |cpu: &mut Cpu| {
                let mut value = bytes_to_word(cpu.registers.b, cpu.registers.c);
                value -= 1;
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.b = byte1;
                cpu.registers.c = byte2;
                Result::None
            },
        }),
        0x0C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = increment_byte(cpu, cpu.registers.c);
                Result::None
            },
        }),
        0x0D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = decrement_byte(cpu, cpu.registers.c);
                Result::None
            },
        }),
        0x0E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD C,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x11 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD DE,nn",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.get_attribute_for_op_code(1);
                cpu.registers.e = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x12 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (DE),A",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.d, cpu.registers.e),
                    cpu.registers.a,
                );
                Result::None
            },
        }),
        0x13 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC DE",
            handler: |cpu: &mut Cpu| {
                let mut value = binary::bytes_to_word(cpu.registers.d, cpu.registers.e);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.d = byte1;
                cpu.registers.e = byte2;
                Result::None
            },
        }),
        0x14 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = increment_byte(cpu, cpu.registers.d);
                Result::None
            },
        }),
        0x15 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = decrement_byte(cpu, cpu.registers.d);
                Result::None
            },
        }),
        0x16 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD D,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x17 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "RLA",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = rotate_left_through_carry(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x18 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "JR r8",
            handler: |cpu: &mut Cpu| {
                jump_to_attribute_address(cpu);
                Result::None
            },
        }),
        0x19 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,DE",
            handler: |cpu: &mut Cpu| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let d_e = bytes_to_word(cpu.registers.d, cpu.registers.e);

                let result = add_words(cpu, h_l, d_e);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                Result::None
            },
        }),
        0x1A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(DE)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu
                    .mmu
                    .read(binary::bytes_to_word(cpu.registers.d, cpu.registers.e));
                Result::None
            },
        }),
        0x1B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC DE",
            handler: |cpu: &mut Cpu| {
                let mut value = bytes_to_word(cpu.registers.d, cpu.registers.e);
                value -= 1;
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.d = byte1;
                cpu.registers.e = byte2;
                Result::None
            },
        }),
        0x1C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = increment_byte(cpu, cpu.registers.e);
                Result::None
            },
        }),
        0x1D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = decrement_byte(cpu, cpu.registers.e);
                Result::None
            },
        }),
        0x1E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD E,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        //TODO: 0x1F
        0x20 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR NZ,r8",
            handler: |cpu: &mut Cpu| {
                if jump_on_flag_reset(cpu, Flag::Z) {
                    return Result::ActionTaken;
                }
                Result::None
            },
        }),
        0x21 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD HL,nn",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.get_attribute_for_op_code(1);
                cpu.registers.l = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x22 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL+),A",
            handler: |cpu: &mut Cpu| {
                let mut value = binary::bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.mmu.write(value, cpu.registers.a);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                Result::None
            },
        }),
        0x23 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC HL",
            handler: |cpu: &mut Cpu| {
                let mut value = binary::bytes_to_word(cpu.registers.h, cpu.registers.l);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                Result::None
            },
        }),
        0x24 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = increment_byte(cpu, cpu.registers.h);
                Result::None
            },
        }),
        0x25 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = decrement_byte(cpu, cpu.registers.h);
                Result::None
            },
        }),
        0x26 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD H,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x27 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DAA",
            handler: |_: &mut Cpu| Result::None, //TODO: Implement DAA
        }),
        0x28 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR Z,n",
            handler: |cpu: &mut Cpu| {
                if jump_on_flag(cpu, Flag::Z) {
                    return Result::ActionTaken;
                }
                Result::None
            },
        }),
        0x29 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,HL",
            handler: |cpu: &mut Cpu| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);

                let result = add_words(cpu, h_l, h_l);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                Result::None
            },
        }),
        0x2A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(HL+)",
            handler: |cpu: &mut Cpu| {
                let mut value = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.registers.a = cpu.mmu.read(value);
                value = value.wrapping_add(1);
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                Result::None
            },
        }),
        0x2B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC HL",
            handler: |cpu: &mut Cpu| {
                let mut value = bytes_to_word(cpu.registers.h, cpu.registers.l);
                value -= 1;
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                Result::None
            },
        }),
        0x2C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = increment_byte(cpu, cpu.registers.l);
                Result::None
            },
        }),
        0x2D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = decrement_byte(cpu, cpu.registers.l);
                Result::None
            },
        }),
        0x2E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD L,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        0x2F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CPL",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.a ^ 0xFF;
                cpu.registers.set_flag(Flag::N);
                cpu.registers.set_flag(Flag::H);
                Result::None
            },
        }),
        0x30 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR NZ,r8",
            handler: |cpu: &mut Cpu| {
                if jump_on_flag_reset(cpu, Flag::C) {
                    return Result::ActionTaken;
                }
                Result::None
            },
        }),
        0x31 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD SP,nn",
            handler: |cpu: &mut Cpu| {
                cpu.registers.sp = binary::bytes_to_word(
                    cpu.get_attribute_for_op_code(1),
                    cpu.get_attribute_for_op_code(0),
                );
                Result::None
            },
        }),
        0x32 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL-), A",
            handler: |cpu: &mut Cpu| {
                let mut value = binary::bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.mmu.write(value, cpu.registers.a);
                value -= 1;
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                Result::None
            },
        }),
        0x33 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC SP",
            handler: |cpu: &mut Cpu| {
                cpu.registers.sp = cpu.registers.sp + 1;
                Result::None
            },
        }),
        0x34 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "INC (HL)",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let result = increment_byte(cpu, cpu.mmu.read(addr));
                cpu.mmu.write(addr, result);
                Result::None
            },
        }),
        0x35 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "DEC (HL)",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let result = decrement_byte(cpu, cpu.mmu.read(addr));
                cpu.mmu.write(addr, result);
                Result::None
            },
        }),
        0x36 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD (HL),n",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.mmu.write(addr, cpu.get_attribute_for_op_code(0));
                Result::None
            },
        }),
        //TODO: 0x37
        0x38 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR C,n",
            handler: |cpu: &mut Cpu| {
                if jump_on_flag(cpu, Flag::C) {
                    return Result::ActionTaken;
                }
                Result::None
            },
        }),
        0x39 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,SP",
            handler: |cpu: &mut Cpu| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);

                let result = add_words(cpu, h_l, cpu.registers.sp);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                Result::None
            },
        }),
        0x3A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(HL-)",
            handler: |cpu: &mut Cpu| {
                let mut value = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.registers.a = cpu.mmu.read(value);
                value = value.wrapping_sub(1);
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                Result::None
            },
        }),
        0x3B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC SP",
            handler: |cpu: &mut Cpu| {
                cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
                Result::None
            },
        }),
        0x3C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = increment_byte(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x3D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = decrement_byte(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x3E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.get_attribute_for_op_code(0);
                Result::None
            },
        }),
        //TODO: 0x3F
        0x40 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.registers.b;
                Result::None
            },
        }),
        0x41 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.registers.c;
                Result::None
            },
        }),
        0x42 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.registers.d;
                Result::None
            },
        }),
        0x43 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.registers.e;
                Result::None
            },
        }),
        0x44 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.registers.h;
                Result::None
            },
        }),
        0x45 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.registers.l;
                Result::None
            },
        }),
        0x46 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD B,(HL)",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.registers.b = cpu.mmu.read(addr);
                Result::None
            },
        }),
        0x47 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.registers.a;
                Result::None
            },
        }),
        0x48 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.registers.b;
                Result::None
            },
        }),
        0x49 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.registers.c;
                Result::None
            },
        }),
        0x4A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.registers.d;
                Result::None
            },
        }),
        0x4B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.registers.e;
                Result::None
            },
        }),
        0x4C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.registers.h;
                Result::None
            },
        }),
        0x4D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.registers.l;
                Result::None
            },
        }),
        0x4E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD C,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = read_hl_addr(cpu);
                Result::None
            },
        }),
        0x4F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LC C,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = cpu.registers.a;
                Result::None
            },
        }),
        0x50 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.registers.b;
                Result::None
            },
        }),
        0x51 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.registers.c;
                Result::None
            },
        }),
        0x52 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.registers.d;
                Result::None
            },
        }),
        0x53 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.registers.e;
                Result::None
            },
        }),
        0x54 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.registers.h;
                Result::None
            },
        }),
        0x55 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.registers.l;
                Result::None
            },
        }),
        0x56 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD D,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = read_hl_addr(cpu);
                Result::None
            },
        }),
        0x57 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.registers.a;
                Result::None
            },
        }),
        0x58 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.registers.b;
                Result::None
            },
        }),
        0x59 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.registers.c;
                Result::None
            },
        }),
        0x5A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.registers.d;
                Result::None
            },
        }),
        0x5B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.registers.e;
                Result::None
            },
        }),
        0x5C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.registers.h;
                Result::None
            },
        }),
        0x5D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.registers.l;
                Result::None
            },
        }),
        0x5E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD E,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = read_hl_addr(cpu);
                Result::None
            },
        }),
        0x5F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = cpu.registers.a;
                Result::None
            },
        }),
        0x60 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.registers.b;
                Result::None
            },
        }),
        0x61 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.registers.c;
                Result::None
            },
        }),
        0x62 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.registers.d;
                Result::None
            },
        }),
        0x63 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.registers.e;
                Result::None
            },
        }),
        0x64 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.registers.h;
                Result::None
            },
        }),
        0x65 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.registers.l;
                Result::None
            },
        }),
        0x66 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD H,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = read_hl_addr(cpu);
                Result::None
            },
        }),
        0x67 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.registers.a;
                Result::None
            },
        }),
        0x68 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.registers.b;
                Result::None
            },
        }),
        0x69 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.registers.c;
                Result::None
            },
        }),
        0x6A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.registers.d;
                Result::None
            },
        }),
        0x6B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.registers.e;
                Result::None
            },
        }),
        0x6C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.registers.h;
                Result::None
            },
        }),
        0x6D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.registers.l;
                Result::None
            },
        }),
        0x6E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD L,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = read_hl_addr(cpu);
                Result::None
            },
        }),
        0x6F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = cpu.registers.a;
                Result::None
            },
        }),
        0x70 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),B",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.b,
                );
                Result::None
            },
        }),
        0x71 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),C",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.c,
                );
                Result::None
            },
        }),
        0x72 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),D",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.d,
                );
                Result::None
            },
        }),
        0x73 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),E",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.e,
                );
                Result::None
            },
        }),
        0x74 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),H",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.h,
                );
                Result::None
            },
        }),
        0x75 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),L",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.l,
                );
                Result::None
            },
        }),
        //TODO: 0x76
        0x77 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),A",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.a,
                );
                Result::None
            },
        }),
        0x78 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.b;
                Result::None
            },
        }),
        0x79 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.c;
                Result::None
            },
        }),
        0x7A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.d;
                Result::None
            },
        }),
        0x7B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.e;
                Result::None
            },
        }),
        0x7C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.h;
                Result::None
            },
        }),
        0x7D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.l;
                Result::None
            },
        }),
        0x7E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = read_hl_addr(cpu);
                Result::None
            },
        }),
        0x7F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.registers.l;
                Result::None
            },
        }),
        0x80 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes(cpu, cpu.registers.a, cpu.registers.b);
                Result::None
            },
        }),
        0x82 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0x85 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0x86 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD A,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0x87 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0x89 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes_carry(cpu, cpu.registers.a, cpu.registers.c);
                Result::None
            },
        }),
        0x8A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes_carry(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0x8B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes_carry(cpu, cpu.registers.a, cpu.registers.e);
                Result::None
            },
        }),
        0x8C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes_carry(cpu, cpu.registers.a, cpu.registers.h);
                Result::None
            },
        }),
        0x8D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes_carry(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0x8E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADC A,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes_carry(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0x8F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes_carry(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0x90 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, cpu.registers.b);
                Result::None
            },
        }),
        0x91 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, cpu.registers.c);
                Result::None
            },
        }),
        0x92 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0x93 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, cpu.registers.e);
                Result::None
            },
        }),
        0x94 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, cpu.registers.h);
                Result::None
            },
        }),
        0x95 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0x96 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SUB (HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0x97 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_byte(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0x98 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.b);
                Result::None
            },
        }),
        0x99 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.c);
                Result::None
            },
        }),
        0x9A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0x9B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.e);
                Result::None
            },
        }),
        0x9C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.h);
                Result::None
            },
        }),
        0x9D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0x9E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SBC A,(HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0x9F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0xA0 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.registers.b);
                Result::None
            },
        }),
        0xA1 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.registers.c);
                Result::None
            },
        }),
        0xA2 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0xA3 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.registers.e);
                Result::None
            },
        }),
        0xA4 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.registers.h);
                Result::None
            },
        }),
        0xA5 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0xA6 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "AND (HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0xA7 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0xA8 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.registers.b);
                Result::None
            },
        }),
        0xA9 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.registers.c);
                Result::None
            },
        }),
        0xAA => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0xAB => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.registers.e);
                Result::None
            },
        }),
        0xAC => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.registers.h);
                Result::None
            },
        }),
        0xAD => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0xAE => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "XOR (HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0xAF => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0xB0 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.registers.b);
                Result::None
            },
        }),
        0xB1 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.registers.c);
                Result::None
            },
        }),
        0xB2 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0xB3 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.registers.e);
                Result::None
            },
        }),
        0xB4 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.registers.h);
                Result::None
            },
        }),
        0xB5 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0xB6 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "OR (HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0xB7 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0xB8 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP B",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.registers.b);
                Result::None
            },
        }),
        0xB9 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP C",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.registers.c);
                Result::None
            },
        }),
        0xBA => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP D",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.registers.d);
                Result::None
            },
        }),
        0xBB => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP E",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.registers.e);
                Result::None
            },
        }),
        0xBC => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP H",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.registers.h);
                Result::None
            },
        }),
        0xBD => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP L",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.registers.l);
                Result::None
            },
        }),
        0xBE => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "CP (HL)",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, read_hl_addr(cpu));
                Result::None
            },
        }),
        0xBF => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP A",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.registers.a);
                Result::None
            },
        }),
        0xC0 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET NZ",
            handler: |cpu: &mut Cpu| {
                if !cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = cpu.mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return Result::JumpedActionTaken;
                }

                Result::None
            },
        }),
        0xC1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP BC",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = cpu.mmu.read(cpu.registers.sp);
                cpu.registers.c = cpu.mmu.read(cpu.registers.sp + 0x01);
                cpu.registers.sp += 2;
                Result::None
            },
        }),
        0xC2 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: Some(16),
            description: "JP NZ,a16",
            handler: |cpu: &mut Cpu| {
                if !cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = bytes_to_word(
                        cpu.get_attribute_for_op_code(1),
                        cpu.get_attribute_for_op_code(0),
                    );
                    return Result::JumpedActionTaken;
                }

                Result::None
            },
        }),
        0xC3 => Some(&Instruction {
            length: 3,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "JP a16",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(
                    cpu.get_attribute_for_op_code(1),
                    cpu.get_attribute_for_op_code(0),
                );
                cpu.registers.pc = addr;
                Result::Jumped
            },
        }),
        0xC5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH BC",
            handler: |cpu: &mut Cpu| {
                cpu.registers.sp -= 2;
                cpu.mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.b, cpu.registers.c),
                );
                Result::None
            },
        }),
        0xC6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD A,n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = add_bytes(cpu, cpu.registers.a, cpu.get_attribute_for_op_code(0));
                Result::None
            },
        }),
        0xC8 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET Z",
            handler: |cpu: &mut Cpu| {
                if cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = cpu.mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return Result::JumpedActionTaken;
                }

                Result::None
            },
        }),
        0xC9 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RET",
            handler: |cpu: &mut Cpu| {
                cpu.registers.pc = cpu.mmu.read_word(cpu.registers.sp);
                cpu.registers.sp += 2;
                Result::Jumped
            },
        }),
        0xCA => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(16),
            description: "JP Z,a16",
            handler: |cpu: &mut Cpu| {
                if cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = bytes_to_word(
                        cpu.get_attribute_for_op_code(1),
                        cpu.get_attribute_for_op_code(0),
                    );
                    return Result::JumpedActionTaken;
                }

                Result::None
            },
        }),
        0xCD => Some(&Instruction {
            length: 3,
            clock_cycles: 24,
            clock_cycles_condition: None,
            description: "CALL a16",
            handler: |cpu: &mut Cpu| {
                //Put address of next instruction onto stack and jump to aa
                cpu.registers.sp -= 2;
                cpu.mmu.write_word(cpu.registers.sp, cpu.registers.pc + 3);
                cpu.registers.pc = binary::bytes_to_word(
                    cpu.get_attribute_for_op_code(1),
                    cpu.get_attribute_for_op_code(0),
                );

                Result::Jumped
            },
        }),
        0xD0 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET NC",
            handler: |cpu: &mut Cpu| {
                if !cpu.registers.check_flag(Flag::C) {
                    cpu.registers.pc = cpu.mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return Result::JumpedActionTaken;
                }

                Result::None
            },
        }),
        0xD1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP DE",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = cpu.mmu.read(cpu.registers.sp);
                cpu.registers.e = cpu.mmu.read(cpu.registers.sp + 1);
                cpu.registers.sp += 2;
                Result::None
            },
        }),
        0xD5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH DE",
            handler: |cpu: &mut Cpu| {
                cpu.registers.sp -= 2;
                cpu.mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.d, cpu.registers.e),
                );
                Result::None
            },
        }),
        0xD6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SUB n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a =
                    substract_byte(cpu, cpu.registers.a, cpu.get_attribute_for_op_code(0));
                Result::None
            },
        }),
        0xD8 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET C",
            handler: |cpu: &mut Cpu| {
                if cpu.registers.check_flag(Flag::C) {
                    cpu.registers.pc = cpu.mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return Result::JumpedActionTaken;
                }

                Result::None
            },
        }),
        0xD9 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RETI",
            handler: |cpu: &mut Cpu| {
                cpu.interrupt_master_enabled = true;
                cpu.registers.pc = cpu.mmu.read_word(cpu.registers.sp);
                cpu.registers.sp += 2;
                Result::Jumped
            },
        }),
        0xE0 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LDH (a8),A",
            handler: |cpu: &mut Cpu| {
                cpu.mmu.write(
                    0xFF00 + cpu.get_attribute_for_op_code(0) as u16,
                    cpu.registers.a,
                );
                Result::None
            },
        }),
        0xE1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP HL",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = cpu.mmu.read(cpu.registers.sp);
                cpu.registers.l = cpu.mmu.read(cpu.registers.sp + 1);
                cpu.registers.sp += 2;
                Result::None
            },
        }),
        0xE2 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (FF00+C),A",
            handler: |cpu: &mut Cpu| {
                cpu.mmu
                    .write(0xFF00 + cpu.registers.c as u16, cpu.registers.a);
                Result::None
            },
        }),
        0xE5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH HL",
            handler: |cpu: &mut Cpu| {
                cpu.registers.sp -= 2;
                cpu.mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                );
                Result::None
            },
        }),
        0xE6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "AND n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = and_bytes(cpu, cpu.registers.a, cpu.get_attribute_for_op_code(0));
                Result::None
            },
        }),
        0xE9 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "JP (HL)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.pc = bytes_to_word(cpu.registers.h, cpu.registers.l);
                Result::Jumped
            },
        }),
        0xEA => Some(&Instruction {
            length: 3,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "LD (a16),A",
            handler: |cpu: &mut Cpu| {
                let addr = binary::bytes_to_word(
                    cpu.get_attribute_for_op_code(1),
                    cpu.get_attribute_for_op_code(0),
                );
                cpu.mmu.write(addr, cpu.registers.a);
                Result::None
            },
        }),
        0xEE => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "XOR n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = xor_bytes(cpu, cpu.registers.a, cpu.get_attribute_for_op_code(0));
                Result::None
            },
        }),
        0xEF => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 28H",
            handler: |cpu: &mut Cpu| {
                rst(cpu, 0x28);
                Result::Jumped
            },
        }),
        0xF0 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LDH A,(a8)",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu
                    .mmu
                    .read(0xFF00 + (cpu.get_attribute_for_op_code(0) as u16));
                Result::None
            },
        }),
        0xF1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP AF",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = cpu.mmu.read(cpu.registers.sp);
                cpu.registers.f = cpu.mmu.read(cpu.registers.sp + 1);
                cpu.registers.sp += 2;
                Result::None
            },
        }),
        0xF3 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DI",
            handler: |cpu: &mut Cpu| {
                cpu.interrupt_master_enabled = false;
                Result::None
            },
        }),
        0xF5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH AF",
            handler: |cpu: &mut Cpu| {
                cpu.registers.sp -= 2;
                cpu.mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.a, cpu.registers.f),
                );
                Result::None
            },
        }),
        0xF6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "OR n",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = or_bytes(cpu, cpu.registers.a, cpu.get_attribute_for_op_code(0));
                Result::None
            },
        }),
        0xFA => Some(&Instruction {
            length: 3,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "LD A,(a16)",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(
                    cpu.get_attribute_for_op_code(1),
                    cpu.get_attribute_for_op_code(0),
                );
                cpu.registers.a = cpu.mmu.read(addr);
                Result::None
            },
        }),
        0xFB => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "EI",
            handler: |cpu: &mut Cpu| {
                cpu.interrupt_master_enabled = true;
                Result::None
            },
        }),
        0xFE => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "CP d8",
            handler: |cpu: &mut Cpu| {
                compare_bytes(cpu, cpu.registers.a, cpu.get_attribute_for_op_code(0));
                Result::None
            },
        }),

        _ => None,
    }
}

fn get_cb_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        0x10 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = rotate_left(cpu, cpu.registers.b);
                Result::None
            },
        }),
        0x11 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = rotate_left(cpu, cpu.registers.c);
                Result::None
            },
        }),
        0x12 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = rotate_left(cpu, cpu.registers.d);
                Result::None
            },
        }),
        0x13 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = rotate_left(cpu, cpu.registers.e);
                Result::None
            },
        }),
        0x14 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = rotate_left(cpu, cpu.registers.h);
                Result::None
            },
        }),
        0x15 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = rotate_left(cpu, cpu.registers.l);
                Result::None
            },
        }),
        //TODO: 0x16
        0x17 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RL A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = rotate_left(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x27 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SLA A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = sla(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x30 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = swap_nibbles(cpu, cpu.registers.b);
                Result::None
            },
        }),
        0x31 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = swap_nibbles(cpu, cpu.registers.c);
                Result::None
            },
        }),
        0x32 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = swap_nibbles(cpu, cpu.registers.d);
                Result::None
            },
        }),
        0x33 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = swap_nibbles(cpu, cpu.registers.e);
                Result::None
            },
        }),
        0x34 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = swap_nibbles(cpu, cpu.registers.h);
                Result::None
            },
        }),
        0x35 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = swap_nibbles(cpu, cpu.registers.l);
                Result::None
            },
        }),
        0x37 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SWAP A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = swap_nibbles(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x38 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL B",
            handler: |cpu: &mut Cpu| {
                cpu.registers.b = srl(cpu, cpu.registers.b);
                Result::None
            },
        }),
        0x39 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL C",
            handler: |cpu: &mut Cpu| {
                cpu.registers.c = srl(cpu, cpu.registers.c);
                Result::None
            },
        }),
        0x3A => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL D",
            handler: |cpu: &mut Cpu| {
                cpu.registers.d = srl(cpu, cpu.registers.d);
                Result::None
            },
        }),
        0x3B => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL E",
            handler: |cpu: &mut Cpu| {
                cpu.registers.e = srl(cpu, cpu.registers.e);
                Result::None
            },
        }),
        0x3C => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL H",
            handler: |cpu: &mut Cpu| {
                cpu.registers.h = srl(cpu, cpu.registers.h);
                Result::None
            },
        }),
        0x3D => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL L",
            handler: |cpu: &mut Cpu| {
                cpu.registers.l = srl(cpu, cpu.registers.l);
                Result::None
            },
        }),
        0x3F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SRL A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = srl(cpu, cpu.registers.a);
                Result::None
            },
        }),
        0x40 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 0,B",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 0);
                Result::None
            },
        }),
        0x41 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 0,C",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.c, 0);
                Result::None
            },
        }),
        0x42 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 0,D",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.d, 0);
                Result::None
            },
        }),
        0x43 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 0,E",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.e, 0);
                Result::None
            },
        }),
        0x44 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 0,H",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.h, 0);
                Result::None
            },
        }),
        0x45 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 0,L",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.l, 0);
                Result::None
            },
        }),
        0x46 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "BIT 0,(HL)",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, read_hl_addr(cpu), 0);
                Result::None
            },
        }),
        0x47 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 0,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 0);
                Result::None
            },
        }),
        0x48 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 1,B",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 1);
                Result::None
            },
        }),
        0x49 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 1,C",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.c, 1);
                Result::None
            },
        }),
        0x4A => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 1,D",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.d, 1);
                Result::None
            },
        }),
        0x4B => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 1,E",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.e, 1);
                Result::None
            },
        }),
        0x4C => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 1,H",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.h, 1);
                Result::None
            },
        }),
        0x4D => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 1,L",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.l, 1);
                Result::None
            },
        }),
        0x4F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 1,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 1);
                Result::None
            },
        }),
        0x50 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,B",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 2);
                Result::None
            },
        }),
        0x51 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,C",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.c, 2);
                Result::None
            },
        }),
        0x52 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,D",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.d, 2);
                Result::None
            },
        }),
        0x53 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,E",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.e, 2);
                Result::None
            },
        }),
        0x54 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,H",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.h, 2);
                Result::None
            },
        }),
        0x55 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,L",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.l, 2);
                Result::None
            },
        }),
        0x56 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,(HL)",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, read_hl_addr(cpu), 2);
                Result::None
            },
        }),
        0x57 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 2,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 2);
                Result::None
            },
        }),
        0x58 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 3,B",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 3);
                Result::None
            },
        }),
        0x59 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 3,C",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.c, 3);
                Result::None
            },
        }),
        0x5A => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 3,D",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.d, 3);
                Result::None
            },
        }),
        0x5B => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 3,E",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.e, 3);
                Result::None
            },
        }),
        0x5C => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 3,H",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.h, 3);
                Result::None
            },
        }),
        0x5D => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 3,L",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.l, 3);
                Result::None
            },
        }),
        0x5F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 3,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 3);
                Result::None
            },
        }),
        0x60 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 4,B",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 4);
                Result::None
            },
        }),
        0x61 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 4,C",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.c, 4);
                Result::None
            },
        }),
        0x62 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 4,D",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.d, 4);
                Result::None
            },
        }),
        0x63 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 4,E",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.e, 4);
                Result::None
            },
        }),
        0x64 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 4,H",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.h, 4);
                Result::None
            },
        }),
        0x65 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 4,L",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.l, 4);
                Result::None
            },
        }),
        0x67 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 4,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 4);
                Result::None
            },
        }),
        0x68 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 5,B",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 5);
                Result::None
            },
        }),
        0x69 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 5,C",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.c, 5);
                Result::None
            },
        }),
        0x6A => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 5,D",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.d, 5);
                Result::None
            },
        }),
        0x6B => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 5,E",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.e, 5);
                Result::None
            },
        }),
        0x6C => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 5,H",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.h, 5);
                Result::None
            },
        }),
        0x6D => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 5,L",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.l, 5);
                Result::None
            },
        }),
        0x6F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 5,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 5);
                Result::None
            },
        }),
        0x70 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 6,B",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 6);
                Result::None
            },
        }),
        0x77 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 6,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 6);
                Result::None
            },
        }),
        0x78 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 7,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.b, 7);
                Result::None
            },
        }),
        0x86 => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RES 0,(HL)",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let result = reset_bit_in_byte(cpu.mmu.read(addr), 0);
                cpu.mmu.write(addr, result);
                Result::None
            },
        }),
        0x87 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "RES 0,A",
            handler: |cpu: &mut Cpu| {
                cpu.registers.a = reset_bit_in_byte(cpu.registers.a, 0);
                Result::None
            },
        }),
        0x7C => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 7,H",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.h, 7);
                Result::None
            },
        }),
        0x7E => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "BIT 7,(HL)",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, read_hl_addr(cpu), 7);
                Result::None
            },
        }),
        0x7F => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "BIT 7,A",
            handler: |cpu: &mut Cpu| {
                check_bit(cpu, cpu.registers.a, 7);
                Result::None
            },
        }),
        0xBE => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RES 7,(HL)",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let result = reset_bit_in_byte(cpu.mmu.read(addr), 7);
                cpu.mmu.write(addr, result);
                Result::None
            },
        }),
        0xFE => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "SET 7,(HL)",
            handler: |cpu: &mut Cpu| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let result = set_bit_in_byte(cpu.mmu.read(addr), 7);
                cpu.mmu.write(addr, result);
                Result::None
            },
        }),
        _ => None,
    }
}

fn check_bit(cpu: &mut Cpu, byte: u8, index: u8) {
    cpu.registers.clear_flag(Flag::Z);
    cpu.registers.clear_flag(Flag::N);

    if !binary::is_bit_set(byte, index) {
        cpu.registers.set_flag(Flag::Z)
    }

    cpu.registers.set_flag(Flag::H)
}

fn jump_on_flag_reset(cpu: &mut Cpu, flag: Flag) -> bool {
    if !cpu.registers.check_flag(flag) {
        jump_to_attribute_address(cpu);
        return true;
    }
    false
}

fn jump_on_flag(cpu: &mut Cpu, flag: Flag) -> bool {
    if cpu.registers.check_flag(flag) {
        jump_to_attribute_address(cpu);
        return true;
    }
    false
}

fn jump_to_attribute_address(cpu: &mut Cpu) {
    let destination = cpu.get_attribute_for_op_code(0);

    if destination < 127 {
        cpu.registers.pc += destination as u16;
    } else {
        cpu.registers.pc -= 256 - destination as u16;
    }
}

fn increment_byte(cpu: &mut Cpu, value: u8) -> u8 {
    let result = value.wrapping_add(1);

    cpu.registers.clear_flag(Flag::H);
    cpu.registers.clear_flag(Flag::Z);
    cpu.registers.clear_flag(Flag::N);

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    //Carry from bit 3 occured?
    if (result & 0xf) + (1 & 0xf) > 0xF {
        cpu.registers.set_flag(Flag::H);
    }

    result
}

fn decrement_byte(cpu: &mut Cpu, value: u8) -> u8 {
    let result = value.wrapping_sub(1);

    cpu.registers.clear_flag(Flag::Z);
    cpu.registers.clear_flag(Flag::H);

    cpu.registers.set_flag(Flag::N);

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if (value & 0xf) < (result & 0xf) {
        cpu.registers.set_flag(Flag::H);
    }

    result as u8
}

//Rotate left through carry flag
fn rotate_left_through_carry(cpu: &mut Cpu, value: u8) -> u8 {
    let mut result = value << 1;

    //Carry occcured so set LSB
    if cpu.registers.check_flag(Flag::C) {
        result |= 0x01;
    }

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    //Set carry flag if bit 7 is set in initial value because it will be shifted out so carry occurs
    if is_bit_set(value, 7) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn rotate_left(cpu: &mut Cpu, value: u8) -> u8 {
    let mut result = value << 1;

    //If bit 7 is set, set bit 0 because bit 7 will get shifted out
    if is_bit_set(value, 7) {
        result |= 0x01;
    }

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    //Set carry flag if bit 7 is set in initial value because it will be shifted out so carry occurs
    if is_bit_set(value, 7) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

//Basiclly substraction but result is thrown away
fn compare_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) {
    substract_byte(cpu, byte1, byte2);
}

fn substract_byte(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let result = byte1.wrapping_sub(byte2);

    cpu.registers.clear_flag(Flag::Z);
    cpu.registers.clear_flag(Flag::C);
    cpu.registers.clear_flag(Flag::H);

    cpu.registers.set_flag(Flag::N);

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    //Half carry
    if (byte1 & 0xf) < (byte2 & 0xf) {
        cpu.registers.set_flag(Flag::H);
    }

    //Carry
    if (byte1 & 0xff) < (byte2 & 0xff) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn substract_bytes_carry(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let mut result = byte1.wrapping_sub(byte2);
    let mut carry: u8 = 0;

    if cpu.registers.check_flag(Flag::C) {
        carry = 1;
    }

    result = result.wrapping_sub(carry);

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    //TODO: Check if this is correct
    if (byte1 & 0xf) < (byte2 & 0xf) + carry {
        cpu.registers.set_flag(Flag::H);
    }

    if (byte1 & 0xff) < (byte2 & 0xff) + carry {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn add_bytes_carry(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let mut result = byte1.wrapping_add(byte2);
    let mut carry: u8 = 0;

    if cpu.registers.check_flag(Flag::C) {
        carry = 1;
    }

    result = result.wrapping_add(carry);

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if ((byte1 & 0xf) + (byte2 & 0xf)) + carry > 0xF {
        cpu.registers.set_flag(Flag::H);
    }

    if byte1 as u16 + byte2 as u16 + carry as u16 > 0xFF {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn add_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let result = byte1.wrapping_add(byte2);

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if (byte1 & 0xf) + (byte2 & 0xf) > 0xF {
        cpu.registers.set_flag(Flag::H);
    }

    if byte1 as u16 + byte2 as u16 > 0xFF {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn add_words(cpu: &mut Cpu, word1: u16, word2: u16) -> u16 {
    let result = word1.wrapping_add(word2);

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if (word1 & 0x7FFF) + (word2 & 0x7FFF) > 0x7FFF {
        cpu.registers.set_flag(Flag::H);
    }

    if word1 as u32 + word2 as u32 > 0xFFFF {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn or_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let result = byte1 | byte2;

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    result
}

fn xor_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let result = byte1 ^ byte2;

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    result
}

fn and_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let result = byte1 & byte2;

    cpu.registers.clear_flag(Flag::Z);
    cpu.registers.clear_flag(Flag::C);
    cpu.registers.clear_flag(Flag::N);

    cpu.registers.set_flag(Flag::H);

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    result
}

fn swap_nibbles(cpu: &mut Cpu, byte: u8) -> u8 {
    let result = (byte >> 4) + (byte << 4);
    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    result
}

fn rst(cpu: &mut Cpu, param: u8) {
    cpu.registers.sp -= 2;
    cpu.mmu.write_word(cpu.registers.sp, cpu.registers.pc + 1);
    cpu.registers.pc = bytes_to_word(0x00, param);
}

fn sla(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.clear_all_flags();
    let result = value << 1;

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if is_bit_set(value, 7) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn srl(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.clear_all_flags();
    let result = value >> 1;

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if is_bit_set(value, 0) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

fn read_hl_addr(cpu: &Cpu) -> u8 {
    cpu.mmu
        .read(bytes_to_word(cpu.registers.h, cpu.registers.l))
}
