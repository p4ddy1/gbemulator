use crate::cartridge::cartridge_base::CartridgeBase;
use crate::cartridge::{Cartridge, RamDumper, CARTRIDGE_TYPE_ADDRESS};

pub struct Mbc2 {
    cartridge_base: CartridgeBase,
}

impl Mbc2 {
    pub fn new(rom: Vec<u8>, ram_dumper: Option<Box<dyn RamDumper + Send>>) -> Self {
        let cartridge_type = rom[CARTRIDGE_TYPE_ADDRESS];
        let has_battery = cartridge_type == 0x06;

        let cartridge_base = CartridgeBase::new(rom, true, Some(512), has_battery, ram_dumper);

        Mbc2 { cartridge_base }
    }
}

impl Cartridge for Mbc2 {
    fn read(&self, address: u16) -> u8 {
        self.cartridge_base.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0..=0x1FFF => {
                if address & 0x100 == 0 {
                    self.cartridge_base.ram_enabled = value == 0x0A
                }
            }
            0x2000..=0x3FFF => {
                if address & 0x100 != 0x100 {
                    return;
                }

                let bank_number = if value == 0 { 1 } else { value };

                self.cartridge_base.rom_bank = bank_number & 0xF;
            }
            _ => {}
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        match address {
            0xA000..=0xA1FF => {
                self.cartridge_base.write_ram(address, value & 0xF);
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        match address {
            0xA000..=0xA1FF => self.cartridge_base.read_ram(address) & 0xF,
            _ => 0,
        }
    }

    fn dump_savegame(&self) {
        self.cartridge_base.dump_savegame();
    }

    fn load_savegame(&mut self) {
        self.cartridge_base.load_savegame();
    }
}
