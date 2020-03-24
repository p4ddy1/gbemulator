pub struct Lcdc {
    pub background_display: bool,
    pub sprite_display: bool,
    pub sprite_size_big: bool,
    pub background_tilemap: bool,
    pub background_tiledata: bool,
    pub window_enabled: bool,
    pub window_tilemap: bool,
    pub display_enabled: bool,
}

impl Lcdc {
    pub fn new(initial_value: u8) -> Lcdc {
        let mut lcdc = Lcdc {
            background_display: false,
            sprite_display: false,
            sprite_size_big: false,
            background_tilemap: false,
            background_tiledata: false,
            window_enabled: false,
            window_tilemap: false,
            display_enabled: false,
        };

        lcdc.set_data(initial_value);
        lcdc
    }

    pub fn set_data(&mut self, data: u8) {
        self.background_display = data & 0x01 == 0x01;
        self.sprite_display = data & 0x02 == 0x02;
        self.sprite_size_big = data & 0x04 == 0x04;
        self.background_tilemap = data & 0x08 == 0x08;
        self.background_tiledata = data & 0x10 == 0x10;
        self.window_enabled = data & 0x20 == 0x20;
        self.window_tilemap = data & 0x40 == 0x40;
        self.display_enabled = data & 0x80 == 0x80;
    }

    pub fn get_data(&self) -> u8 {
        (if self.background_display { 0x01 } else { 0 })
            | (if self.sprite_display { 0x02 } else { 0 })
            | (if self.sprite_size_big { 0x04 } else { 0 })
            | (if self.background_tilemap { 0x08 } else { 0 })
            | (if self.background_tiledata { 0x10 } else { 0 })
            | (if self.window_enabled { 0x20 } else { 0 })
            | (if self.window_tilemap { 0x40 } else { 0 })
            | (if self.display_enabled { 0x80 } else { 0 })
    }
}
