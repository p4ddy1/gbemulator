pub struct VolumeEnvelope {
    pub starting_volume: u8,
    pub add_mode: u8,
    pub period: u8,
    period_load: u8,
    pub current_volume: u8,
    period_counter: u8,
}

impl VolumeEnvelope {
    pub fn new() -> Self {
        VolumeEnvelope {
            starting_volume: 0,
            add_mode: 0,
            period: 0,
            period_load: 0,
            current_volume: 0,
            period_counter: 0,
        }
    }

    pub fn step(&mut self) {
        if self.period == 0 {
            return;
        }

        if self.period_counter < self.period - 1 {
            self.period_counter += 1;
            return;
        }

        self.period_counter = 0;

        if self.add_mode == 0 {
            self.current_volume = self.current_volume.saturating_sub(1);
        } else {
            self.current_volume += 1;
        }
    }

    pub fn process_signal(&self, signal: i16) -> i16 {
        if self.current_volume < 15 {
            return (signal / 15) * self.current_volume as i16;
        }

        signal
    }

    pub fn write(&mut self, value: u8) {
        self.starting_volume = (value & 0xF0) >> 4;
        self.add_mode = (value & 0x08) >> 3;
        self.period_load = value & 0x07;
    }

    pub fn trigger(&mut self) {
        self.current_volume = self.starting_volume;
        self.period = self.period_load;
        self.period_counter = 0;
    }
}
