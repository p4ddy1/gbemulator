use crate::apu::channel::frame_sequencer::FrameSequencer;

pub mod apu;
mod channel;
mod mixer;

pub trait AudioOutput {
    fn output(&mut self, sample: (i16, i16));
    fn get_sample_rate(&self) -> u32;
}

trait Channel {
    fn output(&self) -> i16;
    fn step(&mut self, frame_sequencer: &FrameSequencer, clock_cycles: u8);
    fn write(&mut self, address: u16, value: u8);
}
