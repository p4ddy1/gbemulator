use crate::gpu::screen::Screen;
use crate::gpu::{Pixel, SCREEN_MAX_PIXELS};
use crate::util::binary::{is_bit_set, reset_bit_in_byte, set_bit_in_byte};
use sdl2::render::BlendMode::Mod;

const VRAM_ADDRESS: u16 = 0x8000;
const VRAM_SIZE: usize = 8192;
const OAM_SIZE: usize = 160;
const OAM_ADDRESS: u16 = 0xFE00;

const TILESET_FIRST_BEGIN_ADDRESS: u16 = 0x8000;
const TILESET_SECOND_BEGIN_ADDRESS: u16 = 0x9000;
const TILESET_END_ADDRESS: u16 = 0x97EF;
const BGMAP_BEGIN_ADDRESS: u16 = 0x9800;
const BGMAP_END_ADDRESS: u16 = 0x9BFE;

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
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    clock: u16,
    mode: Mode,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub current_scanline: u8,
    pub lyc: u8,
    pub screen: &'a mut dyn Screen,
    screen_buffer: [Pixel; 68000], //TODO: Find better length
    pub lcdc: u8,
    pub v_blank: bool,  //TODO: Remove!! only for testing!!!
    pub lcd_stat: bool, //TODO: Remove!! only for testing!!!
    pub stat: u8,
    bg_pal: [Pixel; 4],
}

impl<'a> Gpu<'a> {
    pub fn new(screen: &'a mut dyn Screen) -> Gpu<'a> {
        Gpu {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            clock: 0,
            mode: Mode::Hblank,
            scroll_x: 0,
            scroll_y: 0,
            current_scanline: 0,
            lyc: 0,
            screen: screen,
            screen_buffer: [Pixel::Off; 68000],
            lcdc: 0,
            v_blank: true,
            lcd_stat: false,
            stat: 0x84,
            bg_pal: [Pixel::On, Pixel::Light, Pixel::Dark, Pixel::Off],
        }
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        self.vram[(addr - VRAM_ADDRESS) as usize] = value;
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[(addr - VRAM_ADDRESS) as usize]
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        self.oam[(addr - OAM_ADDRESS) as usize] = value;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam[(addr - OAM_ADDRESS) as usize]
    }

    pub fn write_lcdc(&mut self, value: u8) {
        self.lcdc = value;
    }

    pub fn write_lyc(&mut self, value: u8) {
        self.lyc = value;
        self.compare_lyc();
    }

    fn compare_lyc(&mut self) {
        self.stat = reset_bit_in_byte(self.stat, 2);
        if self.lyc == self.current_scanline {
            self.stat = set_bit_in_byte(self.stat, 2);
            if is_bit_set(&self.stat, 6) {
                self.lcd_stat = true;
            } //TODO: Is this correct?
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.stat &= 0xC0;
        self.stat |= mode as u8 & 0x3F;
        self.mode = mode;
        self.fire_stat_interrupt();
    }

    fn fire_stat_interrupt(&mut self) {
        match self.mode {
            Mode::Oam => {
                if is_bit_set(&self.stat, 5) {
                    self.lcd_stat = true;
                }
            }
            Mode::Vram => {
                if is_bit_set(&self.stat, 3) {
                    self.lcd_stat = true;
                }
            }
            Mode::Vblank => {
                if is_bit_set(&self.stat, 4) {
                    self.lcd_stat = true;
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

    fn clear_screen_buffer(&mut self) {
        for y in 0..256 {
            for x in 0..256 {
                self.screen_buffer[y + 256 * x] = Pixel::Off;
            }
        }
    }

    pub fn step(&mut self, cycles: u8) {
        self.clock += cycles as u16;
        self.step_set_mode();
        self.compare_lyc();
    }

    fn step_set_mode(&mut self) {
        match self.mode {
            Mode::Oam => {
                if self.clock >= CYCLES_OAM {
                    self.set_mode(Mode::Vram);
                    self.clock = 0;
                }
            }
            Mode::Vram => {
                if self.clock >= CYCLES_VRAM {
                    self.render_scanline_to_screen();
                    self.set_mode(Mode::Hblank);
                    self.clock = 0;
                }
            }
            Mode::Hblank => {
                if self.clock >= CYCLES_HBLANK {
                    self.clock = 0;
                    self.current_scanline += 1;
                    if self.current_scanline > SCANLINES_DISPLAY {
                        self.set_mode(Mode::Vblank);
                        self.screen.render(&self.screen_buffer);
                        self.v_blank = true; //TODO: Remove! only for testing
                    } else {
                        self.set_mode(Mode::Oam);
                    }
                }
            }
            Mode::Vblank => {
                if self.clock >= CYCLES_VBLANK {
                    self.current_scanline += 1;
                    self.clock = 0;
                    if self.current_scanline > MAX_SCANLINES {
                        self.set_mode(Mode::Oam);
                        self.current_scanline = 0;
                    }
                }
            }
        }
    }

    fn render_scanline_to_screen(&mut self) {
        //Bit 7 = LCD Enable. Disabled? Render nothing
        if !is_bit_set(&self.lcdc, 7) {
            return;
        }

        self.render_background_line();

        if is_bit_set(&self.lcdc, 1) {
            self.render_sprite_line();
        }
    }

    fn render_sprite_line(&mut self) {
        //TODO: Scroll_X is missing, y flip, palette
        let current_line = self.current_scanline.wrapping_add(self.scroll_y);

        for sprite_count in 0..40 {
            //Each sprite is consists for 4 bytes
            //0 = Y, 1 = X, 2 = Tile, 3 = Options
            let sprite_begin_address = OAM_ADDRESS + sprite_count * 4;

            //Offset y = 16
            let sprite_y = self.read_oam(sprite_begin_address).wrapping_sub(16);
            //Check if tile is at current scanline
            if current_line >= sprite_y && current_line < sprite_y + 8 {
                //Offset x = 8
                let sprite_x = self.read_oam(sprite_begin_address + 1).wrapping_sub(8);
                let sprite_tile = self.read_oam(sprite_begin_address + 2);
                let sprite_options = self.read_oam(sprite_begin_address + 3);

                let tile_begin_address = TILESET_FIRST_BEGIN_ADDRESS + (sprite_tile as u16 * 16);

                //Get the offset fot addressing the pixel data in vram
                let line_offset = current_line - sprite_y;

                //Each tile consists of one byte at the y axes
                let tile_data_address = tile_begin_address + (line_offset * 2) as u16;
                //The color data sits one byte after the pixel data
                let tile_color_data_address = tile_begin_address + (line_offset * 2) as u16 + 1;

                let tile_data = self.read_vram(tile_data_address);
                let tile_color_data = self.read_vram(tile_color_data_address);

                for x in 0..8 {
                    let offset =
                        self.current_scanline as usize + 256 * (sprite_x as usize + x as usize);

                    if (self.background_has_priority_over_pixel(&sprite_options, offset)) {
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

    fn calculate_tile_address(&self, tile_number: u8) -> u16 {
        //Use first tileset, tile_number interpreted as unsigned
        if is_bit_set(&self.lcdc, 4) {
            return TILESET_FIRST_BEGIN_ADDRESS + tile_number as u16 * 16;
        }
        //Use second tileset, tile_number interpreted as signed
        TILESET_SECOND_BEGIN_ADDRESS.wrapping_add(((tile_number as i8) as u16).wrapping_mul(16))
    }

    fn render_background_line(&mut self) {
        let y_bgmap = self.current_scanline.wrapping_add(self.scroll_y) as u16;
        let y_tile_address_offset = y_bgmap % 8 * 2;

        //Each tile consists of 8 lines so we stay at one tile for 8 scanlines
        let y_tile_address = BGMAP_BEGIN_ADDRESS + (y_bgmap as u16 / 8 * 32);

        //TODO: Implement tileset changing via LCDC
        for x in 0..=160_u8 {
            let x_bgmap = x.wrapping_add(self.scroll_x);

            let tile_address = y_tile_address + (x_bgmap as u16 / 8);

            let tile = self.read_vram(tile_address);

            let tile_begin_address = self.calculate_tile_address(tile);

            //Each tile consists of one byte at the y axes
            let tile_data_address = tile_begin_address + y_tile_address_offset;
            //The color data sits one byte after the pixel data
            let tile_color_data_address = tile_data_address + 1;

            let tile_data = self.read_vram(tile_data_address);
            let tile_color_data = self.read_vram(tile_color_data_address);

            let pixel_index = 7 - x_bgmap % 8;

            let pixel =
                get_pixel(&self.bg_pal, tile_data, tile_color_data, pixel_index, false).unwrap();

            self.screen_buffer[self.current_scanline as usize + 256 * x as usize] = pixel;
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
