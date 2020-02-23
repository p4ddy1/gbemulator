pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
pub const INTERRUPT_FLAGS_ADDRESS: u16 = 0xFF0F;

pub enum Interrupt {
    Vblank = 0x01,
    LcdStat = 0x02,
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

    pub fn fire_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt_flags = interrupt as u8;
    }
}
