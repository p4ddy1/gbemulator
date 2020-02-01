pub mod gpu;
pub mod screen;

const SCREEN_MAX_PIXELS: usize = 256;
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
