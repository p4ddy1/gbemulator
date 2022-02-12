use lib_gbemulation::gpu::{Screen, BUFFER_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use crate::config::config::Config;

pub const MENU_BAR_HEIGHT: i32 = 19;

pub struct GameboyScreen {
    buffer1: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    buffer2: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    current_buffer: Arc<AtomicU8>,
    config: Arc<RwLock<Config>>
}

impl GameboyScreen {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        GameboyScreen {
            buffer1: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            buffer2: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            current_buffer: Arc::new(AtomicU8::new(1)),
            config
        }
    }

    pub fn draw_to_queue(&self, queue: &wgpu::Queue, texture: &wgpu::Texture, texture_size: wgpu::Extent3d) {
        let current_buffer = self.current_buffer.load(Ordering::SeqCst);

        let pixel_data = *if current_buffer == 1 {
            self.buffer1.lock().unwrap()
        } else {
            self.buffer2.lock().unwrap()
        };

        let mut texture_output = [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4];

        for (i, pixel) in texture_output.iter_mut().enumerate() {
            if i % 4 < 3 {
                let data_index = i - (i / 4);
                *pixel = pixel_data[data_index];
            } else {
                *pixel = 0;
            }
        }

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            },
            &texture_output,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * SCREEN_WIDTH as u32),
                rows_per_image: std::num::NonZeroU32::new(SCREEN_HEIGHT as u32),
            },
            texture_size
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

    fn get_palette(&self) -> [[u8; 3]; 4] {
        let palette = &self.config.read().unwrap().color_palette;
        [palette.color4, palette.color3, palette.color2, palette.color1]
    }
}
