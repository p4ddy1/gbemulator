use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{
    EventLoop, Format, Host, OutputBuffer, Sample, StreamData, StreamId, UnknownTypeOutputBuffer,
};
use lib_gbemulation::apu::AudioOutput;

use crate::EmulationSignal;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, MutexGuard};
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
    sample_rate: Option<u32>,
    buffer: Arc<Mutex<AudioBuffer>>,
    host: Host,
    sync_sender: Option<Sender<EmulationSignal>>,
    event_loop: Arc<EventLoop>,
    current_stream_id: Option<StreamId>,
}

impl CpalAudioOutput {
    pub fn new(buffer_size: usize, sync_sender: Option<Sender<EmulationSignal>>) -> Self {
        let buffer = Arc::new(Mutex::new(AudioBuffer::new(buffer_size * 2)));

        let host = cpal::default_host();
        let event_loop = host.event_loop();

        CpalAudioOutput {
            sample_rate: None,
            buffer,
            host,
            sync_sender,
            event_loop: Arc::new(event_loop),
            current_stream_id: None,
        }
    }

    pub fn get_default_device_name(&self) -> String {
        self.host.default_output_device().unwrap().name().unwrap()
    }

    pub fn start(&mut self, device_name: String) {
        let mut device_list = self.host.devices().unwrap();

        let device = device_list
            .find(|dev| dev.name().unwrap() == device_name)
            .unwrap();

        let default_format = device.default_output_format().unwrap();
        let format = Format {
            channels: 2,
            sample_rate: default_format.sample_rate,
            data_type: default_format.data_type,
        };

        self.sample_rate = Some(format.sample_rate.0);

        let stream_id = self
            .event_loop
            .build_output_stream(&device, &format)
            .unwrap();
        self.current_stream_id = Some(stream_id.clone());

        self.event_loop.play_stream(stream_id).unwrap();

        let buffer = Arc::clone(&self.buffer);
        let sender = self.sync_sender.clone();

        let event_loop = Arc::clone(&self.event_loop);

        thread::Builder::new()
            .name("audio".to_string())
            .spawn(move || event_loop_runner(event_loop, buffer, sender))
            .unwrap();
    }

    pub fn stop(&self) {
        if let Some(stream_id) = &self.current_stream_id {
            self.event_loop.destroy_stream(stream_id.clone());
        }
    }
}

fn event_loop_runner(
    event_loop: Arc<EventLoop>,
    audio_buffer: Arc<Mutex<AudioBuffer>>,
    sync_sender: Option<Sender<EmulationSignal>>,
) {
    event_loop.run(move |_stream_id, stream_result| {
        let stream_data = stream_result.unwrap();

        if let StreamData::Output {
            buffer: buffer_type,
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

            match buffer_type {
                UnknownTypeOutputBuffer::F32(mut cpal_buffer) => {
                    fill_buffer(&mut buffer, &mut cpal_buffer, 0.0)
                }
                UnknownTypeOutputBuffer::I16(mut cpal_buffer) => {
                    fill_buffer(&mut buffer, &mut cpal_buffer, 0 as i16)
                }
                UnknownTypeOutputBuffer::U16(mut cpal_buffer) => {
                    fill_buffer(&mut buffer, &mut cpal_buffer, 0 as u16)
                }
            }
        }
    });
}

fn fill_buffer<T: Sample>(
    buffer: &mut MutexGuard<AudioBuffer>,
    cpal_buffer: &mut OutputBuffer<T>,
    default: T,
) {
    if buffer.data.len() < cpal_buffer.len() {
        println!("Audio Buffer underrun!");
        for sample in cpal_buffer.iter_mut() {
            *sample = default;
        }
        return;
    }

    for (i, sample) in buffer.data.drain(0..cpal_buffer.len()).enumerate() {
        cpal_buffer[i] = T::from(&sample);
    }
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
        match self.sample_rate {
            Some(sample_rate) => sample_rate,
            None => panic!("Sample rate is not set. Please initialize the audio device first"),
        }
    }
}
