use crate::cartridge::Cartridge;
use crate::io::joypad::Joypad;
use crate::memory::interrupts::InterruptState;
use crate::memory::io_bus::IoBus;
use crate::memory::{interrupts, io_bus, ReadWrite};
use crate::util::binary;

const EXT_RAM_START_ADDRESS: u16 = 0xA000;
pub const W_RAM_ADDRESS: u16 = 0xC000;
pub const ECHO_RAM_ADDRESS: u16 = 0xE000;
pub const H_RAM_ADDR: u16 = 0xFF80;
pub const VRAM_ADDRESS: u16 = 0x8000;
pub const OAM_ADDRESS: u16 = 0xFE00;

const W_RAM_SIZE: usize = 8192;
const H_RAM_SIZE: usize = 127;
const V_RAM_SIZE: usize = 8192;
const OAM_SIZE: usize = 160;

pub enum Opcode {
    Regular(u8),
    CB(u8),
}

pub struct Mmu<'a> {
    cartridge: &'a mut dyn Cartridge,
    w_ram: [u8; W_RAM_SIZE],
    h_ram: [u8; H_RAM_SIZE],
    v_ram: [u8; V_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    pub io_bus: IoBus,
    pub interrupts: InterruptState
}

impl<'a> Mmu<'a> {
    pub fn new(cartridge: &'a mut dyn Cartridge) -> Mmu<'a> {
        Mmu {
            cartridge: cartridge,
            w_ram: [0; W_RAM_SIZE],
            h_ram: [0; H_RAM_SIZE],
            v_ram: [0; V_RAM_SIZE],
            oam: [0; OAM_SIZE],
            io_bus: IoBus::new(),
            interrupts: InterruptState::new()
        }
    }

    pub fn step(&mut self, joypad: &mut Joypad) {
        self.io_bus.read_joypad(joypad);
    }

    fn dma_transfer(&mut self, source_address: u8) {
        //DMA Transfer starts to OAM
        //Start address = value * 0x100 (value << 8)
        //Destination = OAM
        //Write everything from start for OAM length
        //OAM Length = 0xA0 (160)
        let start_address: u16 = (source_address as u16) << 8;

        for offset in 0..160 {
            self.write(OAM_ADDRESS + offset, self.read(start_address + offset))
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            W_RAM_ADDRESS..=0xDFFF => self.w_ram[(address - W_RAM_ADDRESS) as usize],
            ECHO_RAM_ADDRESS..=0xFDFF => self.w_ram[(address - ECHO_RAM_ADDRESS) as usize],
            0..=0x7FFF => self.cartridge.read(address),
            interrupts::INTERRUPT_FLAGS_ADDRESS => self.interrupts.interrupt_flags,
            VRAM_ADDRESS..=0x9FFF => self.read_vram(address),
            OAM_ADDRESS..=0xFE9F => self.read_oam(address),
            EXT_RAM_START_ADDRESS..=0xBFFF => self.cartridge.read_ram(address),
            io_bus::IO_START_ADDRESS..=0xFF7F => self.io_bus.read(address),
            H_RAM_ADDR..=0xFFFE => self.h_ram[(address - H_RAM_ADDR) as usize],
            interrupts::INTERRUPT_ENABLE_ADDRESS => self.interrupts.interrupts_enabled,
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            W_RAM_ADDRESS..=0xDFFF => self.w_ram[(address - W_RAM_ADDRESS) as usize] = value,
            EXT_RAM_START_ADDRESS..=0xBFFF => self.cartridge.write_ram(address, value),
            0..=0x7FFF => self.cartridge.write(address, value),
            interrupts::INTERRUPT_FLAGS_ADDRESS => self.interrupts.interrupt_flags = value,
            interrupts::INTERRUPT_ENABLE_ADDRESS => self.interrupts.interrupts_enabled = value,
            //TODO: Implement accurate dma
            0xFF46 => self.dma_transfer(value),
            io_bus::IO_START_ADDRESS..=0xFF7F => self.io_bus.write(address, value),
            H_RAM_ADDR..=0xFFFE => self.h_ram[(address - H_RAM_ADDR) as usize] = value,
            VRAM_ADDRESS..=0x9FFF => self.v_ram[(address - VRAM_ADDRESS) as usize] = value,
            OAM_ADDRESS..=0xFE9F => self.oam[(address - OAM_ADDRESS) as usize] = value,
            _ => {}
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    pub fn read_word(&self, address: u16) -> u16 {
        binary::bytes_to_word(self.read(address + 1), self.read(address))
    }

    /// Direct access to the VRAM for the GPU
    pub fn read_vram(&self, address: u16) -> u8 {
        self.v_ram[(address - VRAM_ADDRESS) as usize]
    }

    /// Direct access to the OAM table for the GPU
    pub fn read_oam(&self, address: u16) -> u8 {
        self.oam[(address - OAM_ADDRESS) as usize]
    }

    pub fn read_opcode(&self, pc: u16) -> Opcode {
        let op_code = self.read(pc);

        match op_code {
            0xCB => Opcode::CB(self.read(pc + 1)),
            _ => Opcode::Regular(op_code),
        }
    }
}
