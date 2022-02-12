use crate::gpu::lcdc::Lcdc;
use crate::gpu::stat::{Mode, Stat};
use crate::gpu::SCREEN_WIDTH;
use crate::gpu::{Screen, BUFFER_SIZE};
use crate::memory::interrupts::Interrupt;
use crate::memory::mmu::{OAM_ADDRESS, VRAM_ADDRESS};
use crate::util::binary::is_bit_set;
use std::sync::Arc;

const V_RAM_SIZE: usize = 8192;
const OAM_SIZE: usize = 160;

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
enum PriorityFlag {
    None,
    Color0,
}

pub struct Gpu {
    pub screen: Arc<dyn Screen>,
    pub lcdc: Lcdc,
    pub stat: Stat,
    pub current_scanline: u8,
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub window_x: u8,
    pub window_y: u8,
    pub interrupts_fired: u8,
    clock: u16,
    screen_buffer: [u8; BUFFER_SIZE],
    bg_priority_map: [PriorityFlag; 65792],
    v_ram: [u8; V_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    lyc: u8,
    bg_pal: [u8; 4],
    sprite_palette0: [u8; 4],
    sprite_palette1: [u8; 4],
    raw_palette_data: [u8; 3],
    color_map: [[u8; 3]; 4],
    lcd_enabled: bool,
    first_frame_after_activation: bool,
}

impl Gpu {
    pub fn new(screen: Arc<dyn Screen>) -> Gpu {
        Gpu {
            screen: screen,
            current_scanline: 0,
            lcdc: Lcdc::new(0x91),
            stat: Stat::new(0x84),
            scroll_y: 0,
            scroll_x: 0,
            window_y: 0,
            window_x: 7,
            lyc: 0,
            interrupts_fired: 0,
            clock: 0,
            screen_buffer: [0; BUFFER_SIZE],
            bg_priority_map: [PriorityFlag::None; 65792],
            v_ram: [0; V_RAM_SIZE],
            oam: [0; OAM_SIZE],
            bg_pal: [0, 1, 2, 3],
            sprite_palette0: [0, 1, 2, 3],
            sprite_palette1: [0, 1, 2, 3],
            raw_palette_data: [0xFC, 0xFF, 0xFF],
            color_map: [[0; 3], [0; 3], [0; 3], [0; 3]],
            lcd_enabled: true,
            first_frame_after_activation: true,
        }
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        self.v_ram[(address - VRAM_ADDRESS) as usize]
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {
        self.v_ram[(address - VRAM_ADDRESS) as usize] = value;
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        self.oam[(address - OAM_ADDRESS) as usize] = value;
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        self.oam[(address - OAM_ADDRESS) as usize]
    }

    pub fn set_bg_pal(&mut self, value: u8) {
        self.raw_palette_data[0] = value;
        set_palette(&mut self.bg_pal, value);
    }

    pub fn get_bg_pal(&self) -> u8 {
        self.raw_palette_data[0]
    }

    pub fn set_sprite_palette0(&mut self, value: u8) {
        self.raw_palette_data[1] = value;
        set_palette(&mut self.sprite_palette0, value);
    }

    pub fn get_sprite_palette0(&self) -> u8 {
        self.raw_palette_data[1]
    }

    pub fn set_sprite_palette1(&mut self, value: u8) {
        self.raw_palette_data[2] = value;
        set_palette(&mut self.sprite_palette1, value);
    }

    pub fn get_sprite_palette1(&self) -> u8 {
        self.raw_palette_data[2]
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.lyc = value;
        self.compare_lyc();
    }

    pub fn set_current_scanline(&mut self, value: u8) {
        self.current_scanline = value;
        self.compare_lyc();
    }

    pub fn get_lyc(&self) -> u8 {
        self.lyc
    }

    pub fn set_lcdc(&mut self, value: u8) {
        self.lcdc.set_data(value);

        //If LCD is renabled after it was disabled set flag so the first frame wont be rendered
        if !self.lcd_enabled && self.lcdc.display_enabled {
            self.lcd_enabled = true;
            self.first_frame_after_activation = true;
        }

        //If LCD is disabled reset the gpu state
        if self.lcd_enabled && !self.lcdc.display_enabled {
            self.clear_screen();
            self.set_current_scanline(0);
            self.stat.mode = Mode::Hblank;
            self.clock = 0;
            self.lcd_enabled = false;
        }
    }

    pub fn get_lcdc(&self) -> u8 {
        self.lcdc.get_data()
    }

    pub fn set_stat(&mut self, value: u8) {
        self.stat.set_data(value);
    }

    pub fn get_stat(&self) -> u8 {
        self.stat.get_data()
    }

    pub fn step(&mut self, clock_cycles: u8) {
        if !self.lcd_enabled {
            return;
        }

        self.clock += clock_cycles as u16;
        self.step_set_mode();
    }

    fn fire_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupts_fired |= interrupt as u8;
    }

    fn step_set_mode(&mut self) {
        match self.stat.mode {
            Mode::Oam => {
                if self.clock >= CYCLES_OAM {
                    self.set_mode(Mode::Vram);
                    self.clock = self.clock % CYCLES_OAM;
                }
            }
            Mode::Vram => {
                if self.clock >= CYCLES_VRAM {
                    self.render_scanline_to_screen();
                    self.set_mode(Mode::Hblank);
                    self.clock = self.clock % CYCLES_VRAM;
                }
            }
            Mode::Hblank => {
                if self.clock >= CYCLES_HBLANK {
                    self.clock = self.clock % CYCLES_HBLANK;

                    if self.current_scanline >= SCANLINES_DISPLAY {
                        self.set_mode(Mode::Vblank);
                        self.render_screen();
                        self.fire_interrupt(Interrupt::Vblank);
                        self.clear_screen();
                    } else {
                        self.set_current_scanline(self.current_scanline + 1);
                        self.set_mode(Mode::Oam);
                    }
                }
            }
            Mode::Vblank => {
                if self.clock >= CYCLES_VBLANK {
                    self.set_current_scanline(self.current_scanline + 1);
                    self.clock = self.clock % CYCLES_VBLANK;
                    if self.current_scanline > MAX_SCANLINES {
                        self.set_mode(Mode::Oam);
                        self.set_current_scanline(0);
                    }
                }
            }
        }
    }

    fn compare_lyc(&mut self) {
        self.stat.coincidence_flag = false;
        if self.lyc == self.current_scanline {
            self.stat.coincidence_flag = true;
            if self.stat.coincidence_interrupt {
                self.fire_interrupt(Interrupt::LcdStat);
            }
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.stat.mode = mode;
        self.fire_stat_interrupt();
    }

    fn fire_stat_interrupt(&mut self) {
        match self.stat.mode {
            Mode::Oam => {
                if self.stat.oam_interrupt {
                    self.fire_interrupt(Interrupt::LcdStat);
                }
            }
            Mode::Hblank => {
                if self.stat.h_blank_interrupt {
                    self.fire_interrupt(Interrupt::LcdStat);
                }
            }
            Mode::Vblank => {
                if self.stat.v_blank_interrupt {
                    self.fire_interrupt(Interrupt::LcdStat);
                }
            }
            _ => {}
        }
    }

    fn clear_screen(&mut self) {
        for i in 0..256 * 256 + 256 {
            self.screen_buffer[i] = 0;
            self.bg_priority_map[i] = PriorityFlag::None;
        }
    }

    fn render_screen(&mut self) {
        //First frame after the screen has been enabled will not be displayed
        if self.first_frame_after_activation {
            self.first_frame_after_activation = false;
            return;
        }

        self.color_map = self.screen.get_palette();
        self.screen.draw(&self.screen_buffer);
    }

    fn render_scanline_to_screen(&mut self) {
        if self.lcdc.background_display {
            self.render_background_line();
        }

        if self.lcdc.sprite_display {
            self.render_sprite_line();
        }
    }

    fn render_sprite_line(&mut self) {
        let current_line = self.current_scanline as i16;

        let mut sprite_height = 8;

        if self.lcdc.sprite_size_big {
            sprite_height = 16;
        }

        for sprite_count in (0..40).rev() {
            //Each sprite is consists for 4 bytes
            //0 = Y, 1 = X, 2 = Tile, 3 = Options
            let sprite_begin_address = OAM_ADDRESS + sprite_count * 4;

            let sprite_y = self.read_oam(sprite_begin_address) as i16 - 16;
            let sprite_x = self.read_oam(sprite_begin_address + 1) as i16 - 8;

            //Check if tile is at current scanline
            if current_line >= sprite_y && current_line < sprite_y + sprite_height {
                let sprite_tile = self.read_oam(sprite_begin_address + 2);
                let sprite_options = self.read_oam(sprite_begin_address + 3);

                let tile_begin_address = TILESET_FIRST_BEGIN_ADDRESS + (sprite_tile as u16 * 16);

                let line_offset = flip_y(&sprite_options, current_line, sprite_height, sprite_y);

                //Each tile consists of one byte at the y axes
                let tile_data_address = tile_begin_address + (line_offset * 2) as u16;
                //The color data sits one byte after the pixel data
                let tile_color_data_address = tile_begin_address + (line_offset * 2) as u16 + 1;

                let tile_data = self.read_vram(tile_data_address);
                let tile_color_data = self.read_vram(tile_color_data_address);

                for x in 0..8 {
                    let x_offset = sprite_x + x as i16;
                    if x_offset < 0 || x_offset >= 160 {
                        continue;
                    }

                    let pixel_index = flip_x(&sprite_options, x);

                    self.draw_sprite_pixel(
                        tile_data,
                        tile_color_data,
                        current_line as u8,
                        x_offset as u8,
                        pixel_index,
                        &sprite_options,
                    );
                }
            }
        }
    }

    fn render_background_line(&mut self) {
        let y_bgmap = self.current_scanline.wrapping_add(self.scroll_y);

        let line_is_window = self.lcdc.window_enabled && self.current_scanline >= self.window_y;

        for x in 0..160_u8 {
            let x_bgmap = x.wrapping_add(self.scroll_x);

            let column_is_window = self.lcdc.window_enabled && x >= self.window_x.wrapping_sub(7);

            let tile_address = if line_is_window && column_is_window {
                self.calculate_window_address(self.current_scanline, x)
            } else {
                self.calculate_bgmap_address(y_bgmap, x_bgmap)
            };

            let tile = self.read_vram(tile_address);

            let tile_begin_address = self.calculate_tile_address(tile);

            //Each tile consists of one byte at the y axes
            let y_tile_address_offset = if line_is_window && column_is_window {
                (self.current_scanline - self.window_y) % 8 * 2
            } else {
                y_bgmap % 8 * 2
            } as u16;

            let tile_data_address = tile_begin_address + y_tile_address_offset;
            //The color data sits one byte after the pixel data
            let tile_color_data_address = tile_data_address + 1;

            let tile_data = self.read_vram(tile_data_address);
            let tile_color_data = self.read_vram(tile_color_data_address);

            let pixel_index = if column_is_window && line_is_window {
                self.window_x.wrapping_sub(x) % 8
            } else {
                7 - (x_bgmap % 8)
            };

            self.draw_background_pixel(
                tile_data,
                tile_color_data,
                self.current_scanline,
                x,
                pixel_index,
            );
        }
    }

    fn calculate_window_address(&self, y: u8, x: u8) -> u16 {
        let address = if self.lcdc.window_tilemap {
            BGMAP_SECOND_BEGIN_ADDRESS
        } else {
            BGMAP_FIRST_BEGIN_ADDRESS
        };

        let y_offset = y.wrapping_sub(self.window_y);
        let x_offset = x.wrapping_sub(self.window_x.wrapping_sub(7));

        calculate_address(address, y_offset, x_offset)
    }

    fn calculate_bgmap_address(&self, y_bgmap: u8, x_bgmap: u8) -> u16 {
        let address = if self.lcdc.background_tilemap {
            BGMAP_SECOND_BEGIN_ADDRESS
        } else {
            BGMAP_FIRST_BEGIN_ADDRESS
        };

        calculate_address(address, y_bgmap, x_bgmap)
    }

    fn calculate_tile_address(&self, tile_number: u8) -> u16 {
        //Use first tileset, tile_number interpreted as unsigned
        if self.lcdc.background_tiledata {
            return TILESET_FIRST_BEGIN_ADDRESS + tile_number as u16 * 16;
        }
        //Use second tileset, tile_number interpreted as signed
        TILESET_SECOND_BEGIN_ADDRESS.wrapping_add(((tile_number as i8) as u16).wrapping_mul(16))
    }

    fn draw_sprite_pixel(
        &mut self,
        tile_data: u8,
        tile_color_data: u8,
        y: u8,
        x: u8,
        pixel_index: u8,
        sprite_options: &u8,
    ) {
        let sprite_palette = if is_bit_set(&sprite_options, 4) {
            &self.sprite_palette1
        } else {
            &self.sprite_palette0
        };

        let color_index = get_color_index(tile_data, tile_color_data, pixel_index);
        //Color 0 is transparent for sprites
        if color_index == 0 {
            return;
        }

        let pixel = sprite_palette[color_index as usize];

        let offset = y as usize + 256 * x as usize;

        if self.background_has_priority_over_pixel(sprite_options, offset) {
            return;
        }

        self.draw_pixel_to_buffer(x as usize, y as usize, self.color_map[pixel as usize]);
    }

    fn background_has_priority_over_pixel(&self, sprite_options: &u8, offset: usize) -> bool {
        if !is_bit_set(&sprite_options, 7) {
            return false;
        }

        //Sprite will only be behind colors 1-3
        match self.bg_priority_map[offset] {
            PriorityFlag::Color0 => false,
            PriorityFlag::None => true,
        }
    }

    fn draw_background_pixel(
        &mut self,
        tile_data: u8,
        tile_color_data: u8,
        y: u8,
        x: u8,
        pixel_index: u8,
    ) {
        let color_index = get_color_index(tile_data, tile_color_data, pixel_index);
        let pixel = self.bg_pal[color_index as usize];
        let offset = y as usize + 256 * x as usize;

        //Set priority information for sprites. Sprite will never be behind color 0
        if color_index == 0 {
            self.bg_priority_map[offset] = PriorityFlag::Color0
        }

        self.draw_pixel_to_buffer(x as usize, y as usize, self.color_map[pixel as usize]);
    }

    fn draw_pixel_to_buffer(&mut self, x: usize, y: usize, rgb: [u8; 3]) {
        let offset = (x * 3) + (y * SCREEN_WIDTH * 3);

        self.screen_buffer[offset] = rgb[0];
        self.screen_buffer[offset + 1] = rgb[1];
        self.screen_buffer[offset + 2] = rgb[2];
    }
}

fn get_color_index(tile_data: u8, tile_color_data: u8, pixel_index: u8) -> u8 {
    (if tile_data & (1 << pixel_index) > 0 {
        1
    } else {
        0
    }) | (if tile_color_data & (1 << pixel_index) > 0 {
        1
    } else {
        0
    }) << 1
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

fn set_palette(palette: &mut [u8], value: u8) {
    for i in 0..4 {
        let color_data = value >> (i * 2) & 3;

        palette[i] = match color_data {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            _ => 3,
        }
    }
}
