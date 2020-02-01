use crate::gpu::{Pixel, SCALE, SCREEN_HEIGHT, SCREEN_MAX_PIXELS, SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub trait Screen {
    fn render(&mut self, screen_buffer: &[Pixel; 65536]);
    fn present(&mut self);
}

pub struct SdlScreen<'a> {
    pub width: u16,
    pub height: u16,
    canvas: Canvas<Window>,
    texture: Texture<'a>,
    buffer: Vec<u8>,
}

impl<'a> SdlScreen<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        width: u16,
        height: u16,
    ) -> SdlScreen<'a> {
        SdlScreen {
            width: width,
            height: height,
            canvas: canvas,
            texture: texture,
            buffer: vec![255; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * 3],
        }
    }

    fn draw_pixel_to_buffer(&mut self, y: usize, x: usize, r: u8, g: u8, b: u8) {
        let offset = (SCREEN_WIDTH * 3 * y) + x * 3;
        self.buffer[offset] = b;
        self.buffer[offset + 1] = g;
        self.buffer[offset + 2] = r;
    }
}

impl<'a> Screen for SdlScreen<'a> {
    fn render(&mut self, screen_buffer: &[Pixel; 65536]) {
        self.canvas.clear();

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel = screen_buffer[y as usize + 256 * x as usize];

                match pixel {
                    Pixel::On => self.draw_pixel_to_buffer(y, x, 255, 255, 255),
                    Pixel::Light => self.draw_pixel_to_buffer(y, x, 200, 200, 200),
                    Pixel::Dark => self.draw_pixel_to_buffer(y, x, 100, 100, 100),
                    Pixel::Off => self.draw_pixel_to_buffer(y, x, 0, 0, 0),
                }
            }
        }

        self.texture.update(
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            &self.buffer,
            SCREEN_WIDTH as usize * 3,
        );
        self.canvas
            .copy(
                &self.texture,
                None,
                Some(Rect::new(0, 0, self.width as u32, self.height as u32)),
            )
            .unwrap();
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}
