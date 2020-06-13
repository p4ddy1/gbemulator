use crate::apu::channel::noise_channel::NoiseChannel;
use crate::apu::channel::square_channel::SquareChannel;
use crate::apu::channel::wave_channel::WaveChannel;
use crate::apu::Channel;

const BASE_ADDRESS: u16 = 0xFF24;

pub struct Mixer {
    square1_left_enabled: bool,
    square1_right_enabled: bool,
    square2_left_enabled: bool,
    square2_right_enabled: bool,
    wave_left_enabled: bool,
    wave_right_enabled: bool,
    noise_left_enabled: bool,
    noise_right_enabled: bool,
}

impl Mixer {
    pub fn new() -> Self {
        Mixer {
            square1_left_enabled: false,
            square1_right_enabled: false,
            square2_left_enabled: false,
            square2_right_enabled: false,
            wave_left_enabled: false,
            wave_right_enabled: false,
            noise_left_enabled: false,
            noise_right_enabled: false,
        }
    }

    pub fn mix(
        &self,
        enabled: bool,
        square_channel1: &SquareChannel,
        square_channel2: &SquareChannel,
        wave_channel: &WaveChannel,
        noise_channel: &NoiseChannel,
    ) -> (i16, i16) {
        let mut output_left = 0;
        let mut output_right = 0;

        if !enabled {
            return (output_left, output_right);
        }

        mix_channel(
            &mut output_left,
            &mut output_right,
            self.square1_left_enabled,
            self.square1_right_enabled,
            square_channel1,
        );

        mix_channel(
            &mut output_left,
            &mut output_right,
            self.square2_left_enabled,
            self.square2_right_enabled,
            square_channel2,
        );

        mix_channel(
            &mut output_left,
            &mut output_right,
            self.wave_left_enabled,
            self.wave_right_enabled,
            wave_channel,
        );

        mix_channel(
            &mut output_left,
            &mut output_right,
            self.noise_left_enabled,
            self.noise_right_enabled,
            noise_channel,
        );

        (output_left, output_right)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < BASE_ADDRESS {
            return;
        }

        let register = address - BASE_ADDRESS;

        match register {
            0 => {} //TODO: Implement,
            1 => self.set_channel_enables(value),
            _ => {}
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if address < BASE_ADDRESS {
            return 0;
        }

        let register = address - BASE_ADDRESS;

        match register {
            0 => 0, //TODO: Implement,
            1 => self.get_channel_enables(),
            _ => 0,
        }
    }

    fn set_channel_enables(&mut self, value: u8) {
        self.square1_left_enabled = value & 0x10 == 0x10;
        self.square1_right_enabled = value & 0x01 == 0x01;
        self.square2_left_enabled = value & 0x20 == 0x20;
        self.square2_right_enabled = value & 0x02 == 0x02;
        self.wave_left_enabled = value & 0x40 == 0x40;
        self.wave_right_enabled = value & 0x04 == 0x04;
        self.noise_left_enabled = value & 0x80 == 0x80;
        self.noise_right_enabled = value & 0x08 == 0x08;
    }

    fn get_channel_enables(&self) -> u8 {
        (if self.square1_left_enabled { 0x10 } else { 0 })
            | (if self.square1_right_enabled { 0x01 } else { 0 })
            | (if self.square2_left_enabled { 0x20 } else { 0 })
            | (if self.square2_right_enabled { 0x02 } else { 0 })
            | (if self.wave_left_enabled { 0x40 } else { 0 })
            | (if self.wave_right_enabled { 0x04 } else { 0 })
            | (if self.noise_left_enabled { 0x80 } else { 0 })
            | (if self.noise_right_enabled { 0x08 } else { 0 })
    }
}

fn mix_channel(
    buffer_left: &mut i16,
    buffer_right: &mut i16,
    left_enable: bool,
    right_enable: bool,
    channel: &dyn Channel,
) {
    let signal = channel.output() / 4;

    if left_enable {
        *buffer_left += signal;
    }

    if right_enable {
        *buffer_right += signal;
    }
}
