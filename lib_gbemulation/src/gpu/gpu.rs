use crate::gpu::screen::Screen;
use crate::gpu::Pixel;
use crate::memory::interrupts::Interrupt;
use crate::memory::mmu::Mmu;
use crate::util::binary::{is_bit_set, reset_bit_in_byte, set_bit_in_byte};

const OAM_ADDRESS: u16 = 0xFE00;
const TILESET_FIRST_BEGIN_ADDRESS: u16 = 0x8000;
const TILESET_SECOND_BEGIN_ADDRESS: u16 = 0x9000;
const BGMAP_FIRST_BEGIN_ADDRESS: u16 = 0x9800;
const BGMAP_SECOND_BEGIN_ADDRESS: u16 = 0x9C00;

const CYCLES_OAM: u16 = 80;
const CYCLES_VRAM: u16 = 172;
const CYCLES_HBLANK: u16 = 204;
const CYCLES_VBLANK: u16 = 456;

const SCANLINES_DISPLAY: u8 = 143;
const MAX_SCANLINES: u8 = 153;

#[derive(Copy, Clone)]
enum Mode {
    Oam = 2,
    Vram = 3,
    Hblank = 0,
    Vblank = 1,
}

pub struct Gpu<'a> {
    clock: u16,
    mode: Mode,
    pub screen: &'a mut dyn Screen,
    screen_buffer: [Pixel; 65792],
    bg_pal: [Pixel; 4],
    sprite_palette0: [Pixel; 4],
    sprite_palette1: [Pixel; 4],
}

impl<'a> Gpu<'a> {
    pub fn new(screen: &'a mut dyn Screen) -> Gpu<'a> {
        Gpu {
            clock: 0,
            mode: Mode::Hblank,
            screen: screen,
            screen_buffer: [Pixel::Off; 65792],
            bg_pal: [Pixel::On, Pixel::Light, Pixel::Dark, Pixel::Off],
            sprite_palette0: [Pixel::On, Pixel::Light, Pixel::Dark, Pixel::Off],
            sprite_palette1: [Pixel::On, Pixel::Light, Pixel::Dark, Pixel::Off],
        }
    }

    fn compare_lyc(&mut self, mmu: &mut Mmu) {
        mmu.io_bus.stat = reset_bit_in_byte(mmu.io_bus.stat, 2);
        if mmu.io_bus.lyc == mmu.io_bus.current_scanline {
            mmu.io_bus.stat = set_bit_in_byte(mmu.io_bus.stat, 2);
            if is_bit_set(&mmu.io_bus.stat, 6) {
                mmu.interrupts.fire_interrupt(&Interrupt::LcdStat);
            } //TODO: Is this correct?
        }
    }

    fn set_mode(&mut self, mmu: &mut Mmu, mode: Mode) {
        mmu.io_bus.stat &= 0xC0;
        mmu.io_bus.stat |= mode as u8 & 0x3F;
        self.mode = mode;
        self.fire_stat_interrupt(mmu);
    }

    fn fire_stat_interrupt(&mut self, mmu: &mut Mmu) {
        match self.mode {
            Mode::Oam => {
                if is_bit_set(&mmu.io_bus.stat, 5) {
                    mmu.interrupts.fire_interrupt(&Interrupt::LcdStat);
                }
            }
            Mode::Vram => {
                if is_bit_set(&mmu.io_bus.stat, 3) {
                    mmu.interrupts.fire_interrupt(&Interrupt::LcdStat);
                }
            }
            Mode::Vblank => {
                if is_bit_set(&mmu.io_bus.stat, 4) {
                    mmu.interrupts.fire_interrupt(&Interrupt::LcdStat);
                }
            }
            _ => {}
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, cycles: u8) {
        self.clock += cycles as u16;
        set_palette(&mut self.bg_pal, mmu.io_bus.bg_palette);
        set_palette(&mut self.sprite_palette0, mmu.io_bus.sprite_palette0);
        set_palette(&mut self.sprite_palette1, mmu.io_bus.sprite_palette1);
        self.step_set_mode(mmu);
        self.compare_lyc(mmu);
    }

    fn step_set_mode(&mut self, mmu: &mut Mmu) {
        match self.mode {
            Mode::Oam => {
                if self.clock >= CYCLES_OAM {
                    self.set_mode(mmu, Mode::Vram);
                    self.clock = 0;
                }
            }
            Mode::Vram => {
                if self.clock >= CYCLES_VRAM {
                    self.render_scanline_to_screen(mmu);
                    self.set_mode(mmu, Mode::Hblank);
                    self.clock = 0;
                }
            }
            Mode::Hblank => {
                if self.clock >= CYCLES_HBLANK {
                    self.clock = 0;
                    mmu.io_bus.current_scanline += 1;
                    if mmu.io_bus.current_scanline > SCANLINES_DISPLAY {
                        self.set_mode(mmu, Mode::Vblank);
                        self.screen.render(&self.screen_buffer);
                        mmu.interrupts.fire_interrupt(&Interrupt::Vblank);
                    } else {
                        self.set_mode(mmu, Mode::Oam);
                    }
                }
            }
            Mode::Vblank => {
                if self.clock >= CYCLES_VBLANK {
                    mmu.io_bus.current_scanline += 1;
                    self.clock = 0;
                    if mmu.io_bus.current_scanline > MAX_SCANLINES {
                        self.set_mode(mmu, Mode::Oam);
                        mmu.io_bus.current_scanline = 0;
                    }
                }
            }
        }
    }

    fn clear_screen(&mut self) {
        for i in 0..256 * 256 + 256 {
            self.screen_buffer[i] = Pixel::On;
        }
    }

    fn render_scanline_to_screen(&mut self, mmu: &mut Mmu) {
        //Bit 7 = LCD Enable. Disabled? Render nothing
        if !is_bit_set(&mmu.io_bus.lcdc, 7) {
            self.clear_screen();
            return;
        }

        self.render_background_line(mmu);

        if is_bit_set(&mmu.io_bus.lcdc, 1) {
            self.render_sprite_line(mmu);
        }
    }

    fn render_sprite_line(&mut self, mmu: &mut Mmu) {
        let current_line = mmu.io_bus.current_scanline as i16;

        let mut sprite_height = 8;

        if is_bit_set(&mmu.io_bus.lcdc, 2) {
            sprite_height = 16;
        }

        for sprite_count in 0..40 {
            //Each sprite is consists for 4 bytes
            //0 = Y, 1 = X, 2 = Tile, 3 = Options
            let sprite_begin_address = OAM_ADDRESS + sprite_count * 4;

            let sprite_y = mmu.read_oam(sprite_begin_address) as i16 - 16;
            let sprite_x = mmu.read_oam(sprite_begin_address + 1) as i16 - 8;

            //Check if tile is at current scanline
            if current_line >= sprite_y && current_line < sprite_y + sprite_height {
                let sprite_tile = mmu.read_oam(sprite_begin_address + 2);
                let sprite_options = mmu.read_oam(sprite_begin_address + 3);

                let tile_begin_address = TILESET_FIRST_BEGIN_ADDRESS + (sprite_tile as u16 * 16);

                let line_offset = flip_y(&sprite_options, current_line, sprite_height, sprite_y);

                //Each tile consists of one byte at the y axes
                let tile_data_address = tile_begin_address + (line_offset * 2) as u16;
                //The color data sits one byte after the pixel data
                let tile_color_data_address = tile_begin_address + (line_offset * 2) as u16 + 1;

                let tile_data = mmu.read_vram(tile_data_address);
                let tile_color_data = mmu.read_vram(tile_color_data_address);

                let sprite_palette = if is_bit_set(&sprite_options, 4) {
                    &self.sprite_palette1
                } else {
                    &self.sprite_palette0
                };

                for x in 0..8 {
                    let x_offset = sprite_x + x as i16;
                    if x_offset < 0 || x_offset > 160 {
                        continue;
                    }

                    let total_offset = current_line as usize + 256 * x_offset as usize;

                    if self.background_has_priority_over_pixel(&sprite_options, total_offset) {
                        continue;
                    }

                    let pixel_index = flip_x(&sprite_options, x);

                    let pixel = get_pixel(
                        sprite_palette,
                        tile_data,
                        tile_color_data,
                        pixel_index,
                        true,
                    );

                    match pixel {
                        Some(value) => self.screen_buffer[total_offset] = value,
                        _ => {}
                    }
                }
            }
        }
    }

    fn background_has_priority_over_pixel(&self, sprite_options: &u8, offset: usize) -> bool {
        if !is_bit_set(&sprite_options, 7) {
            return false;
        }

        match self.screen_buffer[offset] {
            Pixel::On => return false,
            _ => {}
        };

        true
    }

    fn render_background_line(&mut self, mmu: &mut Mmu) {
        let y_bgmap = mmu
            .io_bus
            .current_scanline
            .wrapping_add(mmu.io_bus.scroll_y);

        let window_enabled = is_bit_set(&mmu.io_bus.lcdc, 5);

        let line_is_window = window_enabled && mmu.io_bus.current_scanline >= mmu.io_bus.window_y;

        for x in 0..=160_u8 {
            let x_bgmap = x.wrapping_add(mmu.io_bus.scroll_x);

            let column_is_window = window_enabled && x >= mmu.io_bus.window_x - 7;

            let tile_address = if line_is_window && column_is_window {
                calculate_window_address(&mmu, mmu.io_bus.current_scanline, x)
            } else {
                calculate_bgmap_address(&mmu.io_bus.lcdc, y_bgmap, x_bgmap)
            };

            let tile = mmu.read_vram(tile_address);

            let tile_begin_address = calculate_tile_address(&mmu.io_bus.lcdc, tile);

            //Each tile consists of one byte at the y axes
            let y_tile_address_offset = if line_is_window && column_is_window {
                (mmu.io_bus.current_scanline - mmu.io_bus.window_y) % 8 * 2
            } else {
                y_bgmap % 8 * 2
            } as u16;

            let tile_data_address = tile_begin_address + y_tile_address_offset;
            //The color data sits one byte after the pixel data
            let tile_color_data_address = tile_data_address + 1;

            let tile_data = mmu.read_vram(tile_data_address);
            let tile_color_data = mmu.read_vram(tile_color_data_address);

            let pixel_index = if column_is_window && line_is_window {
                mmu.io_bus.window_x - x
            } else {
                7 - (x_bgmap % 8)
            };

            let pixel =
                get_pixel(&self.bg_pal, tile_data, tile_color_data, pixel_index, false).unwrap();

            self.screen_buffer[mmu.io_bus.current_scanline as usize + 256 * x as usize] = pixel;
        }
    }
}

fn get_pixel(
    palette: &[Pixel],
    tile_data: u8,
    tile_color_data: u8,
    pixel: u8,
    sprite: bool,
) -> Option<Pixel> {
    let pixel_is_active = is_bit_set(&tile_data, pixel);
    let pixel_color_bit = is_bit_set(&tile_color_data, pixel);

    if pixel_is_active && pixel_color_bit {
        return Some(palette[3]);
    }

    if pixel_is_active && !pixel_color_bit {
        return Some(palette[1]);
    }

    //On sprite rendering this is transparent
    if !pixel_is_active && !pixel_color_bit && !sprite {
        return Some(palette[0]);
    }

    if !pixel_is_active && pixel_color_bit {
        return Some(palette[2]);
    }

    None
}

/// Checks the sprite options if x flip is needed and performs it
fn flip_x(sprite_options: &u8, x: u8) -> u8 {
    if is_bit_set(&sprite_options, 5) {
        return x;
    }

    7 - x
}
/// Checks the sprite options if x flip is needed and performs it
fn flip_y(sprite_options: &u8, current_line: i16, sprite_height: i16, y: i16) -> i16 {
    if is_bit_set(&sprite_options, 6) {
        return sprite_height - 1 - (current_line - y);
    }
    current_line - y
}

fn calculate_address(base_address: u16, y: u8, x: u8) -> u16 {
    base_address + (y as u16 / 8 * 32) + (x as u16 / 8)
}

fn calculate_bgmap_address(lcdc: &u8, y_bgmap: u8, x_bgmap: u8) -> u16 {
    let address = if is_bit_set(lcdc, 3) {
        BGMAP_SECOND_BEGIN_ADDRESS
    } else {
        BGMAP_FIRST_BEGIN_ADDRESS
    };

    calculate_address(address, y_bgmap, x_bgmap)
}

fn calculate_window_address(mmu: &Mmu, y: u8, x: u8) -> u16 {
    let address = if is_bit_set(&mmu.io_bus.lcdc, 6) {
        BGMAP_SECOND_BEGIN_ADDRESS
    } else {
        BGMAP_FIRST_BEGIN_ADDRESS
    };

    let y_offset = y.wrapping_sub(mmu.io_bus.window_y);
    let x_offset = x.wrapping_sub(mmu.io_bus.window_x - 7);

    calculate_address(address, y_offset, x_offset)
}

fn calculate_tile_address(lcdc: &u8, tile_number: u8) -> u16 {
    //Use first tileset, tile_number interpreted as unsigned
    if is_bit_set(lcdc, 4) {
        return TILESET_FIRST_BEGIN_ADDRESS + tile_number as u16 * 16;
    }
    //Use second tileset, tile_number interpreted as signed
    TILESET_SECOND_BEGIN_ADDRESS.wrapping_add(((tile_number as i8) as u16).wrapping_mul(16))
}

fn set_palette(palette: &mut [Pixel], value: u8) {
    for i in 0..4 {
        let color_data = value >> (i * 2) & 3;

        palette[i] = match color_data {
            0 => Pixel::On,
            1 => Pixel::Light,
            2 => Pixel::Dark,
            3 => Pixel::Off,
            _ => Pixel::Off,
        }
    }
}
