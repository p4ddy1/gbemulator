use crate::cartridge::cartridge_base::CartridgeBase;
use crate::cartridge::{get_ram_size, Cartridge, RamDumper, CARTRIDGE_TYPE_ADDRESS};

pub struct RomOnlyCartridge {
    cartridge_base: CartridgeBase,
}

impl RomOnlyCartridge {
    pub fn new(rom: Vec<u8>, ram_dumper: Option<Box<dyn RamDumper + Send>>) -> Self {
        let cartridge_type = rom[CARTRIDGE_TYPE_ADDRESS];
        let has_ram = cartridge_type == 0x08 || cartridge_type == 0x09;
        let has_battery = cartridge_type == 0x09;
        let ram_size = get_ram_size(&rom);

        let cartridge_base = CartridgeBase::new(rom, has_ram, ram_size, has_battery, ram_dumper);

        RomOnlyCartridge { cartridge_base }
    }
}

impl Cartridge for RomOnlyCartridge {
    fn read(&self, address: u16) -> u8 {
        self.cartridge_base.read(address)
    }

    fn write(&mut self, _address: u16, _value: u8) {
        //ROM is not writable
        return;
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.cartridge_base.write_ram(address, value);
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.cartridge_base.read_ram(address)
    }

    fn dump_savegame(&self) {
        self.cartridge_base.dump_savegame();
    }

    fn load_savegame(&mut self) {
        self.cartridge_base.load_savegame()
    }
}
