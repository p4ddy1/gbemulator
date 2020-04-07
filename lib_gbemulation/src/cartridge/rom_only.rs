use crate::cartridge::Cartridge;

pub struct RomOnlyCartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    has_battery: bool
}

impl Cartridge for RomOnlyCartridge {
    fn read(&self, address: u16) -> u8 {
        self.rom[address]
    }

    fn write(&mut self, address: u16, value: u8) {
        //ROM is not writable
        return
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

    fn dump_savegame(&self) {
        if !self.has_battery {
            return;
        }

        if let Some(ref ram) = self.ram
    }

    fn load_savegame(&mut self) {
        unimplemented!()
    }
}