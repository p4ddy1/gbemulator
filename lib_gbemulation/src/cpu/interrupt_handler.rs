use crate::cpu::cpu::Cpu;
use crate::memory::interrupts::Interrupt;
use crate::memory::mmu::Mmu;

pub fn handle_interrupts(cpu: &mut Cpu, mmu: &mut Mmu) -> Option<u8> {
    if handle_interrupt(cpu, mmu, &Interrupt::Vblank, 0x0040) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::LcdStat, 0x0048) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::Timer, 0x0050) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::Serial, 0x0058) {
        return Some(12);
    }

    if handle_interrupt(cpu, mmu, &Interrupt::Joypad, 0x0060) {
        return Some(12);
    }

    return None;
}

fn handle_interrupt(cpu: &mut Cpu, mmu: &mut Mmu, interrupt: &Interrupt, isr_address: u16) -> bool {
    if !mmu.interrupts.interrupt_fired(interrupt) {
        return false;
    }
    cpu.interrupt_master_enabled = false;
    cpu.registers.sp -= 2;
    mmu.write_word(cpu.registers.sp, cpu.registers.pc);
    cpu.registers.pc = isr_address;
    mmu.interrupts.reset_interrupt(interrupt);
    true
}
