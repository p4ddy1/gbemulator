use glium::backend::Facade;
use glium::texture::{MipmapsOption, RawImage2d, UncompressedFloatFormat};
use glium::{BlitTarget, Frame, Surface};
use lib_gbemulation::gpu::{Screen, BUFFER_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

pub const MENU_BAR_HEIGHT: i32 = 19;

pub struct GameboyScreen {
    buffer1: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    buffer2: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    current_buffer: Arc<AtomicU8>,
}

impl GameboyScreen {
    pub fn new() -> Self {
        GameboyScreen {
            buffer1: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            buffer2: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            current_buffer: Arc::new(AtomicU8::new(1)),
        }
    }

    pub fn draw_to_frame(&self, facade: &dyn Facade, frame: &mut Frame, width: u32, height: u32) {
        let current_buffer = self.current_buffer.load(Ordering::SeqCst);

        let data = *if current_buffer == 1 {
            self.buffer1.lock().unwrap()
        } else {
            self.buffer2.lock().unwrap()
        };

        let screen =
            RawImage2d::from_raw_rgb_reversed(&data, (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));

        let texture = glium::texture::Texture2d::with_format(
            facade,
            screen,
            UncompressedFloatFormat::U8U8U8,
            MipmapsOption::NoMipmap,
        )
        .unwrap();

        let blit_target = BlitTarget {
            left: 0,
            bottom: 0 as u32,
            width: width as i32,
            height: height as i32 - MENU_BAR_HEIGHT,
        };

        texture.as_surface().blit_whole_color_to(
            frame,
            &blit_target,
            glium::uniforms::MagnifySamplerFilter::Nearest,
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
