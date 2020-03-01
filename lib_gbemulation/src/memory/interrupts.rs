use crate::util::binary::{is_bit_set, reset_bit_in_byte, set_bit_in_byte};

pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
pub const INTERRUPT_FLAGS_ADDRESS: u16 = 0xFF0F;

#[derive(Copy, Clone)]
pub enum Interrupt {
    Vblank = 0,
    LcdStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

pub struct InterruptState {
    pub interrupt_flags: u8,
    pub interrupts_enabled: u8,
}

impl InterruptState {
    pub fn new() -> InterruptState {
        InterruptState {
            interrupt_flags: 0,
            interrupts_enabled: 0,
        }
    }

    pub fn fire_interrupt(&mut self, interrupt: &Interrupt) {
        self.interrupt_flags = set_bit_in_byte(self.interrupt_flags, *interrupt as u8);
    }

    pub fn interrupt_fired(&self, interrupt: &Interrupt) -> bool {
        let interrupt_value = *interrupt as u8;
        let interrupt_fired = is_bit_set(&self.interrupt_flags, interrupt_value);
        let interrupt_enabled = is_bit_set(&self.interrupts_enabled, interrupt_value);

        interrupt_fired && interrupt_enabled
    }

    pub fn reset_interrupt(&mut self, interrupt: &Interrupt) {
        self.interrupt_flags = reset_bit_in_byte(self.interrupt_flags, *interrupt as u8);
    }
}
