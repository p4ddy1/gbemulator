use crate::gpu::{Pixel, SCREEN_HEIGHT, SCREEN_MAX_PIXELS, SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u8 = 4;

pub trait Screen {
    fn render(&mut self, screen_buffer: &[[Pixel; SCREEN_MAX_PIXELS]; SCREEN_MAX_PIXELS]);
}

//TODO: This rendering module in inefficent and should only used for testing
pub struct SdlScreen {
    pub width: u8,
    pub height: u8,
    canvas: Canvas<Window>,
}

impl SdlScreen {
    pub fn new() -> SdlScreen {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "gb",
                SCREEN_WIDTH as u32 * SCALE as u32,
                SCREEN_HEIGHT as u32 * SCALE as u32,
            )
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        SdlScreen {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            canvas: canvas,
        }
    }

    fn draw_pixel(&mut self, y: i32, x: i32) {
        self.canvas
            .fill_rect(Rect::new(
                x * SCALE as i32,
                y * SCALE as i32,
                SCALE as u32,
                SCALE as u32,
            ))
            .unwrap();
    }
}

impl Screen for SdlScreen {
    fn render(&mut self, screen_buffer: &[[Pixel; SCREEN_MAX_PIXELS]; SCREEN_MAX_PIXELS]) {
        self.canvas.clear();

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let pixel = screen_buffer[y][x];
                match pixel {
                    Pixel::On => {
                        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }
                    Pixel::Light => {
                        self.canvas.set_draw_color(Color::RGB(200, 200, 200));
                    }
                    Pixel::Dark => {
                        self.canvas.set_draw_color(Color::RGB(100, 100, 100));
                    }
                    Pixel::Off => {
                        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                    }
                }
                self.draw_pixel(y as i32, x as i32);
            }
        }

        self.canvas.present();
    }
}
