pub mod gpu;
pub mod lcdc;
pub mod screen;
pub mod stat;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCALE: u8 = 4;

#[derive(Clone, Copy)]
pub enum Pixel {
    Off,
    Dark,
    Light,
    On,
}
