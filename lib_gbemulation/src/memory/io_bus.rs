use crate::io::joypad::Joypad;
use crate::memory::ReadWrite;

pub const IO_START_ADDRESS: u16 = 0xFF00;

pub struct IoBus {
    pub divider: u8,
    pub counter: u8,
    pub modulo: u8,
    pub timer_control: u8,
    pub lcdc: u8,
    pub stat: u8,
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub bgpal: u8,
    pub lyc: u8,
    pub current_scanline: u8,
    joypad_select: u8,
    joypad: u8,
    unmapped: [u8; 128],
}

impl IoBus {
    pub fn new() -> IoBus {
        IoBus {
            divider: 0,
            counter: 0,
            modulo: 0,
            timer_control: 0,
            lcdc: 0,
            stat: 0x84,
            scroll_y: 0,
            scroll_x: 0,
            bgpal: 0,
            lyc: 0,
            current_scanline: 0,
            unmapped: [0; 128],
            joypad_select: 0xFF,
            joypad: 0xFF,
        }
    }

    pub fn read_joypad(&mut self, joypad: &mut Joypad) {
        joypad.select_keys_by_write(self.joypad_select);
        self.joypad = joypad.read_input();
    }
}

impl ReadWrite for IoBus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad,
            0xFF04 => self.divider,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => self.timer_control,
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.current_scanline,
            0xFF45 => self.lyc,
            0xFF47 => self.bgpal,
            _ => self.unmapped[(address - IO_START_ADDRESS) as usize],
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.joypad_select = value,
            0xFF04 => self.divider = 0,
            0xFF05 => self.counter = value,
            0xFF06 => self.modulo = value,
            0xFF07 => self.timer_control = value,
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgpal = value,
            _ => self.unmapped[(address - IO_START_ADDRESS) as usize] = value,
        }
    }
}
