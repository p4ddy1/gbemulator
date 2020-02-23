use crate::memory::ReadWrite;

pub struct IoBus {
    divider: u8,
    lcdc: u8,
    stat: u8,
    scroll_y: u8,
    scroll_x: u8,
    bgpal: u8,
    lyc: u8,
}

impl IoBus {
    pub fn new() -> IoBus {
        IoBus {
            divider: 0,
            lcdc: 0,
            stat: 0,
            scroll_y: 0,
            scroll_x: 0,
            bgpal: 0,
            lyc: 0,
        }
    }
}

impl ReadWrite for IoBus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider,
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF45 => self.lyc,
            0xFF47 => self.bgpal,
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.divider = 0,
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgpal = value,
            _ => {}
        }
    }
}
