use crate::cartridge::mbc1::Mbc1;
use crate::cartridge::rom_only::RomOnlyCartridge;
use crate::cartridge::mbc2::Mbc2;

pub mod cartridge_base;
pub mod mbc1;
pub mod mbc2;
pub mod rom_only;

pub const EXT_RAM_SIZE: usize = 8192;
pub const EXT_RAM_ADDRESS: usize = 0xA000;
const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;
const RAM_SIZE_ADDRESS: usize = 0x149;

pub trait Cartridge {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn dump_savegame(&self);
    fn load_savegame(&mut self);
}

pub trait RamDumper {
    fn dump(&self, data: &Vec<u8>);
    fn load(&self) -> Option<Vec<u8>>;
}

pub fn new_cartridge(
    rom: Vec<u8>,
    ram_dumper: Option<Box<dyn RamDumper>>,
) -> Result<Box<dyn Cartridge>, String> {
    let cartridge_type = rom[CARTRIDGE_TYPE_ADDRESS];
    match cartridge_type {
        0x00 | 0x08..=0x09 => Ok(Box::new(RomOnlyCartridge::new(rom, ram_dumper))),
        0x01..=0x03 => Ok(Box::new(Mbc1::new(rom, ram_dumper))),
        0x05..=0x06 => Ok(Box::new(Mbc2::new(rom, ram_dumper))),
        _ => Err(format!("Unknown cartridge type: 0x{:X}", cartridge_type)),
    }
}

pub fn get_ram_size(rom: &Vec<u8>) -> Option<usize> {
    match rom[RAM_SIZE_ADDRESS] {
        0x00 => None,
        0x01 => Some(2 * 1024),
        0x02 => Some(8 * 1024),
        0x03 => Some(32 * 1024),
        0x04 => Some(128 * 1024),
        0x05 => Some(64 * 1024),
        _ => None,
    }
}

pub fn create_ram(ram_size: Option<usize>) -> Option<Vec<u8>> {
    match ram_size {
        Some(size) => Some(vec![0; size]),
        None => None,
    }
}
