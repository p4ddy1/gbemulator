use crate::clock::Clock;
use crate::cpu::cpu::Cpu;
use crate::gpu::gpu::Gpu;
use crate::io::joypad::Joypad;
use crate::io::timer::Timer;
use crate::memory::mmu::Mmu;
use std::thread;
use std::time::Duration;

const CPU_CLOCK_HZ: usize = 4194304;
const FPS: usize = 60;

pub struct Emulation {
    clock: Clock,
    timer: Timer,
}

impl Emulation {
    pub fn new() -> Emulation {
        Emulation {
            clock: Clock::new(CPU_CLOCK_HZ, FPS),
            timer: Timer::new(),
        }
    }

    /// This method will cycle the emulator and sleep afterwards for an amount of time
    /// Execute in a loop
    pub fn cycle(&mut self, cpu: &mut Cpu, gpu: &mut Gpu, mmu: &mut Mmu, joypad: &mut Joypad) {
        //TODO: Check if this is the correct way
        while self.clock.clock_cycles_passed_frame < self.clock.clock_cycles_per_frame {
            let mut last_cycle = cpu.step(mmu);
            last_cycle += mmu.step(joypad);
            gpu.step(mmu, last_cycle);
            self.clock.cycle(last_cycle);
            self.timer.step(mmu, last_cycle);
        }

        self.clock.reset();
        gpu.screen.present();
        thread::sleep(Duration::from_nanos(self.clock.frame_time_ns));
    }
}
