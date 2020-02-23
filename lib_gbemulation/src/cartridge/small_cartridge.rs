use crate::cartridge::{Cartridge, EXT_RAM_ADDRESS, EXT_RAM_SIZE};
use std::fs;

pub struct SmallCartridge {
    data: Vec<u8>,
    ram: [u8; EXT_RAM_SIZE],
}

impl SmallCartridge {
    pub fn new(data: Vec<u8>) -> SmallCartridge {
        SmallCartridge {
            data,
            ram: [0; EXT_RAM_SIZE],
        }
    }

    pub fn new_from_file(filename: &str) -> Result<SmallCartridge, &str> {
        let data = match fs::read(filename) {
            Ok(data) => data,
            Err(_) => {
                return Err("Could not open file");
            }
        };

        Ok(SmallCartridge {
            data,
            ram: [0; EXT_RAM_SIZE],
        })
    }
}

impl Cartridge for SmallCartridge {
    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, _address: u16, _value: u8) {
        //Write does nothing, because no MBC is here
        return;
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize - EXT_RAM_ADDRESS] = value;
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize - EXT_RAM_ADDRESS]
    }
}
