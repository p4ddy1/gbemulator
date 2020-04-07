use crate::cartridge::{create_ram, get_ram_size, Cartridge, RamDumper, CARTRIDGE_TYPE_ADDRESS};

pub struct RomOnlyCartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    has_battery: bool,
    ram_dumper: Option<Box<dyn RamDumper>>,
}

impl RomOnlyCartridge {
    pub fn new(rom: Vec<u8>, ram_dumper: Option<Box<dyn RamDumper>>) -> Self {
        let cartridge_type = rom[CARTRIDGE_TYPE_ADDRESS];
        let has_ram = cartridge_type == 0x08 || cartridge_type == 0x09;
        let has_battery = cartridge_type == 0x09;

        let ram = if has_ram {
            create_ram(get_ram_size(&rom))
        } else {
            None
        };

        RomOnlyCartridge {
            rom,
            ram,
            has_battery,
            ram_dumper,
        }
    }
}

impl Cartridge for RomOnlyCartridge {
    fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write(&mut self, _address: u16, _value: u8) {
        //ROM is not writable
        return;
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if let Some(ref mut ram) = self.ram {
            ram[address as usize] = value;
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if let Some(ref ram) = self.ram {
            return ram[address as usize];
        }

        0
    }

    fn get_ram(&self) -> &Option<Vec<u8>> {
        &self.ram
    }

    fn set_ram(&mut self, data: Vec<u8>) {
        if let Some(ref mut ram) = self.ram {
            *ram = data;
        }
    }

    fn get_ram_dumper(&self) -> &Option<Box<dyn RamDumper>> {
        &self.ram_dumper
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }
}
