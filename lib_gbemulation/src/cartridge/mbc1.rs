use crate::cartridge::cartridge_base::CartridgeBase;
use crate::cartridge::{get_ram_size, Cartridge, RamDumper, CARTRIDGE_TYPE_ADDRESS};

enum Mode {
    RomBankingMode,
    RamBankingMode,
}

pub struct Mbc1 {
    cartridge_base: CartridgeBase,
    selected_mode: Mode,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>, ram_dumper: Option<Box<dyn RamDumper + Send>>) -> Self {
        let cartridge_type = rom[CARTRIDGE_TYPE_ADDRESS];
        let has_ram = cartridge_type == 0x02 || cartridge_type == 0x03;
        let has_battery = cartridge_type == 0x03;
        let ram_size = get_ram_size(&rom);

        let cartridge_base = CartridgeBase::new(rom, has_ram, ram_size, has_battery, ram_dumper);

        Mbc1 {
            cartridge_base,
            selected_mode: Mode::RomBankingMode,
        }
    }
}

impl Cartridge for Mbc1 {
    fn read(&self, address: u16) -> u8 {
        self.cartridge_base.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0..=0x1FFF => {
                self.cartridge_base.ram_enabled = value == 0x0A;
            }
            //Address range for rom bank number
            0x2000..=0x3FFF => {
                //0 is also 1
                let bank_number = if value == 0 { 1 } else { value };
                //Only set lower 5 bits
                self.cartridge_base.rom_bank =
                    self.cartridge_base.rom_bank & 0x60 | bank_number & 0x1F;
            }
            //Address range for RAM bank number
            0x4000..=0x5FFF => match self.selected_mode {
                Mode::RamBankingMode => {
                    self.cartridge_base.ram_bank = value;
                }
                Mode::RomBankingMode => {
                    //Only set upper 2 bits
                    self.cartridge_base.rom_bank =
                        self.cartridge_base.rom_bank | (value & 0x03) << 5;
                }
            },
            //Select Mode
            0x6000..=0x7FFF => match value {
                0 => self.selected_mode = Mode::RomBankingMode,
                1 => self.selected_mode = Mode::RamBankingMode,
                _ => {}
            },
            _ => {}
        }
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
        self.cartridge_base.load_savegame();
    }
}
