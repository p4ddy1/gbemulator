use crate::gpu::screen::Screen;
use crate::gpu::{Pixel, SCREEN_MAX_PIXELS};
use crate::util::binary::is_bit_set;

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

enum Mode {
    Oam,
    Vram,
    Hblank,
    Vblank,
}

pub struct Gpu<'a> {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    clock: u16,
    mode: Mode,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub current_scanline: u8,
    pub screen: &'a mut dyn Screen,
    screen_buffer: [Pixel; 65536],
    pub lcdc: u8,
    pub v_blank: bool, //TODO: Remove!! only for testing!!!
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
            screen: screen,
            screen_buffer: [Pixel::Off; 65536],
            lcdc: 0,
            v_blank: true,
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
    }

    fn step_set_mode(&mut self) {
        match self.mode {
            Mode::Oam => {
                if self.clock >= CYCLES_OAM {
                    self.mode = Mode::Vram;
                    self.clock = 0;
                }
            }
            Mode::Vram => {
                if self.clock >= CYCLES_VRAM {
                    self.mode = Mode::Hblank;
                    self.clock = 0;
                }
            }
            Mode::Hblank => {
                if self.clock >= CYCLES_HBLANK {
                    self.render_scanline_to_screen();
                    self.current_scanline += 1;
                    self.clock = 0;
                    if self.current_scanline > SCANLINES_DISPLAY {
                        self.mode = Mode::Vblank;
                        self.v_blank = true; //TODO: Remove! only for testing
                    } else {
                        self.mode = Mode::Oam;
                    }
                }
            }
            Mode::Vblank => {
                if self.clock >= CYCLES_VBLANK {
                    self.current_scanline += 1;
                    self.clock = 0;
                    if self.current_scanline > MAX_SCANLINES {
                        self.screen.render(&self.screen_buffer);
                        self.mode = Mode::Oam;
                        self.current_scanline = 0;
                    }
                }
            }
        }
    }

    fn render_scanline_to_screen(&mut self) {
        //Bit 7 = LCD Enable. Disabled? Render nothing
        if !is_bit_set(self.lcdc, 7) {
            return;
        }

        self.render_background_line();

        if is_bit_set(self.lcdc, 1) {
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

                    //Handle sprite priority
                    if is_bit_set(sprite_options, 7) {
                        match self.screen_buffer[offset] {
                            Pixel::On => {}
                            _ => continue,
                        };
                    }

                    //X Flip
                    let pixel_index = if is_bit_set(sprite_options, 5) {
                        x
                    } else {
                        7 - x
                    };

                    //Checking each bit of the tile and setting the according pixel on the framebuffer
                    let pixel_is_active = is_bit_set(tile_data, pixel_index);
                    let pixel_color_bit = is_bit_set(tile_color_data, pixel_index);
                    let mut pixel: Option<Pixel> = None;

                    //Color 0 on sprites is transparent

                    if pixel_is_active && pixel_color_bit {
                        pixel = Some(self.bg_pal[3]);
                    }

                    if pixel_is_active && !pixel_color_bit {
                        pixel = Some(self.bg_pal[1]);
                    }

                    if !pixel_is_active && pixel_color_bit {
                        pixel = Some(self.bg_pal[2]);
                    }

                    match pixel {
                        Some(value) => self.screen_buffer[offset] = value,
                        _ => {}
                    }
                }
            }
        }
    }

    fn render_background_line(&mut self) {
        let y_bgmap = self.current_scanline.wrapping_add(self.scroll_y) as u16;

        //Each tile consists of 8 lines so we stay at one tile for 8 scanlines
        let y_tile_address = BGMAP_BEGIN_ADDRESS + (y_bgmap as u16 / 8 * 32);

        //TODO: Implement tileset changing via LCDC

        for x in 0..=255_u8 {
            let x_bgmap = x.wrapping_add(self.scroll_x);

            let tile_address = y_tile_address + (x_bgmap as u16 / 8);

            let tile = self.read_vram(tile_address);

            //TODO: Optimize this
            let mut tile_begin_address = TILESET_FIRST_BEGIN_ADDRESS;

            if !is_bit_set(self.lcdc, 4) {
                tile_begin_address = TILESET_SECOND_BEGIN_ADDRESS;
            }

            //Get the begin address for the tile on the background. Each tile consists of 16 bytes
            if tile_begin_address == TILESET_FIRST_BEGIN_ADDRESS {
                tile_begin_address += tile as u16 * 16;
            } else {
                if tile < 127 {
                    tile_begin_address += tile as u16 * 16;
                } else {
                    tile_begin_address -= (256 - tile as u16) * 16;
                }
            }

            //Each tile consists of one byte at the y axes
            let tile_data_address = tile_begin_address + (y_bgmap % 8 * 2);
            //The color data sits one byte after the pixel data
            let tile_color_data_address = tile_begin_address + (y_bgmap % 8 * 2) + 1;

            let tile_data = self.read_vram(tile_data_address);
            let tile_color_data = self.read_vram(tile_color_data_address);

            //Checking each bit of the tile and setting the according pixel on the framebuffer
            let pixel_is_active = is_bit_set(tile_data, 7 - x_bgmap % 8);
            let pixel_color_bit = is_bit_set(tile_color_data, 7 - x_bgmap % 8);
            let mut pixel = Pixel::Off;

            if pixel_is_active && pixel_color_bit {
                pixel = self.bg_pal[3];
            }

            if pixel_is_active && !pixel_color_bit {
                pixel = self.bg_pal[1];
            }

            if !pixel_is_active && !pixel_color_bit {
                pixel = self.bg_pal[0];
            }

            if !pixel_is_active && pixel_color_bit {
                pixel = self.bg_pal[2];
            }

            self.screen_buffer[self.current_scanline as usize + 256 * x as usize] = pixel
        }
    }
}
