use crate::cartridge::Cartridge;
use crate::memory::io_bus::IoBus;
use crate::memory::ram::Ram;
use crate::memory::{ram, ReadWrite};

const EXT_RAM_START_ADDRESS: u16 = 0xA000;
const IO_START_ADDRESS: u16 = 0xFF00;

pub struct Mmu<'a> {
    cartridge: &'a mut dyn Cartridge,
    ram: Ram,
    io_bus: IoBus,
}

impl<'a> Mmu<'a> {
    pub fn new(cartridge: &'a mut dyn Cartridge) -> Mmu<'a> {
        Mmu {
            cartridge: cartridge,
            ram: Ram::new(),
            io_bus: IoBus::new(),
        }
    }

    fn dma_transfer(&mut self, source_address: u8) {
        //DMA Transfer starts to OAM
        //Start address = value * 0x100 (value << 8)
        //Destination = OAM
        //Write everything from start for OAM length
        //OAM Length = 0xA0 (160)
        let start_address: u16 = (source_address as u16) << 8;

        for offset in 0..160 {
            self.write(ram::OAM_ADDRESS + offset, self.read(start_address + offset))
        }
        //TODO: Cycles are missing here
        //The transfer takes 160 machine cycles
    }
}

impl<'a> ReadWrite for Mmu<'a> {
    fn read(&self, address: u16) -> u8 {
        match address {
            0..=0x7FFF => self.cartridge.read(address),
            EXT_RAM_START_ADDRESS..=0xBFFF => self.cartridge.read_ram(address),
            IO_START_ADDRESS..=0xFF7E => self.io_bus.read(address),
            _ => self.ram.read(address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0..=0x7FFF => self.cartridge.write(address, value),
            EXT_RAM_START_ADDRESS..=0xBFFF => self.cartridge.write_ram(address, value),
            0xFF46 => self.dma_transfer(value),
            IO_START_ADDRESS..=0xFF7E => self.io_bus.write(address, value),
            _ => self.write(address, value),
        }
    }
}
