pub mod gpu;
pub mod screen;

const SCREEN_MAX_PIXELS: usize = 256;
const SCREEN_WIDTH: u8 = 160;
const SCREEN_HEIGHT: u8 = 144;

#[derive(Clone, Copy)]
pub enum Pixel {
    Off,
    Dark,
    Light,
    On,
}
