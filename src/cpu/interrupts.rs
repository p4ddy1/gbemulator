use crate::cpu::cpu::Cpu;
use crate::memory::mmu;
use crate::util::binary::is_bit_set;

pub fn handle_interrupts(cpu: &mut Cpu) -> Option<u8> {
    if !cpu.interrupt_master_enabled {
        return None;
    }

    let enabled_interrupts = cpu.mmu.read(mmu::INTERRUPT_ENABLE_ADDRESS);
    let fired_interrupts = cpu.mmu.read(mmu::INTERRUPT_FLAGS_ADDRESS);

    if vblank_interrupt_occured(enabled_interrupts, fired_interrupts) {
        handle_vblank(cpu);
        //Reset fired interrupts
        cpu.mmu.write(
            mmu::INTERRUPT_FLAGS_ADDRESS,
            fired_interrupts & (255 - 0x01),
        );
        //12 clock cycles
        return Some(12);
    }

    None
}

fn vblank_interrupt_occured(enabled_interrupts: u8, fired_interrupts: u8) -> bool {
    is_bit_set(enabled_interrupts, 0) && is_bit_set(fired_interrupts, 0)
}

fn handle_vblank(cpu: &mut Cpu) {
    cpu.interrupt_master_enabled = false;
    cpu.registers.sp -= 2;
    cpu.mmu.write_word(cpu.registers.sp, cpu.registers.pc);
    cpu.registers.pc = 0x0040;
}
