pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
pub const INTERRUPT_FLAGS_ADDRESS: u16 = 0xFF0F;

#[derive(Copy, Clone)]
pub enum Interrupt {
    Vblank = 0x01,
    LcdStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
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
        self.interrupt_flags |= *interrupt as u8;
    }

    pub fn interrupt_fired(&self, interrupt: &Interrupt) -> bool {
        let interrupt_value = *interrupt as u8;

        self.interrupt_flags & interrupt_value == interrupt_value
            && self.interrupts_enabled & interrupt_value == interrupt_value
    }

    pub fn reset_interrupt(&mut self, interrupt: &Interrupt) {
        self.interrupt_flags &= *interrupt as u8 ^ 0xFF;
    }
}
