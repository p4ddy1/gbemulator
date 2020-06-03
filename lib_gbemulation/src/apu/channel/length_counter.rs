pub struct LengthCounter {
    enabled: bool,
    counter: u16,
    counter_size: u16,
}

pub enum LengthCounterResult {
    None,
    DisableChannel,
}

impl LengthCounter {
    pub fn new(counter_size: u16) -> Self {
        LengthCounter {
            enabled: false,
            counter: 0,
            counter_size: counter_size,
        }
    }

    pub fn set_length(&mut self, value: u8) {
        self.counter = self.counter_size - value as u16;
    }

    pub fn set_enabled(&mut self, value: u8) {
        self.enabled = ((value & 0x40) >> 6) == 1;
    }

    pub fn step(&mut self) -> LengthCounterResult {
        if self.enabled {
            self.counter -= 1;
        }

        if self.enabled && self.counter == 0 {
            self.enabled = false;
            return LengthCounterResult::DisableChannel;
        }

        LengthCounterResult::None
    }

    pub fn trigger(&mut self) {
        if self.counter == 0 {
            self.counter = self.counter_size;
        }
    }
}
