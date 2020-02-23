pub mod mbc1_cartridge;
pub mod small_cartridge;

pub const EXT_RAM_SIZE: usize = 8192;
pub const EXT_RAM_ADDRESS: usize = 0xA000;

pub trait Cartridge {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
}
