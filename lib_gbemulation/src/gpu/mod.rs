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
    fn render(&mut self, screen_buffer: &[Pixel; 65792]);
    fn present(&mut self);
}