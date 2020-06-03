use crate::apu::channel::frame_sequencer::FrameSequencer;
use crate::apu::channel::length_counter::{LengthCounter, LengthCounterResult};
use crate::apu::Channel;
use crate::util::binary::is_bit_set;
use std::i16;

const WAVETABLE_START_ADDRESS: u16 = 0xFF30;

pub struct WaveChannel {
    frequency: u16,
    length_counter: LengthCounter,
    timer: i16,
    wavetable_pointer: u8,
    enabled: bool,
    wavetable: [u8; 32],
    volume_code: u8,
    dac_enabled: bool,
    base_address: u16,
}

impl WaveChannel {
    pub fn new(base_address: u16) -> Self {
        WaveChannel {
            frequency: 0,
            length_counter: LengthCounter::new(256),
            timer: 0,
            wavetable_pointer: 0,
            enabled: false,
            wavetable: [0; 32],
            volume_code: 0,
            dac_enabled: false,
            base_address,
        }
    }

    pub fn set_frequency_lsb(&mut self, value: u8) {
        self.frequency = self.frequency & 0x700 | value as u16;
    }

    pub fn set_frequency_msb(&mut self, value: u8) {
        self.frequency = self.frequency & 0xFF | (value as u16 & 0x7) << 8;
    }

    pub fn set_volume_code(&mut self, value: u8) {
        self.volume_code = (value & 0x60) >> 5;
    }

    pub fn set_dac_power(&mut self, value: u8) {
        if is_bit_set(&value, 7) {
            self.dac_enabled = true;
            return;
        }

        self.dac_enabled = false;
        self.enabled = false;
    }

    pub fn write_wavetable(&mut self, address: u16, value: u8) {
        let position = (address - WAVETABLE_START_ADDRESS) * 2;
        self.wavetable[position as usize] = (value & 0xF0) >> 4;
        self.wavetable[position as usize + 1] = value & 0xF;
    }

    pub fn trigger(&mut self, value: u8) {
        //Do nothing if bit 7 is not set
        if !is_bit_set(&value, 7) {
            return;
        }

        if self.dac_enabled {
            self.enabled = true;
        }

        self.timer = self.get_period();
        self.length_counter.trigger();
        self.wavetable_pointer = 0;
    }

    fn get_period(&self) -> i16 {
        (2048 - self.frequency as i16) * 2
    }

    fn handle_length_counter(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.length_counter_trigger {
            if let LengthCounterResult::DisableChannel = self.length_counter.step() {
                self.enabled = false;
            }
        }
    }

    fn get_volume_shift_amount(&self) -> i16 {
        match self.volume_code {
            0 => 4, //0%
            1 => 0, //100%
            2 => 1, //50%
            3 => 2, //25%
            _ => 4,
        }
    }

    fn process_signal(&self, sample: u8) -> i16 {
        if !self.dac_enabled || self.volume_code == 0 {
            return 0;
        }

        let volume_shift = self.get_volume_shift_amount();

        //Map values of 0-15 to signed integer. 0 = -32768 15 = 32767
        let out = if sample <= 7 {
            ((i16::MAX / 7) * (7 - sample as i16)) * -1
        } else {
            (i16::MAX / 7) * (sample as i16 - 8)
        };

        out >> volume_shift
    }
}

impl Channel for WaveChannel {
    fn step(&mut self, frame_sequencer: &FrameSequencer, clock_cycles: u8) {
        if !self.enabled {
            return;
        }

        self.handle_length_counter(frame_sequencer);

        if self.timer <= 0 {
            self.timer += self.get_period();

            self.wavetable_pointer += 1;

            if self.wavetable_pointer > 31 {
                self.wavetable_pointer = 0;
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

        let sample = self.wavetable[self.wavetable_pointer as usize];

        self.process_signal(sample)
    }

    fn write(&mut self, address: u16, value: u8) {
        if address < self.base_address {
            return;
        }

        let register = address - self.base_address;

        match register {
            0 => self.set_dac_power(value),
            1 => self.length_counter.set_length(value),
            2 => self.set_volume_code(value),
            3 => self.set_frequency_lsb(value),
            4 => {
                self.trigger(value);
                self.set_frequency_msb(value);
                self.length_counter.set_enabled(value);
            }
            _ => {}
        }
    }
}
