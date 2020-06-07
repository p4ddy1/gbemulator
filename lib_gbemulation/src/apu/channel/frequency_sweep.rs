pub struct FrequencySweep {
    pub frequency: u16,
    period: i8,
    period_load: u8,
    period_counter: u8,
    negate: u8,
    shift: u8,
    enabled: bool,
}

pub enum FrequencySweepResult {
    None,
    Overflowed,
    Sweeped(u16),
}

impl FrequencySweep {
    pub fn new() -> Self {
        FrequencySweep {
            frequency: 0,
            period: 0,
            period_load: 0,
            period_counter: 0,
            negate: 0,
            shift: 0,
            enabled: false,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.period_load = (value & 0x70) >> 4;
        self.negate = (value & 0x08) >> 3;
        self.shift = value & 0x07;
    }

    pub fn step(&mut self) -> FrequencySweepResult {
        if !self.enabled {
            return FrequencySweepResult::None;
        }

        self.period -= 1;

        if self.period <= 0 {
            self.period = self.period_load as i8;
            if self.period == 0 {
                return FrequencySweepResult::None;
            }

            if !self.calculate_frequency() {
                return FrequencySweepResult::Overflowed;
            }

            return FrequencySweepResult::Sweeped(self.frequency);
        }

        FrequencySweepResult::None
    }

    pub fn calculate_frequency(&mut self) -> bool {
        if self.negate == 0 {
            let frequency = self.frequency + (self.frequency >> self.shift);
            if frequency > 2047 {
                return false;
            }
            self.frequency = frequency;
        } else {
            let frequency = self.frequency as i16 - (self.frequency >> self.shift) as i16;
            if frequency < 0 {
                return false;
            }
            self.frequency = frequency as u16;
        }

        true
    }

    pub fn trigger(&mut self, frequency: u16) -> FrequencySweepResult {
        self.frequency = frequency;
        self.period_counter = 0;

        if self.shift > 0 && self.period_load > 0 {
            self.period = self.period_load as i8;
            self.enabled = true;
        } else {
            self.enabled = false;
        }

        FrequencySweepResult::None
    }
}
