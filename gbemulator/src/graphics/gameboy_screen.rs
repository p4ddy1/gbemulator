use glium::backend::Facade;
use glium::texture::{MipmapsOption, RawImage2d, UncompressedFloatFormat};
use glium::{Frame, Surface, Texture2d, Rect};
use lib_gbemulation::gpu::{Screen, BUFFER_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::rc::Rc;

pub struct GameboyScreen {
    buffer1: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    buffer2: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    current_buffer: Arc<AtomicU8>
}

impl GameboyScreen {
    pub fn new() -> Self {
        GameboyScreen {
            buffer1: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            buffer2: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            current_buffer: Arc::new(AtomicU8::new(1)),
        }
    }

    pub fn draw_to_texture(&self, texture: &Rc<Texture2d>) {
        let current_buffer = self.current_buffer.load(Ordering::SeqCst);

        let data = *if current_buffer == 1 {
            self.buffer1.lock().unwrap()
        } else {
            self.buffer2.lock().unwrap()
        };

        let screen =
            RawImage2d::from_raw_rgb(data.to_vec(), (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));

        texture.write(
            Rect {left: 0, bottom: 0, width: SCREEN_WIDTH as u32, height: SCREEN_HEIGHT as u32},
            screen
        );
    }
}

impl Screen for GameboyScreen {
    fn draw(&self, screen_buffer: &[u8; BUFFER_SIZE]) {
        let mut buffer = if self.current_buffer.load(Ordering::SeqCst) == 1 {
            self.current_buffer.store(2, Ordering::SeqCst);
            self.buffer2.lock().unwrap()
        } else {
            self.current_buffer.store(1, Ordering::SeqCst);
            self.buffer1.lock().unwrap()
        };

        *buffer = *screen_buffer;
    }
}
