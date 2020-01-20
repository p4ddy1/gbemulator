pub mod gpu;
pub mod screen;

const SCREEN_MAX_PIXELS: usize = 256;
pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;
pub const SCALE: u8 = 4;

#[derive(Clone, Copy)]
pub enum Pixel {
    Off,
    Dark,
    Light,
    On,
}
