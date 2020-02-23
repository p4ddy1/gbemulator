use crate::memory::ReadWrite;

pub const W_RAM_ADDRESS: u16 = 0xC000;
pub const ECHO_RAM_ADDRESS: u16 = 0xE000;
pub const H_RAM_ADDR: u16 = 0xFF80;
pub const VRAM_ADDRESS: u16 = 0x8000;
pub const OAM_ADDRESS: u16 = 0xFE00;

const W_RAM_SIZE: usize = 8192;
const ECHO_RAM_SIZE: usize = 7679;
const H_RAM_SIZE: usize = 127;
const V_RAM_SIZE: usize = 8192;
const OAM_SIZE: usize = 159;

pub struct Ram {
    w_ram: [u8; W_RAM_SIZE],
    echo_ram: [u8; ECHO_RAM_SIZE],
    h_ram: [u8; H_RAM_SIZE],
    v_ram: [u8; V_RAM_SIZE],
    oam: [u8; OAM_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            w_ram: [0; W_RAM_SIZE],
            echo_ram: [0; ECHO_RAM_SIZE],
            h_ram: [0; H_RAM_SIZE],
            v_ram: [0; V_RAM_SIZE],
            oam: [0; OAM_SIZE],
        }
    }
}

impl ReadWrite for Ram {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            W_RAM_ADDRESS..=0xDFFF => self.w_ram[(address - W_RAM_ADDRESS) as usize] = value,
            ECHO_RAM_ADDRESS..=0xFDFE => {
                self.echo_ram[(address - ECHO_RAM_ADDRESS) as usize] = value
            }
            H_RAM_ADDR..=0xFFFD => self.w_ram[(address - H_RAM_ADDR) as usize] = value,
            VRAM_ADDRESS..=0x9FFF => self.v_ram[(address - VRAM_ADDRESS) as usize] = value,
            OAM_ADDRESS..=0xFE9E => self.oam[(address - OAM_ADDRESS) as usize] = value,
            _ => println!("Tried to write to unkown RAM Address: 0x{:X}", address),
        }
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            W_RAM_ADDRESS..=0xDFFF => self.w_ram[(address - W_RAM_ADDRESS) as usize],
            ECHO_RAM_ADDRESS..=0xFDFE => self.echo_ram[(address - ECHO_RAM_ADDRESS) as usize],
            H_RAM_ADDR..=0xFFFD => self.w_ram[(address - H_RAM_ADDR) as usize],
            VRAM_ADDRESS..=0x9FFF => self.v_ram[(address - VRAM_ADDRESS) as usize],
            OAM_ADDRESS..=0xFE9E => self.oam[(address - OAM_ADDRESS) as usize],
            _ => {
                println!("Tried to read from unkown RAM Address: 0x{:X}", address);
                0
            }
        }
    }
}
