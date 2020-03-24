#[derive(Copy, Clone)]
pub enum Mode {
    Oam = 2,
    Vram = 3,
    Hblank = 0,
    Vblank = 1,
}

pub struct Stat {
    pub mode: Mode,
    pub coincidence_flag: bool,
    pub h_blank_interrupt: bool,
    pub v_blank_interrupt: bool,
    pub oam_interrupt: bool,
    pub coincidence_interrupt: bool,
}

impl Stat {
    pub fn new(initial_value: u8) -> Stat {
        let mut stat = Stat {
            mode: Mode::Hblank,
            coincidence_flag: false,
            h_blank_interrupt: false,
            v_blank_interrupt: false,
            oam_interrupt: false,
            coincidence_interrupt: false,
        };

        stat.set_data(initial_value);
        stat
    }

    pub fn set_data(&mut self, data: u8) {
        self.h_blank_interrupt = data & 0x08 == 0x08;
        self.v_blank_interrupt = data & 0x10 == 0x10;
        self.oam_interrupt = data & 0x20 == 0x20;
        self.coincidence_interrupt = data & 0x40 == 0x40;
    }

    pub fn get_data(&self) -> u8 {
        self.mode as u8
            | (if self.coincidence_flag { 0x04 } else { 0 })
            | (if self.h_blank_interrupt { 0x08 } else { 0 })
            | (if self.v_blank_interrupt { 0x10 } else { 0 })
            | (if self.oam_interrupt { 0x20 } else { 0 })
            | (if self.coincidence_interrupt { 0x40 } else { 0 })
    }
}
