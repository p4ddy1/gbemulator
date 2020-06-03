use lib_gbemulation::gpu::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH};

use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use std::sync::Mutex;

pub struct SdlScreen<'a> {
    pub width: u16,
    pub height: u16,
    canvas: Canvas<Window>,
    texture: Texture<'a>,
    screen_buffer: &'a ScreenBuffer,
}

impl<'a> SdlScreen<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        width: u16,
        height: u16,
        screen_buffer: &'a ScreenBuffer,
    ) -> SdlScreen<'a> {
        SdlScreen {
            width: width,
            height: height,
            canvas: canvas,
            texture: texture,
            screen_buffer,
        }
    }

    fn output_buffer(&mut self) {
        let current_buffer = self.screen_buffer.current_buffer.lock().unwrap();

        let buffer = *if *current_buffer == 1 {
            self.screen_buffer.buffer1.lock().unwrap()
        } else {
            self.screen_buffer.buffer2.lock().unwrap()
        };

        self.texture
            .update(
                Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
                &buffer,
                SCREEN_WIDTH as usize * 3,
            )
            .unwrap();

        self.canvas
            .copy(
                &self.texture,
                None,
                Some(Rect::new(0, 0, self.width as u32, self.height as u32)),
            )
            .unwrap();
    }

    pub fn present(&mut self) {
        self.output_buffer();
        self.canvas.present();
    }
}

pub struct ScreenBuffer {
    pub buffer1: Mutex<[u8; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]>,
    pub buffer2: Mutex<[u8; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]>,
    pub current_buffer: Mutex<u8>,
}

impl ScreenBuffer {
    pub fn new() -> Self {
        ScreenBuffer {
            buffer1: Mutex::new([255; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]),
            buffer2: Mutex::new([255; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]),
            current_buffer: Mutex::new(1),
        }
    }
}

impl Screen for ScreenBuffer {
    fn draw(&self, screen_buffer: &[u8; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]) {
        let mut current_buffer = self.current_buffer.lock().unwrap();

        let mut buffer = if *current_buffer == 1 {
            self.buffer1.lock().unwrap()
        } else {
            self.buffer2.lock().unwrap()
        };

        *buffer = *screen_buffer;

        if *current_buffer == 1 {
            *current_buffer = 2;
        } else {
            *current_buffer = 1;
        }
    }
}
