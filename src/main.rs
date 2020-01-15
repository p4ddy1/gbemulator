use crate::cartridge::cartridge::Cartridge;
use crate::cpu::cpu::Cpu;
use crate::gpu::gpu::Gpu;
use crate::gpu::screen::SdlScreen;
use crate::memory::mmu::Mmu;
use std::thread;
use std::time::Duration;

mod cartridge;
mod cpu;
mod gpu;
mod memory;
mod util;

fn main() {
    let cartridge = match Cartridge::new_from_file("testrom/tetris.gb") {
        Ok(c) => c,
        Err(e) => {
            panic!(e);
        }
    };

    let bios = match Cartridge::new_from_file("testrom/bios.gb") {
        Ok(c) => c,
        Err(e) => {
            panic!(e);
        }
    };
    let mut screen = SdlScreen::new();

    let mut gpu = Gpu::new(&mut screen);
    let mut mmu = Mmu::new(&cartridge, &mut gpu, Some(&bios));
    let mut cpu = Cpu::new(&mut mmu);

    const CPU_CLOCK_HZ: usize = 4194304;
    const FPS: usize = 120;
    const CLOCK_CYCLES_PER_FRAME: usize = CPU_CLOCK_HZ / FPS;
    const FRAME_TIME_NS: u64 = 1000000000 / FPS as u64;

    let mut clock_cycles_passed = 0;

    loop {
        while clock_cycles_passed < CLOCK_CYCLES_PER_FRAME {
            let last_cycle = cpu.execute_program_counter();
            clock_cycles_passed += last_cycle as usize;
        }

        clock_cycles_passed = 0;

        thread::sleep(Duration::from_nanos(FRAME_TIME_NS));
    }
}
