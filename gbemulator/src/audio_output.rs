use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{
    Device, EventLoop, Format, Host, SampleFormat, SampleRate, StreamData, UnknownTypeOutputBuffer,
};
use lib_gbemulation::apu::AudioOutput;

use crate::EmulationSignal;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

struct AudioBuffer {
    pub data: Vec<i16>,
    pub buffer_size: usize,
}

impl AudioBuffer {
    pub fn new(buffer_size: usize) -> Self {
        //Buffer needs to be twice as big because of stereo sound
        let size = buffer_size * 2;
        AudioBuffer {
            data: vec![0; size],
            buffer_size: size,
        }
    }
}

pub struct CpalAudioOutput {
    sample_rate: u32,
    buffer: Arc<Mutex<AudioBuffer>>,
    host: Host,
    selected_device: Option<Device>,
    sync_sender: Option<Sender<EmulationSignal>>,
}

impl CpalAudioOutput {
    pub fn new(
        sample_rate: u32,
        buffer_size: usize,
        sync_sender: Option<Sender<EmulationSignal>>,
    ) -> Self {
        let buffer = Arc::new(Mutex::new(AudioBuffer::new(buffer_size * 2)));

        let host = cpal::default_host();

        CpalAudioOutput {
            sample_rate,
            buffer,
            host,
            selected_device: None,
            sync_sender,
        }
    }

    pub fn get_output_device_names(&self) -> Vec<String> {
        let devices = self.host.devices().unwrap();
        devices.map(|dev| dev.name().unwrap()).collect()
    }

    pub fn set_device(&mut self, device_name: String) {
        let mut devices = self.host.devices().unwrap();
        let device = devices
            .find(|dev| dev.name().unwrap() == device_name)
            .unwrap();
        self.selected_device = Some(device);
    }

    pub fn start(&self) {
        let format = Format {
            channels: 2,
            sample_rate: SampleRate(self.sample_rate),
            data_type: SampleFormat::I16,
        };

        let event_loop = self.host.event_loop();

        let stream_id = if let Some(selected_device) = &self.selected_device {
            event_loop
                .build_output_stream(selected_device, &format)
                .unwrap()
        } else {
            event_loop
                .build_output_stream(&self.host.default_output_device().unwrap(), &format)
                .unwrap()
        };

        event_loop.play_stream(stream_id).unwrap();

        let buffer = Arc::clone(&self.buffer);
        let sender = self.sync_sender.clone();

        thread::Builder::new()
            .name("audio".to_string())
            .spawn(move || event_loop_runner(event_loop, buffer, sender))
            .unwrap();
    }
}

fn event_loop_runner(
    event_loop: EventLoop,
    audio_buffer: Arc<Mutex<AudioBuffer>>,
    sync_sender: Option<Sender<EmulationSignal>>,
) {
    event_loop.run(move |_stream_id, stream_result| {
        let stream_data = stream_result.unwrap();

        if let StreamData::Output {
            buffer: UnknownTypeOutputBuffer::I16(mut cpal_buffer),
        } = stream_data
        {
            let mut buffer = audio_buffer.lock().unwrap();

            if let Some(sender) = &sync_sender {
                if buffer.data.len() < buffer.buffer_size / 2 {
                    match sender.send(EmulationSignal::Cycle) {
                        Ok(_) => {}
                        Err(_) => {
                            return;
                        }
                    }
                }
            }

            //We dont have enough data to satisfy the output
            if buffer.data.len() < cpal_buffer.len() {
                println!("Audio Buffer underrun!");
                for sample in cpal_buffer.iter_mut() {
                    *sample = 0;
                }
                return;
            }

            for (i, sample) in buffer.data.drain(0..cpal_buffer.len()).enumerate() {
                cpal_buffer[i] = sample;
            }
        }
    });
}

impl AudioOutput for CpalAudioOutput {
    fn output(&mut self, sample: (i16, i16)) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.data.len() < buffer.buffer_size {
            buffer.data.push(sample.0);
            buffer.data.push(sample.1);
        }
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }
}
