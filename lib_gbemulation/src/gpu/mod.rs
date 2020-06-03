pub mod gpu;
pub mod lcdc;
pub mod stat;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCALE: u8 = 4;

#[derive(Clone, Copy)]
pub enum Pixel {
    Color3,
    Color2,
    Color1,
    Color0,
}

pub trait Screen {
    fn draw(&self, screen_buffer: &[u8; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]);
}
