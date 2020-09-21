use crate::apu::channel::frame_sequencer::FrameSequencer;
use crate::apu::channel::noise_channel::NoiseChannel;
use crate::apu::channel::square_channel::SquareChannel;
use crate::apu::channel::wave_channel::WaveChannel;
use crate::apu::mixer::Mixer;
use crate::apu::{AudioOutput, Channel};
use crate::emulation::CPU_CLOCK_HZ;
use crate::util::binary::is_bit_set;

const SQUARE_CHANNEL_1_START_ADDRESS: u16 = 0xFF10;
const SQUARE_CHANNEL_1_END_ADDRESS: u16 = 0xFF14;
const SQUARE_CHANNEL_2_START_ADDRESS: u16 = 0xFF15;
const SQUARE_CHANNEL_2_END_ADDRESS: u16 = 0xFF19;
const WAVE_CHANNEL_START_ADDRESS: u16 = 0xFF1A;
const WAVE_CHANNEL_END_ADDRESS: u16 = 0xFF1E;
const NOISE_CHANNEL_START_ADDRESS: u16 = 0xFF1F;
const NOISE_CHANNEL_END_ADDRESS: u16 = 0xFF23;

pub struct Apu<'a> {
    pub audio_output: &'a mut dyn AudioOutput,
    frame_sequencer: FrameSequencer,
    square_channel1: SquareChannel,
    square_channel2: SquareChannel,
    wave_channel: WaveChannel,
    noise_channel: NoiseChannel,
    mixer: Mixer,
    clock: u16,
    output_step: u16,
    enbaled: bool,
}

impl<'a> Apu<'a> {
    pub fn new(audio_output: &'a mut dyn AudioOutput) -> Self {
        let output_step = (CPU_CLOCK_HZ / audio_output.get_sample_rate() as usize) as u16;
        Apu {
            audio_output,
            frame_sequencer: FrameSequencer::new(),
            square_channel1: SquareChannel::new(SQUARE_CHANNEL_1_START_ADDRESS, true),
            square_channel2: SquareChannel::new(SQUARE_CHANNEL_2_START_ADDRESS, false),
            wave_channel: WaveChannel::new(WAVE_CHANNEL_START_ADDRESS),
            noise_channel: NoiseChannel::new(NOISE_CHANNEL_START_ADDRESS),
            mixer: Mixer::new(),
            clock: 0,
            output_step,
            enbaled: false,
        }
    }

    pub fn step(&mut self, clock_cycles: u8) {
        self.clock += clock_cycles as u16;

        if self.enbaled {
            self.frame_sequencer.step(clock_cycles);

            self.square_channel1
                .step(&self.frame_sequencer, clock_cycles);
            self.square_channel2
                .step(&self.frame_sequencer, clock_cycles);
            self.wave_channel.step(&self.frame_sequencer, clock_cycles);
            self.noise_channel.step(&self.frame_sequencer, clock_cycles);
        }

        //TODO: Do downsampling in a different way because this causes bad quality
        while self.clock >= self.output_step {
            let (output_left, output_right) = self.mixer.mix(
                self.enbaled,
                &self.square_channel1,
                &self.square_channel2,
                &self.wave_channel,
                &self.noise_channel,
            );

            self.audio_output.output((output_left, output_right));
            self.clock -= self.output_step;
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            SQUARE_CHANNEL_1_START_ADDRESS..=SQUARE_CHANNEL_1_END_ADDRESS => {
                self.square_channel1.write(address, value);
            }
            SQUARE_CHANNEL_2_START_ADDRESS..=SQUARE_CHANNEL_2_END_ADDRESS => {
                self.square_channel2.write(address, value);
            }
            WAVE_CHANNEL_START_ADDRESS..=WAVE_CHANNEL_END_ADDRESS => {
                self.wave_channel.write(address, value)
            }
            NOISE_CHANNEL_START_ADDRESS..=NOISE_CHANNEL_END_ADDRESS => {
                self.noise_channel.write(address, value)
            }
            0xFF24..=0xFF25 => self.mixer.write(address, value),
            0xFF26 => {
                self.enbaled = is_bit_set(&value, 7);
                if self.enbaled {
                    self.frame_sequencer.reset();
                }
            }
            0xFF30..=0xFF3F => self.wave_channel.write_wavetable(address, value),
            _ => {}
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        //TODO: Implement all the reads
        match address {
            0xFF24..=0xFF26 => self.mixer.read(address),
            _ => 0,
        }
    }
}
