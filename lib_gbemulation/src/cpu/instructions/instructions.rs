use crate::cpu::cpu::{Cpu, InterruptAction};
use crate::cpu::instructions::functions::rotate_left;
use crate::cpu::instructions::{functions, read_hl_addr, ExecutionType, Instruction};
use crate::cpu::registers::Flag;
use crate::memory::mmu::{Mmu, Opcode};
use crate::util::binary;
use crate::util::binary::{bytes_to_word, word_to_bytes};

pub fn get_instruction(op_code: &u8) -> Option<&Instruction> {
    match op_code {
        0x00 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "NOP",
            handler: |_: &mut Cpu, _: &mut Mmu, _: &Opcode| ExecutionType::None,
        }),
        0x01 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD BC,nn",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.b = functions::get_argument(cpu, mmu, 1);
                cpu.registers.c = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x02 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (BC),A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.b, cpu.registers.c),
                    cpu.registers.a,
                );
                ExecutionType::None
            },
        }),
        0x03 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC BC",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let mut value = binary::bytes_to_word(cpu.registers.b, cpu.registers.c);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.b = byte1;
                cpu.registers.c = byte2;
                ExecutionType::None
            },
        }),
        0x04 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = functions::increment_byte(cpu, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0x05 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = functions::decrement_byte(cpu, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0x06 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD B,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.b = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x07 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "RLCA",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = rotate_left(cpu, cpu.registers.a, false);
                ExecutionType::None
            },
        }),
        0x08 => Some(&Instruction {
            length: 3,
            clock_cycles: 20,
            clock_cycles_condition: None,
            description: "LD aa,SP",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = bytes_to_word(
                    functions::get_argument(cpu, mmu, 1),
                    functions::get_argument(cpu, mmu, 0),
                );
                mmu.write_word(addr, cpu.registers.sp);
                ExecutionType::None
            },
        }),
        0x09 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,BC",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let b_c = bytes_to_word(cpu.registers.b, cpu.registers.c);

                let result = functions::add_words(cpu, h_l, b_c);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                ExecutionType::None
            },
        }),
        0x0A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(BC)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = mmu.read(bytes_to_word(cpu.registers.b, cpu.registers.c));
                ExecutionType::None
            },
        }),
        0x0B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC BC",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let mut value = bytes_to_word(cpu.registers.b, cpu.registers.c);
                value = value.wrapping_sub(1);
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.b = byte1;
                cpu.registers.c = byte2;
                ExecutionType::None
            },
        }),
        0x0C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = functions::increment_byte(cpu, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0x0D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = functions::decrement_byte(cpu, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0x0E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD C,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.c = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x0F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "RRCA",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::rotate_right(cpu, cpu.registers.a, false);
                ExecutionType::None
            },
        }),
        0x10 => Some(&Instruction {
            length: 2,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "STOP 0",
            handler: |_cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                //TODO: Implement stop
                ExecutionType::None
            },
        }),
        0x11 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD DE,nn",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.d = functions::get_argument(cpu, mmu, 1);
                cpu.registers.e = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x12 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (DE),A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.d, cpu.registers.e),
                    cpu.registers.a,
                );
                ExecutionType::None
            },
        }),
        0x13 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC DE",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let mut value = binary::bytes_to_word(cpu.registers.d, cpu.registers.e);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.d = byte1;
                cpu.registers.e = byte2;
                ExecutionType::None
            },
        }),
        0x14 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = functions::increment_byte(cpu, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0x15 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = functions::decrement_byte(cpu, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0x16 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD D,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.d = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x17 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "RLA",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::rotate_left_through_carry(cpu, cpu.registers.a, false);
                ExecutionType::None
            },
        }),
        0x18 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "JR r8",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::jump_to_attribute_address(cpu, mmu);
                ExecutionType::None
            },
        }),
        0x19 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,DE",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let d_e = bytes_to_word(cpu.registers.d, cpu.registers.e);

                let result = functions::add_words(cpu, h_l, d_e);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                ExecutionType::None
            },
        }),
        0x1A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(DE)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = mmu.read(binary::bytes_to_word(cpu.registers.d, cpu.registers.e));
                ExecutionType::None
            },
        }),
        0x1B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC DE",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let mut value = bytes_to_word(cpu.registers.d, cpu.registers.e);
                value = value.wrapping_sub(1);
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.d = byte1;
                cpu.registers.e = byte2;
                ExecutionType::None
            },
        }),
        0x1C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = functions::increment_byte(cpu, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0x1D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = functions::decrement_byte(cpu, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0x1E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD E,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.e = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x1F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "RRA",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::rotate_right_through_carry(cpu, cpu.registers.a, false);
                ExecutionType::None
            },
        }),
        0x20 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR NZ,r8",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if functions::jump_on_flag_reset(cpu, mmu, Flag::Z) {
                    return ExecutionType::ActionTaken;
                }
                ExecutionType::None
            },
        }),
        0x21 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD HL,nn",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.h = functions::get_argument(cpu, mmu, 1);
                cpu.registers.l = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x22 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL+),A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let mut value = binary::bytes_to_word(cpu.registers.h, cpu.registers.l);
                mmu.write(value, cpu.registers.a);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                ExecutionType::None
            },
        }),
        0x23 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC HL",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let mut value = binary::bytes_to_word(cpu.registers.h, cpu.registers.l);
                value = value.wrapping_add(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                ExecutionType::None
            },
        }),
        0x24 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = functions::increment_byte(cpu, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0x25 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = functions::decrement_byte(cpu, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0x26 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD H,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.h = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x27 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DAA",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.clear_flag(Flag::Z);

                //Flag N will be set after a substraction
                if cpu.registers.check_flag(Flag::N) {
                    //If carry has occured substract 6 from upper nibble
                    if cpu.registers.check_flag(Flag::C) {
                        cpu.registers.a = cpu.registers.a.wrapping_sub(0x60);
                    }
                    //If half-carry has occured substract 6 from lower nibble
                    if cpu.registers.check_flag(Flag::H) {
                        cpu.registers.a = cpu.registers.a.wrapping_sub(0x6);
                    }
                }
                //Flag N is reset after an addition
                else {
                    if cpu.registers.check_flag(Flag::C) || cpu.registers.a > 0x99 {
                        cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
                        cpu.registers.set_flag(Flag::C);
                    }

                    if cpu.registers.check_flag(Flag::H) || (cpu.registers.a & 0x0f) > 0x09 {
                        cpu.registers.a = cpu.registers.a.wrapping_add(0x6);
                    }
                }

                if cpu.registers.a == 0 {
                    cpu.registers.set_flag(Flag::Z);
                }

                cpu.registers.clear_flag(Flag::H);
                ExecutionType::None
            },
        }),
        0x28 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR Z,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if functions::jump_on_flag(cpu, mmu, Flag::Z) {
                    return ExecutionType::ActionTaken;
                }
                ExecutionType::None
            },
        }),
        0x29 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,HL",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);

                let result = functions::add_words(cpu, h_l, h_l);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                ExecutionType::None
            },
        }),
        0x2A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(HL+)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let mut value = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.registers.a = mmu.read(value);
                value = value.wrapping_add(1);
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                ExecutionType::None
            },
        }),
        0x2B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC HL",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let mut value = bytes_to_word(cpu.registers.h, cpu.registers.l);
                value = value.wrapping_sub(1);
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                ExecutionType::None
            },
        }),
        0x2C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = functions::increment_byte(cpu, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0x2D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = functions::decrement_byte(cpu, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0x2E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD L,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.l = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x2F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CPL",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.a ^ 0xFF;
                cpu.registers.set_flag(Flag::N);
                cpu.registers.set_flag(Flag::H);
                ExecutionType::None
            },
        }),
        0x30 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR NC,r8",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if functions::jump_on_flag_reset(cpu, mmu, Flag::C) {
                    return ExecutionType::ActionTaken;
                }
                ExecutionType::None
            },
        }),
        0x31 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD SP,nn",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.sp = binary::bytes_to_word(
                    functions::get_argument(cpu, mmu, 1),
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0x32 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL-), A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let mut value = binary::bytes_to_word(cpu.registers.h, cpu.registers.l);
                mmu.write(value, cpu.registers.a);
                value = value.wrapping_sub(1);
                let (byte1, byte2) = binary::word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                ExecutionType::None
            },
        }),
        0x33 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "INC SP",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
                ExecutionType::None
            },
        }),
        0x34 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "INC (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let result = functions::increment_byte(cpu, mmu.read(addr));
                mmu.write(addr, result);
                ExecutionType::None
            },
        }),
        0x35 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "DEC (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                let result = functions::decrement_byte(cpu, mmu.read(addr));
                mmu.write(addr, result);
                ExecutionType::None
            },
        }),
        0x36 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD (HL),n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                mmu.write(addr, functions::get_argument(cpu, mmu, 0));
                ExecutionType::None
            },
        }),
        0x37 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SCF",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.set_flag(Flag::C);
                cpu.registers.clear_flag(Flag::N);
                cpu.registers.clear_flag(Flag::H);
                ExecutionType::None
            },
        }),
        0x38 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: Some(12),
            description: "JR C,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if functions::jump_on_flag(cpu, mmu, Flag::C) {
                    return ExecutionType::ActionTaken;
                }
                ExecutionType::None
            },
        }),
        0x39 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD HL,SP",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);

                let result = functions::add_words(cpu, h_l, cpu.registers.sp);
                let (byte1, byte2) = word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                ExecutionType::None
            },
        }),
        0x3A => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(HL-)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let mut value = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.registers.a = mmu.read(value);
                value = value.wrapping_sub(1);
                let (byte1, byte2) = word_to_bytes(value);
                cpu.registers.h = byte1;
                cpu.registers.l = byte2;
                ExecutionType::None
            },
        }),
        0x3B => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "DEC SP",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
                ExecutionType::None
            },
        }),
        0x3C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "INC A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::increment_byte(cpu, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0x3D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DEC A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::decrement_byte(cpu, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0x3E => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::get_argument(cpu, mmu, 0);
                ExecutionType::None
            },
        }),
        0x3F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CCF",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                if cpu.registers.check_flag(Flag::C) {
                    cpu.registers.clear_flag(Flag::C);
                } else {
                    cpu.registers.set_flag(Flag::C);
                }

                cpu.registers.clear_flag(Flag::N);
                cpu.registers.clear_flag(Flag::H);
                ExecutionType::None
            },
        }),
        0x40 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = cpu.registers.b;
                ExecutionType::None
            },
        }),
        0x41 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = cpu.registers.c;
                ExecutionType::None
            },
        }),
        0x42 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = cpu.registers.d;
                ExecutionType::None
            },
        }),
        0x43 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = cpu.registers.e;
                ExecutionType::None
            },
        }),
        0x44 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = cpu.registers.h;
                ExecutionType::None
            },
        }),
        0x45 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = cpu.registers.l;
                ExecutionType::None
            },
        }),
        0x46 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD B,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.registers.b = mmu.read(addr);
                ExecutionType::None
            },
        }),
        0x47 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD B,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.b = cpu.registers.a;
                ExecutionType::None
            },
        }),
        0x48 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = cpu.registers.b;
                ExecutionType::None
            },
        }),
        0x49 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = cpu.registers.c;
                ExecutionType::None
            },
        }),
        0x4A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = cpu.registers.d;
                ExecutionType::None
            },
        }),
        0x4B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = cpu.registers.e;
                ExecutionType::None
            },
        }),
        0x4C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = cpu.registers.h;
                ExecutionType::None
            },
        }),
        0x4D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD C,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = cpu.registers.l;
                ExecutionType::None
            },
        }),
        0x4E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD C,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.c = read_hl_addr(cpu, mmu);
                ExecutionType::None
            },
        }),
        0x4F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LC C,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.c = cpu.registers.a;
                ExecutionType::None
            },
        }),
        0x50 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = cpu.registers.b;
                ExecutionType::None
            },
        }),
        0x51 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = cpu.registers.c;
                ExecutionType::None
            },
        }),
        0x52 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = cpu.registers.d;
                ExecutionType::None
            },
        }),
        0x53 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = cpu.registers.e;
                ExecutionType::None
            },
        }),
        0x54 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = cpu.registers.h;
                ExecutionType::None
            },
        }),
        0x55 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = cpu.registers.l;
                ExecutionType::None
            },
        }),
        0x56 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD D,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.d = read_hl_addr(cpu, mmu);
                ExecutionType::None
            },
        }),
        0x57 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD D,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.d = cpu.registers.a;
                ExecutionType::None
            },
        }),
        0x58 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = cpu.registers.b;
                ExecutionType::None
            },
        }),
        0x59 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = cpu.registers.c;
                ExecutionType::None
            },
        }),
        0x5A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = cpu.registers.d;
                ExecutionType::None
            },
        }),
        0x5B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = cpu.registers.e;
                ExecutionType::None
            },
        }),
        0x5C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = cpu.registers.h;
                ExecutionType::None
            },
        }),
        0x5D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = cpu.registers.l;
                ExecutionType::None
            },
        }),
        0x5E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD E,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.e = read_hl_addr(cpu, mmu);
                ExecutionType::None
            },
        }),
        0x5F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD E,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.e = cpu.registers.a;
                ExecutionType::None
            },
        }),
        0x60 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = cpu.registers.b;
                ExecutionType::None
            },
        }),
        0x61 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = cpu.registers.c;
                ExecutionType::None
            },
        }),
        0x62 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = cpu.registers.d;
                ExecutionType::None
            },
        }),
        0x63 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = cpu.registers.e;
                ExecutionType::None
            },
        }),
        0x64 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = cpu.registers.h;
                ExecutionType::None
            },
        }),
        0x65 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = cpu.registers.l;
                ExecutionType::None
            },
        }),
        0x66 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD H,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.h = read_hl_addr(cpu, mmu);
                ExecutionType::None
            },
        }),
        0x67 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD H,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.h = cpu.registers.a;
                ExecutionType::None
            },
        }),
        0x68 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = cpu.registers.b;
                ExecutionType::None
            },
        }),
        0x69 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = cpu.registers.c;
                ExecutionType::None
            },
        }),
        0x6A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = cpu.registers.d;
                ExecutionType::None
            },
        }),
        0x6B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = cpu.registers.e;
                ExecutionType::None
            },
        }),
        0x6C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = cpu.registers.h;
                ExecutionType::None
            },
        }),
        0x6D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = cpu.registers.l;
                ExecutionType::None
            },
        }),
        0x6E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD L,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.l = read_hl_addr(cpu, mmu);
                ExecutionType::None
            },
        }),
        0x6F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD L,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.l = cpu.registers.a;
                ExecutionType::None
            },
        }),
        0x70 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),B",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.b,
                );
                ExecutionType::None
            },
        }),
        0x71 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),C",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.c,
                );
                ExecutionType::None
            },
        }),
        0x72 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),D",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.d,
                );
                ExecutionType::None
            },
        }),
        0x73 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),E",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.e,
                );
                ExecutionType::None
            },
        }),
        0x74 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.h,
                );
                ExecutionType::None
            },
        }),
        0x75 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),L",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.l,
                );
                ExecutionType::None
            },
        }),
        0x76 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "HALT",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                //TODO: Halt instructions slows emulation down. Check why. Also HALT Bug seems to be implemented wrong
                cpu.is_halted = true;
                ExecutionType::None
            },
        }),
        0x77 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (HL),A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                    cpu.registers.a,
                );
                ExecutionType::None
            },
        }),
        0x78 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.b;
                ExecutionType::None
            },
        }),
        0x79 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.c;
                ExecutionType::None
            },
        }),
        0x7A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.d;
                ExecutionType::None
            },
        }),
        0x7B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.e;
                ExecutionType::None
            },
        }),
        0x7C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.h;
                ExecutionType::None
            },
        }),
        0x7D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.l;
                ExecutionType::None
            },
        }),
        0x7E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = read_hl_addr(cpu, mmu);
                ExecutionType::None
            },
        }),
        0x7F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "LD A,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = cpu.registers.a;
                ExecutionType::None
            },
        }),
        0x80 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0x81 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0x82 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0x83 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0x84 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0x85 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0x86 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD A,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::add_bytes(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0x87 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADD A,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0x88 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0x89 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0x8A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0x8B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0x8C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0x8D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0x8E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADC A,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::add_bytes_carry(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0x8F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "ADC A,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0x90 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0x91 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0x92 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0x93 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0x94 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0x95 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0x96 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SUB (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_byte(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0x97 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SUB A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0x98 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0x99 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0x9A => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0x9B => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0x9C => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0x9D => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0x9E => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SBC A,(HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0x9F => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "SBC A,A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::substract_bytes_carry(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0xA0 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0xA1 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0xA2 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0xA3 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0xA4 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0xA5 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0xA6 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "AND (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::and_bytes(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0xA7 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "AND A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0xA8 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0xA9 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0xAA => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0xAB => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0xAC => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0xAD => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0xAE => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "XOR (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::xor_bytes(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0xAF => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "XOR A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0xB0 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0xB1 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0xB2 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0xB3 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0xB4 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0xB5 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0xB6 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "OR (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0xB7 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "OR A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::or_bytes(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0xB8 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP B",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, cpu.registers.b);
                ExecutionType::None
            },
        }),
        0xB9 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP C",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, cpu.registers.c);
                ExecutionType::None
            },
        }),
        0xBA => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP D",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, cpu.registers.d);
                ExecutionType::None
            },
        }),
        0xBB => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP E",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, cpu.registers.e);
                ExecutionType::None
            },
        }),
        0xBC => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP H",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, cpu.registers.h);
                ExecutionType::None
            },
        }),
        0xBD => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP L",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, cpu.registers.l);
                ExecutionType::None
            },
        }),
        0xBE => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "CP (HL)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, read_hl_addr(cpu, mmu));
                ExecutionType::None
            },
        }),
        0xBF => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "CP A",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(cpu, cpu.registers.a, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0xC0 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET NZ",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if !cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xC1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP BC",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.b = mmu.read(cpu.registers.sp + 0x01);
                cpu.registers.c = mmu.read(cpu.registers.sp);
                cpu.registers.sp += 2;
                ExecutionType::None
            },
        }),
        0xC2 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(16),
            description: "JP NZ,a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if !cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = bytes_to_word(
                        functions::get_argument(cpu, mmu, 1),
                        functions::get_argument(cpu, mmu, 0),
                    );
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xC3 => Some(&Instruction {
            length: 3,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "JP a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = bytes_to_word(
                    functions::get_argument(cpu, mmu, 1),
                    functions::get_argument(cpu, mmu, 0),
                );
                cpu.registers.pc = addr;
                ExecutionType::Jumped
            },
        }),
        0xC4 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(24),
            description: "CALL NZ a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if !cpu.registers.check_flag(Flag::Z) {
                    functions::call(cpu, mmu);
                    return ExecutionType::JumpedActionTaken;
                }
                ExecutionType::None
            },
        }),
        0xC5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH BC",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.sp -= 2;
                mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.b, cpu.registers.c),
                );
                ExecutionType::None
            },
        }),
        0xC6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADD A,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes(
                    cpu,
                    cpu.registers.a,
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0xC7 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 00H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x00);
                ExecutionType::Jumped
            },
        }),
        0xC8 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET Z",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xC9 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RET",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.pc = mmu.read_word(cpu.registers.sp);
                cpu.registers.sp = cpu.registers.sp.wrapping_add(2);
                ExecutionType::Jumped
            },
        }),
        0xCA => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(16),
            description: "JP Z,a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if cpu.registers.check_flag(Flag::Z) {
                    cpu.registers.pc = bytes_to_word(
                        functions::get_argument(cpu, mmu, 1),
                        functions::get_argument(cpu, mmu, 0),
                    );
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xCC => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(24),
            description: "CALL Z a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if cpu.registers.check_flag(Flag::Z) {
                    functions::call(cpu, mmu);
                    return ExecutionType::JumpedActionTaken;
                }
                ExecutionType::None
            },
        }),
        0xCD => Some(&Instruction {
            length: 3,
            clock_cycles: 24,
            clock_cycles_condition: None,
            description: "CALL a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::call(cpu, mmu);
                ExecutionType::Jumped
            },
        }),
        0xCE => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "ADC A,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::add_bytes_carry(
                    cpu,
                    cpu.registers.a,
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0xCF => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 08H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x08);
                ExecutionType::Jumped
            },
        }),
        0xD0 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET NC",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if !cpu.registers.check_flag(Flag::C) {
                    cpu.registers.pc = mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xD1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP DE",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.d = mmu.read(cpu.registers.sp + 1);
                cpu.registers.e = mmu.read(cpu.registers.sp);
                cpu.registers.sp += 2;
                ExecutionType::None
            },
        }),
        0xD2 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(16),
            description: "JP NC,a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if !cpu.registers.check_flag(Flag::C) {
                    cpu.registers.pc = bytes_to_word(
                        functions::get_argument(cpu, mmu, 1),
                        functions::get_argument(cpu, mmu, 0),
                    );
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xD4 => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(24),
            description: "CALL NC a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if !cpu.registers.check_flag(Flag::C) {
                    functions::call(cpu, mmu);
                    return ExecutionType::JumpedActionTaken;
                }
                ExecutionType::None
            },
        }),
        0xD5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH DE",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.sp -= 2;
                mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.d, cpu.registers.e),
                );
                ExecutionType::None
            },
        }),
        0xD6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SUB n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_byte(
                    cpu,
                    cpu.registers.a,
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0xD7 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 10H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x10);
                ExecutionType::Jumped
            },
        }),
        0xD8 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: Some(20),
            description: "RET C",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if cpu.registers.check_flag(Flag::C) {
                    cpu.registers.pc = mmu.read_word(cpu.registers.sp);
                    cpu.registers.sp += 2;
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xD9 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RETI",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.interrupt_action = InterruptAction::Enable;
                cpu.registers.pc = mmu.read_word(cpu.registers.sp);
                cpu.registers.sp += 2;
                ExecutionType::Jumped
            },
        }),
        0xDA => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(16),
            description: "JP C,a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if cpu.registers.check_flag(Flag::C) {
                    cpu.registers.pc = bytes_to_word(
                        functions::get_argument(cpu, mmu, 1),
                        functions::get_argument(cpu, mmu, 0),
                    );
                    return ExecutionType::JumpedActionTaken;
                }

                ExecutionType::None
            },
        }),
        0xDC => Some(&Instruction {
            length: 3,
            clock_cycles: 12,
            clock_cycles_condition: Some(24),
            description: "CALL C a16",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                if cpu.registers.check_flag(Flag::C) {
                    functions::call(cpu, mmu);
                    return ExecutionType::JumpedActionTaken;
                }
                ExecutionType::None
            },
        }),
        0xDE => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "SBC A,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::substract_bytes_carry(
                    cpu,
                    cpu.registers.a,
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0xDF => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 18H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x18);
                ExecutionType::Jumped
            },
        }),
        0xE0 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LDH (a8),A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(
                    0xFF00 + functions::get_argument(cpu, mmu, 0) as u16,
                    cpu.registers.a,
                );
                ExecutionType::None
            },
        }),
        0xE1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP HL",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.h = mmu.read(cpu.registers.sp + 1);
                cpu.registers.l = mmu.read(cpu.registers.sp);
                cpu.registers.sp += 2;
                ExecutionType::None
            },
        }),
        0xE2 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD (FF00+C),A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                mmu.write(0xFF00 + cpu.registers.c as u16, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0xE5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH HL",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.sp -= 2;
                mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.h, cpu.registers.l),
                );
                ExecutionType::None
            },
        }),
        0xE6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "AND n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::and_bytes(
                    cpu,
                    cpu.registers.a,
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0xE7 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 20H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x20);
                ExecutionType::Jumped
            },
        }),
        0xE8 => Some(&Instruction {
            length: 2,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "ADD SP,n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.sp = functions::add_to_sp(cpu, functions::get_argument(cpu, mmu, 0));
                ExecutionType::None
            },
        }),
        0xE9 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "JP (HL)",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.registers.pc = bytes_to_word(cpu.registers.h, cpu.registers.l);
                ExecutionType::Jumped
            },
        }),
        0xEA => Some(&Instruction {
            length: 3,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "LD (a16),A",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = binary::bytes_to_word(
                    functions::get_argument(cpu, mmu, 1),
                    functions::get_argument(cpu, mmu, 0),
                );
                mmu.write(addr, cpu.registers.a);
                ExecutionType::None
            },
        }),
        0xEE => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "XOR n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = functions::xor_bytes(
                    cpu,
                    cpu.registers.a,
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0xEF => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 28H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x28);
                ExecutionType::Jumped
            },
        }),
        0xF0 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LDH A,(a8)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = mmu.read(0xFF00 + (functions::get_argument(cpu, mmu, 0) as u16));
                ExecutionType::None
            },
        }),
        0xF1 => Some(&Instruction {
            length: 1,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "POP AF",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = mmu.read(cpu.registers.sp + 1);
                //Only the upper 4 bits are writable
                cpu.registers.f = 0xF0 & mmu.read(cpu.registers.sp);
                cpu.registers.sp += 2;
                ExecutionType::None
            },
        }),
        0xF2 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD A,(C)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a = mmu.read(0xFF00 + cpu.registers.c as u16);
                ExecutionType::None
            },
        }),
        0xF3 => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "DI",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.interrupt_action = InterruptAction::Disable;
                ExecutionType::None
            },
        }),
        0xF5 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "PUSH AF",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.sp -= 2;
                mmu.write_word(
                    cpu.registers.sp,
                    binary::bytes_to_word(cpu.registers.a, cpu.registers.f),
                );
                ExecutionType::None
            },
        }),
        0xF6 => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "OR n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                cpu.registers.a =
                    functions::or_bytes(cpu, cpu.registers.a, functions::get_argument(cpu, mmu, 0));
                ExecutionType::None
            },
        }),
        0xF7 => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 30H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x30);
                ExecutionType::Jumped
            },
        }),
        0xF8 => Some(&Instruction {
            length: 2,
            clock_cycles: 12,
            clock_cycles_condition: None,
            description: "LD HL,SP+n",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let result = functions::add_to_sp(cpu, functions::get_argument(cpu, mmu, 0));
                let (byte1, byte2) = binary::word_to_bytes(result);

                cpu.registers.h = byte1;
                cpu.registers.l = byte2;

                ExecutionType::None
            },
        }),
        0xF9 => Some(&Instruction {
            length: 1,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "LD SP,HL",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                let h_l = bytes_to_word(cpu.registers.h, cpu.registers.l);
                cpu.registers.sp = h_l;

                ExecutionType::None
            },
        }),
        0xFA => Some(&Instruction {
            length: 3,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "LD A,(a16)",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                let addr = bytes_to_word(
                    functions::get_argument(cpu, mmu, 1),
                    functions::get_argument(cpu, mmu, 0),
                );
                cpu.registers.a = mmu.read(addr);
                ExecutionType::None
            },
        }),
        0xFB => Some(&Instruction {
            length: 1,
            clock_cycles: 4,
            clock_cycles_condition: None,
            description: "EI",
            handler: |cpu: &mut Cpu, _: &mut Mmu, _: &Opcode| {
                cpu.interrupt_action = InterruptAction::Enable;
                ExecutionType::None
            },
        }),
        0xFE => Some(&Instruction {
            length: 2,
            clock_cycles: 8,
            clock_cycles_condition: None,
            description: "CP d8",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::compare_bytes(
                    cpu,
                    cpu.registers.a,
                    functions::get_argument(cpu, mmu, 0),
                );
                ExecutionType::None
            },
        }),
        0xFF => Some(&Instruction {
            length: 1,
            clock_cycles: 16,
            clock_cycles_condition: None,
            description: "RST 38H",
            handler: |cpu: &mut Cpu, mmu: &mut Mmu, _: &Opcode| {
                functions::rst(cpu, mmu, 0x38);
                ExecutionType::Jumped
            },
        }),
        _ => None,
    }
}
