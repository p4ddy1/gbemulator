use crate::clock::Clock;
use crate::cpu::cpu::Cpu;
use crate::io::joypad::Joypad;
use crate::memory::mmu::Mmu;

pub const CPU_CLOCK_HZ: usize = 4194304;
pub const FPS: f32 = 60.0;

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
    pub fn cycle(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, joypad: &Joypad) {
        while self.clock.clock_cycles_passed_frame <= self.clock.clock_cycles_per_frame {
            let last_cycle = cpu.step(mmu);
            mmu.step(joypad, last_cycle);
            self.clock.cycle(last_cycle);
        }

        self.clock.reset();
    }
}
