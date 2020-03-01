use crate::gpu::screen::Screen;
use crate::gpu::Pixel;
use crate::memory::interrupts::Interrupt;
use crate::memory::mmu::Mmu;

use crate::util::binary::{is_bit_set, reset_bit_in_byte, set_bit_in_byte};

const OAM_ADDRESS: u16 = 0xFE00;
const TILESET_FIRST_BEGIN_ADDRESS: u16 = 0x8000;
const TILESET_SECOND_BEGIN_ADDRESS: u16 = 0x9000;
const BGMAP_BEGIN_ADDRESS: u16 = 0x9800;

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
    screen_buffer: [Pixel; 68000], //TODO: Find better length
    bg_pal: [Pixel; 4],
}

impl<'a> Gpu<'a> {
    pub fn new(screen: &'a mut dyn Screen) -> Gpu<'a> {
        Gpu {
            clock: 0,
            mode: Mode::Hblank,
            screen: screen,
            screen_buffer: [Pixel::Off; 68000],
            bg_pal: [Pixel::On, Pixel::Light, Pixel::Dark, Pixel::Off],
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

    pub fn set_bgpal(&mut self, value: u8) {
        for i in 0..4 {
            let color_data = value >> (i * 2) & 3;

            self.bg_pal[i] = match color_data {
                0 => Pixel::On,
                1 => Pixel::Light,
                2 => Pixel::Dark,
                3 => Pixel::Off,
                _ => Pixel::Off,
            }
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, cycles: u8) {
        self.clock += cycles as u16;
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
                    if mmu.io_bus.current_scanline >= SCANLINES_DISPLAY {
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

    fn render_scanline_to_screen(&mut self, mmu: &mut Mmu) {
        //Bit 7 = LCD Enable. Disabled? Render nothing
        if !is_bit_set(&mmu.io_bus.lcdc, 7) {
            return;
        }

        self.render_background_line(mmu);

        if is_bit_set(&mmu.io_bus.lcdc, 1) {
            self.render_sprite_line(mmu);
        }
    }

    fn render_sprite_line(&mut self, mmu: &mut Mmu) {
        //TODO: Scroll_X is missing, y flip, palette
        let current_line = mmu
            .io_bus
            .current_scanline
            .wrapping_add(mmu.io_bus.scroll_y);

        for sprite_count in 0..40 {
            //Each sprite is consists for 4 bytes
            //0 = Y, 1 = X, 2 = Tile, 3 = Options
            let sprite_begin_address = OAM_ADDRESS + sprite_count * 4;

            //Offset y = 16
            let sprite_y = mmu.read_oam(sprite_begin_address).wrapping_sub(16);
            //Check if tile is at current scanline
            if current_line >= sprite_y && current_line < sprite_y + 8 {
                //Offset x = 8
                let sprite_x = mmu.read_oam(sprite_begin_address + 1).wrapping_sub(8);
                let sprite_tile = mmu.read_oam(sprite_begin_address + 2);
                let sprite_options = mmu.read_oam(sprite_begin_address + 3);

                let tile_begin_address = TILESET_FIRST_BEGIN_ADDRESS + (sprite_tile as u16 * 16);

                //Get the offset fot addressing the pixel data in vram
                let line_offset = current_line - sprite_y;

                //Each tile consists of one byte at the y axes
                let tile_data_address = tile_begin_address + (line_offset * 2) as u16;
                //The color data sits one byte after the pixel data
                let tile_color_data_address = tile_begin_address + (line_offset * 2) as u16 + 1;

                let tile_data = mmu.read_vram(tile_data_address);
                let tile_color_data = mmu.read_vram(tile_color_data_address);

                for x in 0..8 {
                    let offset = mmu.io_bus.current_scanline as usize
                        + 256 * (sprite_x as usize + x as usize);

                    if self.background_has_priority_over_pixel(&sprite_options, offset) {
                        continue;
                    }

                    let pixel_index = flip_x(&sprite_options, x);

                    let pixel =
                        get_pixel(&self.bg_pal, tile_data, tile_color_data, pixel_index, true);

                    match pixel {
                        Some(value) => self.screen_buffer[offset] = value,
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

    fn calculate_tile_address(&self, lcdc: &u8, tile_number: u8) -> u16 {
        //Use first tileset, tile_number interpreted as unsigned
        if is_bit_set(lcdc, 4) {
            return TILESET_FIRST_BEGIN_ADDRESS + tile_number as u16 * 16;
        }
        //Use second tileset, tile_number interpreted as signed
        TILESET_SECOND_BEGIN_ADDRESS.wrapping_add(((tile_number as i8) as u16).wrapping_mul(16))
    }

    fn render_background_line(&mut self, mmu: &mut Mmu) {
        let y_bgmap = mmu
            .io_bus
            .current_scanline
            .wrapping_add(mmu.io_bus.scroll_y) as u16;
        let y_tile_address_offset = y_bgmap % 8 * 2;

        //Each tile consists of 8 lines so we stay at one tile for 8 scanlines
        let y_tile_address = BGMAP_BEGIN_ADDRESS + (y_bgmap as u16 / 8 * 32);

        //TODO: Implement tileset changing via LCDC
        for x in 0..=160_u8 {
            let x_bgmap = x.wrapping_add(mmu.io_bus.scroll_x);

            let tile_address = y_tile_address + (x_bgmap as u16 / 8);

            let tile = mmu.read_vram(tile_address);

            let tile_begin_address = self.calculate_tile_address(&mmu.io_bus.lcdc, tile);

            //Each tile consists of one byte at the y axes
            let tile_data_address = tile_begin_address + y_tile_address_offset;
            //The color data sits one byte after the pixel data
            let tile_color_data_address = tile_data_address + 1;

            let tile_data = mmu.read_vram(tile_data_address);
            let tile_color_data = mmu.read_vram(tile_color_data_address);

            let pixel_index = 7 - x_bgmap % 8;

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
