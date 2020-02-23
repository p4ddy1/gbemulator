use crate::cartridge::{Cartridge, EXT_RAM_ADDRESS, EXT_RAM_SIZE};
use std::fs;

enum Mode {
    RomBankingMode,
    RamBankingMode,
}

pub struct Mbc1Cartridge {
    data: Vec<u8>,
    ram: [u8; EXT_RAM_SIZE * 3],
    selected_bank: u8,
    selected_ram_bank: u8,
    selected_mode: Mode,
}

impl Mbc1Cartridge {
    pub fn new_from_file(filename: String) -> Result<Mbc1Cartridge, &'static str> {
        let data = match fs::read(filename) {
            Ok(data) => data,
            Err(_) => {
                return Err("Could not open file");
            }
        };

        Ok(Mbc1Cartridge {
            data,
            ram: [0; EXT_RAM_SIZE * 3],
            selected_bank: 1,
            selected_ram_bank: 0,
            selected_mode: Mode::RomBankingMode,
        })
    }
}

impl Cartridge for Mbc1Cartridge {
    fn read(&self, address: u16) -> u8 {
        match address {
            //Bank 00. Read directly from rom
            0x0..=0x3FFF => self.data[address as usize],
            //Bank 01-7F
            0x4000..=0x7FFF => {
                let offset = 0x4000 * self.selected_bank as u16;
                self.data[(address as usize - 0x4000) + offset as usize]
            }
            _ => panic!("Address unkown: 0x{:X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            //Address range for rom bank number
            0x2000..=0x3FFF => {
                //0 is also 1
                if value == 0 {
                    self.selected_bank = 1;
                }

                self.selected_bank = value;
            }
            //Address range for RAM bank number
            0x4000..=0x5FFF => match self.selected_mode {
                Mode::RamBankingMode => {
                    self.selected_ram_bank = value;
                }
                Mode::RomBankingMode => {
                    self.selected_bank = self.selected_bank & 0xC0 | (value << 5);
                }
            },
            //Select Mode
            0x6000..=0x7FFF => {
                if value == 0 {
                    self.selected_mode = Mode::RomBankingMode;
                } else if value == 1 {
                    self.selected_mode = Mode::RamBankingMode;
                }
            }
            _ => {}
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        let offset = EXT_RAM_SIZE * self.selected_ram_bank as usize;
        self.ram[(address as usize - EXT_RAM_ADDRESS) + offset] = value;
    }

    fn read_ram(&self, address: u16) -> u8 {
        let offset = EXT_RAM_SIZE * self.selected_ram_bank as usize;
        self.ram[(address as usize - EXT_RAM_ADDRESS) + offset]
    }
}
