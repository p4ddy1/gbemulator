use crate::cpu::cpu::Cpu;
use crate::gpu::gpu::Gpu;
use crate::io::joypad::Joypad;
use crate::memory::mmu::Mmu;
use std::thread;
use std::time::Duration;

const DIV_DIVIDER: usize = 256;

pub struct Emulation {
    cpu_clock_hz: usize,
    clock_cycles_passed_frame: usize,
    clock_cycles_passed_timer: usize,
    clock_cycles_per_frame: usize,
    frame_time_ns: u64,
}

impl Emulation {
    pub fn new() -> Emulation {
        let cpu_clock_hz = 4194304;
        let fps = 60;
        let clock_cycles_per_frame: usize = cpu_clock_hz / fps;
        let frame_time: u64 = 1000000000 / fps as u64;

        Emulation {
            cpu_clock_hz: cpu_clock_hz,
            clock_cycles_passed_frame: 0,
            clock_cycles_passed_timer: 0,
            clock_cycles_per_frame: clock_cycles_per_frame,
            frame_time_ns: frame_time,
        }
    }

    /// This method will cycle the emulator and sleep afterwards for an amount of time
    /// Execute in a loop
    pub fn cycle(&mut self, cpu: &mut Cpu, gpu: &mut Gpu, mmu: &mut Mmu, joypad: &mut Joypad) {
        //TODO: Check if this is the correct way
        while self.clock_cycles_passed_frame < self.clock_cycles_per_frame {
            let mut last_cycle = cpu.step(mmu);
            last_cycle += mmu.step(joypad);
            gpu.step(mmu, last_cycle);

            self.clock_cycles_passed_frame += last_cycle as usize;

            self.clock_cycles_passed_timer += self.clock_cycles_passed_frame;

            if self.clock_cycles_passed_timer % DIV_DIVIDER == 0 {
                mmu.io_bus.divider = mmu.io_bus.divider.wrapping_add(1);
            }

            if self.clock_cycles_passed_timer > self.cpu_clock_hz {
                self.clock_cycles_passed_timer = 0;
            }
        }

        self.clock_cycles_passed_frame = 0;
        gpu.screen.present();
        thread::sleep(Duration::from_nanos(self.frame_time_ns));
    }
}
