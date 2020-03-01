use crate::memory::interrupts::Interrupt;
use crate::memory::mmu::Mmu;
use crate::util::binary::is_bit_set;

const DIVIDER_CYCLES: u32 = 256;
const SPEED_0_CYCLES: u32 = 1024;
const SPEED_1_CYCLES: u32 = 16;
const SPEED_2_CYCLES: u32 = 64;
const SPEED_3_CYCLES: u32 = 256;

pub struct Timer {
    clock_cycles_divider: u32,
    clock_cycles_timer: u32,
    has_overflowed: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            clock_cycles_divider: 0,
            clock_cycles_timer: 0,
            has_overflowed: false,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, clock_cycles: u8) {
        let cycles = clock_cycles as u32;
        self.clock_cycles_divider += cycles;

        while self.clock_cycles_divider >= DIVIDER_CYCLES {
            mmu.io_bus.divider = mmu.io_bus.divider.wrapping_add(1);
            self.clock_cycles_divider -= DIVIDER_CYCLES;
        }

        let speed = mmu.io_bus.timer_control << 6;
        let running = is_bit_set(&mmu.io_bus.timer_control, 2);

        if !running {
            return;
        }

        self.clock_cycles_timer += cycles;

        let timer_cycles = match speed {
            0 => SPEED_0_CYCLES,
            0x40 => SPEED_1_CYCLES,
            0x80 => SPEED_2_CYCLES,
            0xC0 => SPEED_3_CYCLES,
            _ => panic!("Unknown timer speed: 0x{:X}", speed),
        };

        if self.has_overflowed {
            mmu.interrupts.fire_interrupt(&Interrupt::Timer);
            mmu.io_bus.counter = mmu.io_bus.modulo;
            self.has_overflowed = false;
        }

        //It could happen that the timer needs to be incremented multiple times within a given cycle
        while self.clock_cycles_timer >= timer_cycles {
            if mmu.io_bus.counter == 255 {
                //Overflow does not happen immediately
                self.has_overflowed = true;
                mmu.io_bus.counter = 0;
                return;
            }

            mmu.io_bus.counter += 1;
            self.clock_cycles_timer -= timer_cycles;
        }
    }
}
