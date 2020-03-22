use crate::cpu::cpu::Cpu;
use crate::cpu::registers::Flag;
use crate::memory::mmu::Mmu;
use crate::util::binary::{bytes_to_word, is_bit_set};

pub fn rotate_left(cpu: &mut Cpu, value: u8, check_for_zero: bool) -> u8 {
    let mut result = value << 1;

    //If bit 7 is set, set bit 0 because bit 7 will get shifted out
    if is_bit_set(&value, 7) {
        result |= 0x01;
    }

    cpu.registers.clear_all_flags();

    if result == 0 && check_for_zero {
        cpu.registers.set_flag(Flag::Z);
    }

    //Set carry flag if bit 7 is set in initial value because it will be shifted out so carry occurs
    if is_bit_set(&value, 7) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

//Rotate left through carry flag
pub fn rotate_left_through_carry(cpu: &mut Cpu, value: u8, check_for_zero: bool) -> u8 {
    let mut result = value << 1;

    //Carry occcured so set LSB
    if cpu.registers.check_flag(Flag::C) {
        result |= 0x01;
    }

    cpu.registers.clear_all_flags();

    if result == 0 && check_for_zero {
        cpu.registers.set_flag(Flag::Z);
    }

    //Set carry flag if bit 7 is set in initial value because it will be shifted out so carry occurs
    if is_bit_set(&value, 7) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn rotate_right(cpu: &mut Cpu, value: u8, check_for_zero: bool) -> u8 {
    let mut result = value >> 1;

    //If bit 0 is set, set bit 7 because bit 0 will get shifted out
    if is_bit_set(&value, 0) {
        result |= 0x80;
    }

    cpu.registers.clear_all_flags();

    if result == 0 && check_for_zero {
        cpu.registers.set_flag(Flag::Z);
    }

    //Set carry flag if bit 0 is set in initial value because it will be shifted out so carry occurs
    if is_bit_set(&value, 0) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn rotate_right_through_carry(cpu: &mut Cpu, value: u8, check_for_zero: bool) -> u8 {
    let mut result = value >> 1;

    if cpu.registers.check_flag(Flag::C) {
        result |= 0x80;
    }

    cpu.registers.clear_all_flags();

    if result == 0 && check_for_zero {
        cpu.registers.set_flag(Flag::Z);
    }

    //Set carry flag if bit 0 is set in initial value because it will be shifted out so carry occurs
    if is_bit_set(&value, 0) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn check_bit(cpu: &mut Cpu, byte: u8, index: u8) {
    cpu.registers.clear_flag(Flag::Z);
    cpu.registers.clear_flag(Flag::N);

    if !is_bit_set(&byte, index) {
        cpu.registers.set_flag(Flag::Z)
    }

    cpu.registers.set_flag(Flag::H)
}

pub fn swap_nibbles(cpu: &mut Cpu, byte: u8) -> u8 {
    let result = (byte >> 4) + (byte << 4);
    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    result
}

pub fn sla(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.clear_all_flags();
    let result = value << 1;

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if is_bit_set(&value, 7) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn sra(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.clear_all_flags();
    let mut result = value >> 1;

    if is_bit_set(&value, 7) {
        result |= 0x80;
    }

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if is_bit_set(&value, 0) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn srl(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.clear_all_flags();
    let result = value >> 1;

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if is_bit_set(&value, 0) {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn jump_on_flag_reset(cpu: &mut Cpu, mmu: &Mmu, flag: Flag) -> bool {
    if !cpu.registers.check_flag(flag) {
        jump_to_attribute_address(cpu, mmu);
        return true;
    }
    false
}

pub fn jump_on_flag(cpu: &mut Cpu, mmu: &Mmu, flag: Flag) -> bool {
    if cpu.registers.check_flag(flag) {
        jump_to_attribute_address(cpu, mmu);
        return true;
    }
    false
}

pub fn jump_to_attribute_address(cpu: &mut Cpu, mmu: &Mmu) {
    let destination = get_argument(cpu, mmu, 0);

    cpu.registers.pc = cpu.registers.pc.wrapping_add((destination as i8) as u16);
}

pub fn increment_byte(cpu: &mut Cpu, value: u8) -> u8 {
    let result = value.wrapping_add(1);

    cpu.registers.clear_flag(Flag::H);
    cpu.registers.clear_flag(Flag::Z);
    cpu.registers.clear_flag(Flag::N);

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    //Carry from bit 3 occured?
    if (value & 0xf) + (1 & 0xf) > 0xF {
        cpu.registers.set_flag(Flag::H);
    }

    result
}

pub fn decrement_byte(cpu: &mut Cpu, value: u8) -> u8 {
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

//Basiclly substraction but result is thrown away
pub fn compare_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) {
    substract_byte(cpu, byte1, byte2);
}

pub fn substract_byte(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
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

pub fn substract_bytes_carry(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let mut result = byte1.wrapping_sub(byte2);
    let mut carry: u8 = 0;

    if cpu.registers.check_flag(Flag::C) {
        carry = 1;
    }

    result = result.wrapping_sub(carry);

    cpu.registers.clear_all_flags();
    cpu.registers.set_flag(Flag::N);

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    if (byte1 & 0xf) < (byte2 & 0xf) + carry {
        cpu.registers.set_flag(Flag::H);
    }

    if (byte1 as u16 & 0xff) < (byte2 as u16 & 0xff) + carry as u16 {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn add_bytes_carry(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
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

pub fn add_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
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

pub fn add_words(cpu: &mut Cpu, word1: u16, word2: u16) -> u16 {
    let result = word1.wrapping_add(word2);

    cpu.registers.clear_flag(Flag::H);
    cpu.registers.clear_flag(Flag::N);
    cpu.registers.clear_flag(Flag::C);

    if (word1 & 0xFFF) + (word2 & 0xFFF) > 0xFFF {
        cpu.registers.set_flag(Flag::H);
    }

    if word1 as u32 + word2 as u32 > 0xFFFF {
        cpu.registers.set_flag(Flag::C);
    }

    result
}

pub fn add_to_sp(cpu: &mut Cpu, value: u8) -> u16 {
    let value_signed = value as i8;
    let result = cpu.registers.sp.wrapping_add(value_signed as u16);

    cpu.registers.clear_all_flags();

    if value_signed > 0 {
        //Addition
        if (cpu.registers.sp & 0xFF) + value as u16 > 0xFF {
            cpu.registers.set_flag(Flag::C);
        }

        if (cpu.registers.sp & 0xF) + (value as u16 & 0xF) > 0xF {
            cpu.registers.set_flag(Flag::H);
        }
    } else {
        //Substraction
        if result & 0xFF < cpu.registers.sp & 0xFF {
            cpu.registers.set_flag(Flag::C);
        }

        if result & 0xF < cpu.registers.sp & 0xF {
            cpu.registers.set_flag(Flag::H);
        }
    }

    result
}

pub fn or_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let result = byte1 | byte2;

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    result
}

pub fn xor_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
    let result = byte1 ^ byte2;

    cpu.registers.clear_all_flags();

    if result == 0 {
        cpu.registers.set_flag(Flag::Z);
    }

    result
}

pub fn and_bytes(cpu: &mut Cpu, byte1: u8, byte2: u8) -> u8 {
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

pub fn rst(cpu: &mut Cpu, mmu: &mut Mmu, param: u8) {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(2);
    mmu.write_word(cpu.registers.sp, cpu.registers.pc + 1);
    cpu.registers.pc = bytes_to_word(0x00, param);
}

pub fn call(cpu: &mut Cpu, mmu: &mut Mmu) {
    //Put address of next instruction onto stack and jump to aa
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(2);
    mmu.write_word(cpu.registers.sp, cpu.registers.pc + 3);
    cpu.registers.pc = bytes_to_word(get_argument(cpu, mmu, 1), get_argument(cpu, mmu, 0));
}

pub fn get_argument(cpu: &Cpu, mmu: &Mmu, index: u16) -> u8 {
    mmu.read(cpu.registers.pc.wrapping_add(index + 1))
}
