use crate::emulation::CPU_CLOCK_HZ;

const CYCLES_VOLUME_ENVELOPE_TIMER: u32 = (CPU_CLOCK_HZ / 64) as u32;
const CYCLES_LENGTH_COUNTER_TIMER: u32 = (CPU_CLOCK_HZ / 256) as u32;
const CYCLES_SWEEP_TIMER: u32 = (CPU_CLOCK_HZ / 128) as u32;

pub struct FrameSequencer {
    pub volume_envelope_trigger: bool,
    pub length_counter_trigger: bool,
    pub sweep_timer_trigger: bool,
    volume_envelope_timer: u32,
    length_counter_timer: u32,
    sweep_timer: u32,
}

impl FrameSequencer {
    pub fn new() -> Self {
        FrameSequencer {
            volume_envelope_trigger: false,
            length_counter_trigger: false,
            sweep_timer_trigger: false,
            volume_envelope_timer: 0,
            length_counter_timer: 0,
            sweep_timer: 0,
        }
    }

    pub fn step(&mut self, clock_cycles: u8) {
        cycle_timer(
            &mut self.volume_envelope_timer,
            CYCLES_VOLUME_ENVELOPE_TIMER,
            &mut self.volume_envelope_trigger,
            clock_cycles,
        );

        cycle_timer(
            &mut self.length_counter_timer,
            CYCLES_LENGTH_COUNTER_TIMER,
            &mut self.length_counter_trigger,
            clock_cycles,
        );

        cycle_timer(
            &mut self.sweep_timer,
            CYCLES_SWEEP_TIMER,
            &mut self.sweep_timer_trigger,
            clock_cycles,
        );
    }

    pub fn reset(&mut self) {
        self.volume_envelope_timer = 0;
        self.length_counter_timer = 0;
        self.sweep_timer = 0;
    }
}

fn cycle_timer(timer: &mut u32, limit: u32, trigger: &mut bool, clock_cycles: u8) {
    *timer += clock_cycles as u32;

    if limit <= *timer {
        *timer -= limit;
        *trigger = true;
    } else {
        *trigger = false;
    }
}
