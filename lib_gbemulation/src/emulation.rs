use crate::clock::Clock;
use crate::cpu::cpu::Cpu;
use crate::io::joypad::Joypad;
use crate::memory::mmu::Mmu;
use std::thread;
use std::time::Duration;

const CPU_CLOCK_HZ: usize = 4194304;
const FPS: usize = 60;

pub struct Emulation {
    clock: Clock,
}

impl Emulation {
    pub fn new() -> Emulation {
        Emulation {
            clock: Clock::new(CPU_CLOCK_HZ, FPS),
        }
    }

    /// This method will cycle the emulator and sleep afterwards for an amount of time
    /// Execute in a loop
    pub fn cycle(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, joypad: &mut Joypad) {
        //TODO: Check if this is the correct way
        while self.clock.clock_cycles_passed_frame < self.clock.clock_cycles_per_frame {
            let last_cycle = cpu.step(mmu);
            mmu.step(joypad, last_cycle);
            self.clock.cycle(last_cycle);
        }

        self.clock.reset();
        mmu.gpu.screen.present();
        thread::sleep(Duration::from_nanos(self.clock.frame_time_ns));
    }
}
