use crate::apu::channel::frame_sequencer::FrameSequencer;
use crate::apu::channel::length_counter::{LengthCounter, LengthCounterResult};
use crate::apu::channel::volume_envelope::VolumeEnvelope;
use crate::apu::Channel;
use crate::util::binary::is_bit_set;

const DIVISOR_CODE_MAP: [usize; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

pub struct NoiseChannel {
    volume_envelope: VolumeEnvelope,
    length_counter: LengthCounter,
    timer: i32,
    lfsr: u16,
    clock_shift: u8,
    lfsr_width_mode: u8,
    divisor_code: u8,
    enabled: bool,
    base_address: u16,
}

impl NoiseChannel {
    pub fn new(base_address: u16) -> Self {
        NoiseChannel {
            volume_envelope: VolumeEnvelope::new(),
            length_counter: LengthCounter::new(64),
            timer: 0,
            lfsr: 0,
            clock_shift: 0,
            lfsr_width_mode: 0,
            divisor_code: 0,
            enabled: false,
            base_address,
        }
    }

    fn set_length_counter_length(&mut self, value: u8) {
        self.length_counter.set_length(value & 0x3F);
    }

    fn set_clock_shift(&mut self, value: u8) {
        self.clock_shift = (value & 0xF0) >> 4;
    }

    fn set_lfsr_width_mode(&mut self, value: u8) {
        self.lfsr_width_mode = (value & 0x8) >> 3;
    }

    fn set_divisor_code(&mut self, value: u8) {
        self.divisor_code = value & 0x7;
    }

    fn get_period(&self) -> i32 {
        (DIVISOR_CODE_MAP[self.divisor_code as usize] << self.clock_shift) as i32
    }

    fn trigger(&mut self, value: u8) {
        //Do nothing if bit 7 is not set
        if !is_bit_set(&value, 7) {
            return;
        }

        self.lfsr = 0xFFFF;

        self.enabled = true;
        self.volume_envelope.trigger();
        self.length_counter.trigger();
        self.timer = self.get_period();
    }

    fn handle_length_counter(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.length_counter_trigger {
            if let LengthCounterResult::DisableChannel = self.length_counter.step() {
                self.enabled = false;
            }
        }
    }

    fn handle_volume_envelope(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.volume_envelope_trigger {
            self.volume_envelope.step();
        }
    }

    fn cycle_lfsr(&mut self) {
        let bit_0 = self.lfsr & 0x1;
        let bit_1 = (self.lfsr & 0x2) >> 1;
        let feedback = bit_1 ^ bit_0;
        self.lfsr = (self.lfsr & 0x7FFF) | (feedback << 15);
        self.lfsr = self.lfsr >> 1;

        if self.lfsr_width_mode == 1 {
            self.lfsr = (self.lfsr & 0x1FBF) | (feedback << 6);
        }
    }
}

impl Channel for NoiseChannel {
    fn output(&self) -> i16 {
        if !self.enabled {
            return 0;
        }

        let output = self.lfsr & 0x1;

        let signal = if output == 0 {
            1 * i16::MAX
        } else {
            -1 * i16::MAX
        };

        self.volume_envelope.process_signal(signal)
    }

    fn step(&mut self, frame_sequencer: &FrameSequencer, clock_cycles: u8) {
        if !self.enabled {
            return;
        }

        self.handle_length_counter(frame_sequencer);
        self.handle_volume_envelope(frame_sequencer);

        if self.timer <= 0 {
            self.cycle_lfsr();
            self.timer += self.get_period();
        }

        self.timer -= clock_cycles as i32;
    }

    fn write(&mut self, address: u16, value: u8) {
        if address < self.base_address {
            return;
        }

        let register = address - self.base_address;

        match register {
            1 => self.set_length_counter_length(value),
            2 => self.volume_envelope.write(value),
            3 => {
                self.set_clock_shift(value);
                self.set_lfsr_width_mode(value);
                self.set_divisor_code(value)
            }
            4 => {
                self.length_counter.set_enabled(value);
                self.trigger(value);
            }
            _ => {}
        }
    }
}
