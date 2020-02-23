mod io_bus;
pub mod mmu;
pub mod mmu_old;
mod ram;

pub trait ReadWrite {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
