use crate::apu::channel::frame_sequencer::FrameSequencer;
use crate::apu::channel::frequency_sweep::{FrequencySweep, FrequencySweepResult};
use crate::apu::channel::length_counter::{LengthCounter, LengthCounterResult};
use crate::apu::channel::volume_envelope::VolumeEnvelope;
use crate::apu::Channel;
use crate::util::binary::is_bit_set;
use std::i16;

const DUTY_MAP: [[i16; 8]; 4] = [
    [-1, -1, -1, -1, -1, -1, -1, 1], //12.5%
    [1, -1, -1, -1, -1, -1, -1, 1],  //25%
    [1, -1, -1, -1, -1, 1, 1, 1],    //50%
    [-1, 1, 1, 1, 1, 1, 1, -1],      //75%
];

pub struct SquareChannel {
    frequency: u16,
    frequency_sweep: Option<FrequencySweep>,
    duty: u8,
    volume_envelope: VolumeEnvelope,
    length_counter: LengthCounter,
    timer: i16,
    waveform_pointer: u8,
    enabled: bool,
    base_address: u16,
}

impl SquareChannel {
    pub fn new(base_address: u16, sweep_enabled: bool) -> Self {
        let frequency_sweep = if sweep_enabled {
            Some(FrequencySweep::new())
        } else {
            None
        };

        SquareChannel {
            frequency: 0,
            frequency_sweep,
            duty: 0,
            volume_envelope: VolumeEnvelope::new(),
            length_counter: LengthCounter::new(64),
            timer: 0,
            waveform_pointer: 0,
            enabled: false,
            base_address,
        }
    }

    pub fn set_frequency_lsb(&mut self, value: u8) {
        self.frequency = self.frequency & 0x700 | value as u16;
    }

    pub fn set_frequency_msb(&mut self, value: u8) {
        self.frequency = self.frequency & 0xFF | (value as u16 & 0x7) << 8;
    }

    pub fn set_length_counter_length(&mut self, value: u8) {
        self.length_counter.set_length(value & 0x3F);
    }

    pub fn set_duty(&mut self, value: u8) {
        self.duty = (value & 0xC0) >> 6;
    }

    pub fn trigger(&mut self, value: u8) {
        //Do nothing if bit 7 is not set
        if !is_bit_set(&value, 7) {
            return;
        }

        self.enabled = true;

        self.volume_envelope.trigger();

        self.length_counter.trigger();
        self.timer = self.get_period();

        if let Some(ref mut frequency_sweep) = self.frequency_sweep {
            let result = frequency_sweep.trigger(self.frequency);
            self.handle_sweep_result(result)
        }
    }

    fn get_period(&self) -> i16 {
        (2048 - self.frequency as i16) * 4
    }

    fn handle_volume_envelope(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.volume_envelope_trigger {
            self.volume_envelope.step();
        }
    }

    fn handle_length_counter(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.length_counter_trigger {
            if let LengthCounterResult::DisableChannel = self.length_counter.step() {
                self.enabled = false;
            }
        }
    }

    fn handle_frequency_sweep(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.sweep_timer_trigger {
            if let Some(ref mut frequency_sweep) = self.frequency_sweep {
                let result = frequency_sweep.step();
                self.handle_sweep_result(result);
            }
        }
    }

    fn handle_sweep_result(&mut self, result: FrequencySweepResult) {
        match result {
            FrequencySweepResult::Overflowed => {
                self.enabled = false;
            }
            FrequencySweepResult::Sweeped(frequency) => {
                self.frequency = frequency;
            }
            _ => {}
        }
    }
}

impl Channel for SquareChannel {
    fn step(&mut self, frame_sequencer: &FrameSequencer, clock_cycles: u8) {
        if !self.enabled {
            return;
        }

        self.handle_volume_envelope(frame_sequencer);
        self.handle_length_counter(frame_sequencer);
        self.handle_frequency_sweep(frame_sequencer);

        if self.timer <= 0 {
            self.timer += self.get_period();

            self.waveform_pointer += 1;

            if self.waveform_pointer > 7 {
                self.waveform_pointer = 0;
            }
        }

        if self.get_period() >= clock_cycles as i16 {
            self.timer -= clock_cycles as i16;
        }
    }

    fn output(&self) -> i16 {
        if !self.enabled {
            return 0;
        }

        let signal = DUTY_MAP[self.duty as usize][self.waveform_pointer as usize] * i16::MAX;

        self.volume_envelope.process_signal(signal)
    }

    fn write(&mut self, address: u16, value: u8) {
        if address < self.base_address {
            return;
        }

        let register = address - self.base_address;

        match register {
            0 => {
                if let Some(ref mut frequency_sweep) = self.frequency_sweep {
                    frequency_sweep.write(value);
                }
            }
            1 => {
                self.set_duty(value);
                self.set_length_counter_length(value);
            }
            2 => self.volume_envelope.write(value),
            3 => self.set_frequency_lsb(value),
            4 => {
                self.set_frequency_msb(value);

                self.length_counter.set_enabled(value);
                self.trigger(value);
            }
            _ => {}
        }
    }
}
