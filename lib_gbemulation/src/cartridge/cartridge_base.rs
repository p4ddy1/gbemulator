use crate::cartridge::{create_ram, RamDumper, EXT_RAM_ADDRESS, EXT_RAM_SIZE};

pub struct CartridgeBase {
    pub rom: Vec<u8>,
    pub ram: Option<Vec<u8>>,
    pub rom_bank: u8,
    pub ram_bank: u8,
    pub ram_enabled: bool,
    has_battery: bool,
    ram_dumper: Option<Box<dyn RamDumper + Send>>,
}

impl CartridgeBase {
    pub fn new(
        rom: Vec<u8>,
        has_ram: bool,
        ram_size: Option<usize>,
        has_battery: bool,
        ram_dumper: Option<Box<dyn RamDumper + Send>>,
    ) -> Self {
        let ram = if has_ram { create_ram(ram_size) } else { None };

        let mut base = CartridgeBase {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            has_battery,
            ram_dumper,
        };

        base.load_savegame();
        base
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            //Bank 00. Read directly from rom
            0x0..=0x3FFF => self.rom[address as usize],
            //Bank 01-7F
            0x4000..=0x7FFF => {
                let offset = 0x4000 * self.rom_bank as usize;
                self.rom[(address as usize - 0x4000) + offset]
            }
            _ => panic!("Address unknown: 0x{:X}", address),
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let ram_bank = self.ram_bank as usize;

        if let Some(ref mut ram) = self.ram {
            let offset = EXT_RAM_SIZE * ram_bank;
            ram[(address as usize - EXT_RAM_ADDRESS) + offset] = value;
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0;
        }

        let ram_bank = self.ram_bank as usize;

        if let Some(ref ram) = self.ram {
            let offset = EXT_RAM_SIZE * ram_bank;
            return ram[(address as usize - EXT_RAM_ADDRESS) + offset];
        }

        0
    }

    pub fn dump_savegame(&self) {
        if !self.has_battery {
            return;
        }

        if let Some(ref ram) = self.ram {
            if let Some(ref dumper) = self.ram_dumper {
                dumper.dump(ram)
            }
        }
    }

    pub fn load_savegame(&mut self) {
        if !self.has_battery {
            return;
        }

        if let Some(ref dumper) = self.ram_dumper {
            if let Some(data) = dumper.load() {
                if let Some(ref mut ram) = self.ram {
                    *ram = data;
                }
            }
        }
    }
}
