use crate::apu::apu::Apu;

use crate::cartridge::Cartridge;
use crate::gpu::gpu::Gpu;
use crate::io::joypad::Joypad;
use crate::io::timer::Timer;
use crate::memory::interrupts;
use crate::memory::interrupts::InterruptState;
use crate::util::binary;

const EXT_RAM_START_ADDRESS: u16 = 0xA000;
pub const W_RAM_ADDRESS: u16 = 0xC000;
pub const ECHO_RAM_ADDRESS: u16 = 0xE000;
pub const H_RAM_ADDR: u16 = 0xFF80;
pub const VRAM_ADDRESS: u16 = 0x8000;
pub const OAM_ADDRESS: u16 = 0xFE00;

const W_RAM_SIZE: usize = 8192;
const H_RAM_SIZE: usize = 127;

pub enum Opcode {
    Regular(u8),
    CB(u8),
}

pub struct Mmu<'a> {
    pub gpu: &'a mut Gpu,
    pub timer: Timer,
    pub interrupts: InterruptState,
    pub apu: &'a mut Apu<'a>,
    w_ram: [u8; W_RAM_SIZE],
    h_ram: [u8; H_RAM_SIZE],
    joypad_select: u8,
    joypad: u8,
    cartridge: &'a mut dyn Cartridge,
}

impl<'a> Mmu<'a> {
    pub fn new(
        cartridge: &'a mut dyn Cartridge,
        gpu: &'a mut Gpu,
        apu: &'a mut Apu<'a>,
    ) -> Mmu<'a> {
        Mmu {
            gpu,
            timer: Timer::new(),
            interrupts: InterruptState::new(),
            apu,
            w_ram: [0; W_RAM_SIZE],
            h_ram: [0; H_RAM_SIZE],
            joypad_select: 0xFF,
            joypad: 0xFF,
            cartridge,
        }
    }

    pub fn step(&mut self, joypad: &Joypad, clock_cycles: u8) {
        self.read_joypad(joypad);
        self.gpu.step(clock_cycles);
        self.timer.step(clock_cycles);
        self.apu.step(clock_cycles);
        self.interrupts.interrupt_flags |= self.timer.interrupts_fired;
        self.interrupts.interrupt_flags |= self.gpu.interrupts_fired;
        self.gpu.interrupts_fired = 0;
        self.timer.interrupts_fired = 0;
    }

    pub fn save(&self) {
        self.cartridge.dump_savegame();
    }

    fn read_joypad(&mut self, joypad: &Joypad) {
        self.joypad = joypad.read_input(self.joypad_select);
    }

    fn dma_transfer(&mut self, source_address: u8) {
        //DMA Transfer starts to OAM
        //Start address = value * 0x100 (value << 8)
        //Destination = OAM
        //Write everything from start for OAM length
        //OAM Length = 0xA0 (160)
        let start_address: u16 = (source_address as u16) << 8;

        for offset in 0..160 {
            self.gpu
                .write_oam(OAM_ADDRESS + offset, self.read(start_address + offset))
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            W_RAM_ADDRESS..=0xDFFF => self.w_ram[(address - W_RAM_ADDRESS) as usize],
            ECHO_RAM_ADDRESS..=0xFDFF => self.w_ram[(address - ECHO_RAM_ADDRESS) as usize],
            0..=0x7FFF => self.cartridge.read(address),
            interrupts::INTERRUPT_FLAGS_ADDRESS => self.interrupts.interrupt_flags,
            VRAM_ADDRESS..=0x9FFF => self.gpu.read_vram(address),
            OAM_ADDRESS..=0xFE9F => self.gpu.read_oam(address),
            EXT_RAM_START_ADDRESS..=0xBFFF => self.cartridge.read_ram(address),
            0xFF00 => self.joypad,
            0xFF04 => self.timer.divider,
            0xFF05 => self.timer.counter,
            0xFF06 => self.timer.modulo,
            0xFF07 => self.timer.timer_control,
            0xFF10..=0xFF3F => self.apu.read(address),
            0xFF40 => self.gpu.get_lcdc(),
            0xFF41 => self.gpu.get_stat(),
            0xFF42 => self.gpu.scroll_y,
            0xFF43 => self.gpu.scroll_x,
            0xFF44 => self.gpu.current_scanline,
            0xFF45 => self.gpu.get_lyc(),
            0xFF47 => self.gpu.get_bg_pal(),
            0xFF48 => self.gpu.get_sprite_palette0(),
            0xFF49 => self.gpu.get_sprite_palette1(),
            0xFF4A => self.gpu.window_y,
            0xFF4B => self.gpu.window_x,
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
            0xFF00 => self.joypad_select = value,
            0xFF04 => self.timer.divider = 0,
            0xFF05 => self.timer.counter = value,
            0xFF06 => self.timer.modulo = value,
            0xFF07 => self.timer.timer_control = value,
            0xFF10..=0xFF3F => self.apu.write(address, value),
            0xFF40 => self.gpu.set_lcdc(value),
            0xFF41 => self.gpu.set_stat(value),
            0xFF42 => self.gpu.scroll_y = value,
            0xFF43 => self.gpu.scroll_x = value,
            0xFF45 => self.gpu.set_lyc(value),
            //TODO: Implement accurate dma
            0xFF46 => self.dma_transfer(value),
            0xFF47 => self.gpu.set_bg_pal(value),
            0xFF48 => self.gpu.set_sprite_palette0(value),
            0xFF49 => self.gpu.set_sprite_palette1(value),
            0xFF4A => self.gpu.window_y = value,
            0xFF4B => {
                if value < 7 {
                    return;
                }
                self.gpu.window_x = value
            }
            H_RAM_ADDR..=0xFFFE => self.h_ram[(address - H_RAM_ADDR) as usize] = value,
            VRAM_ADDRESS..=0x9FFF => self.gpu.write_vram(address, value),
            OAM_ADDRESS..=0xFE9F => self.gpu.write_oam(address, value),
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

    pub fn read_opcode(&self, pc: u16) -> Opcode {
        let op_code = self.read(pc);

        match op_code {
            0xCB => Opcode::CB(self.read(pc + 1)),
            _ => Opcode::Regular(op_code),
        }
    }
}
